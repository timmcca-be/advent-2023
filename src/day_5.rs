use lazy_static::lazy_static;
use regex::Regex;
use std::vec::Vec;

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

struct RangeMapping {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

impl RangeMapping {
    /// Returns a tuple of:
    /// - A vector of ranges that were updated by this mapping
    /// - A vector of ranges that were not updated by this mapping
    fn apply_to_ranges(&self, ranges: &Vec<Range>) -> (Vec<Range>, Vec<Range>) {
        let mut updated_ranges: Vec<Range> = Vec::new();
        let mut non_updated_ranges: Vec<Range> = Vec::new();

        let mapping_source_range = Range {
            start: self.source_start,
            length: self.range_length,
        };

        for range in ranges {
            let (bottom, center, top) = range.split(mapping_source_range);
            match bottom {
                Some(range) => non_updated_ranges.push(range),
                None => {}
            }
            match top {
                Some(range) => non_updated_ranges.push(range),
                None => {}
            }
            match center {
                Some(range) => updated_ranges.push(Range {
                    start: range.start - self.source_start + self.destination_start,
                    length: range.length,
                }),
                None => {}
            }
        }

        return (updated_ranges, non_updated_ranges);
    }
}

struct RangeMappingBatchesIterator<'a> {
    lines: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> Iterator for RangeMappingBatchesIterator<'a> {
    type Item = Vec<RangeMapping>;

    fn next(&mut self) -> Option<Vec<RangeMapping>> {
        // skip section header
        if self.lines.next() == None {
            return None;
        }

        let mut range_mappings: Vec<RangeMapping> = Vec::new();
        while let Some(line) = self.lines.next() {
            if line == "" {
                break;
            }

            let numbers: Vec<i64> = NUMBER_PATTERN
                .find_iter(&line)
                .map(|m| m.as_str().parse::<i64>().unwrap())
                .collect();
            if numbers.len() != 3 {
                panic!("invalid line");
            }

            range_mappings.push(RangeMapping {
                destination_start: numbers[0],
                source_start: numbers[1],
                range_length: numbers[2],
            })
        }

        return Some(range_mappings);
    }
}

pub fn step_1<'a>(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();

    let mut values: Vec<i64> = NUMBER_PATTERN
        .find_iter(&lines_iterator.next().unwrap())
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();

    // skip empty line
    lines_iterator.next();

    let range_mapping_batches_iterator = RangeMappingBatchesIterator {
        lines: Box::new(lines_iterator),
    };
    for range_mapping_batch in range_mapping_batches_iterator {
        for value in values.iter_mut() {
            for range_mapping in &range_mapping_batch {
                if *value >= range_mapping.source_start
                    && *value < range_mapping.source_start + range_mapping.range_length
                {
                    *value = *value - range_mapping.source_start + range_mapping.destination_start;
                    break;
                }
            }
        }
    }

    println!("lowest location: {}", values.iter().min().unwrap());
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn split_on_number(&self, split_point: i64) -> (Option<Range>, Option<Range>) {
        if split_point <= self.start {
            return (None, Some(*self));
        }
        if split_point >= self.start + self.length {
            return (Some(*self), None);
        }

        return (
            Some(Range {
                start: self.start,
                length: split_point - self.start,
            }),
            Some(Range {
                start: split_point,
                length: self.start + self.length - split_point,
            }),
        );
    }

    /// Returns a tuple containing:
    /// - The subrange of self that is below other
    /// - The subrange of self that intersects other
    /// - The subrange of self that is above other
    fn split(&self, other: Range) -> (Option<Range>, Option<Range>, Option<Range>) {
        let (bottom, non_bottom) = self.split_on_number(other.start);
        let (center, top) = match non_bottom {
            Some(range) => range.split_on_number(other.start + other.length),
            None => (None, None),
        };

        return (bottom, center, top);
    }
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let seed_numbers: Vec<i64> = NUMBER_PATTERN
        .find_iter(&lines_iterator.next().unwrap())
        .map(|m| m.as_str().parse::<i64>().unwrap())
        .collect();
    let mut seeds_iterator = seed_numbers.iter();

    let mut ranges: Vec<Range> = Vec::new();
    while let Some(seed) = seeds_iterator.next() {
        ranges.push(Range {
            start: *seed,
            length: *seeds_iterator.next().unwrap(),
        });
    }

    // skip empty line
    lines_iterator.next();

    let range_mapping_batches_iterator = RangeMappingBatchesIterator {
        lines: Box::new(lines_iterator),
    };
    for range_mapping_batch in range_mapping_batches_iterator {
        let mut updated_ranges: Vec<Range> = Vec::new();
        for range in &ranges {
            let mut non_updated_ranges: Vec<Range> = Vec::new();
            non_updated_ranges.push(*range);
            for range_mapping in &range_mapping_batch {
                let (newly_updated_ranges, still_non_updated_ranges) =
                    range_mapping.apply_to_ranges(&non_updated_ranges);
                updated_ranges = updated_ranges
                    .into_iter()
                    .chain(newly_updated_ranges)
                    .collect();
                non_updated_ranges = still_non_updated_ranges;
            }

            // anything still left in non_updated_ranges has been checked against every
            // mapping in this batch, so we know that its updated value is the same as its
            // previous value. thus we can just append non_updated_ranges to updated_ranges.
            updated_ranges = updated_ranges
                .into_iter()
                .chain(non_updated_ranges)
                .collect();
        }

        ranges = updated_ranges;
    }

    println!(
        "lowest location: {}",
        ranges.iter().map(|range| range.start).min().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_other_below() {
        // arrange
        let target = Range {
            start: 10,
            length: 10,
        };
        let splitter = Range {
            start: 0,
            length: 8,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                None,
                None,
                Some(Range {
                    start: 10,
                    length: 10,
                })
            )
        )
    }

    #[test]
    fn test_split_other_above() {
        // arrange
        let target = Range {
            start: 0,
            length: 10,
        };
        let splitter = Range {
            start: 12,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                Some(Range {
                    start: 0,
                    length: 10,
                }),
                None,
                None,
            )
        )
    }

    #[test]
    fn test_split_other_grazes_below() {
        // arrange
        let target = Range {
            start: 10,
            length: 10,
        };
        let splitter = Range {
            start: 0,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                None,
                None,
                Some(Range {
                    start: 10,
                    length: 10,
                })
            )
        )
    }

    #[test]
    fn test_split_other_grazes_above() {
        // arrange
        let target = Range {
            start: 0,
            length: 10,
        };
        let splitter = Range {
            start: 10,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                Some(Range {
                    start: 0,
                    length: 10,
                }),
                None,
                None,
            )
        )
    }

    #[test]
    fn test_split_other_intersects_top() {
        // arrange
        let target = Range {
            start: 0,
            length: 10,
        };
        let splitter = Range {
            start: 5,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                Some(Range {
                    start: 0,
                    length: 5,
                }),
                Some(Range {
                    start: 5,
                    length: 5
                }),
                None,
            )
        )
    }

    #[test]
    fn test_split_other_intersects_bottom() {
        // arrange
        let target = Range {
            start: 10,
            length: 10,
        };
        let splitter = Range {
            start: 5,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                None,
                Some(Range {
                    start: 10,
                    length: 5
                }),
                Some(Range {
                    start: 15,
                    length: 5,
                }),
            )
        )
    }

    #[test]
    fn test_split_other_is_nested() {
        // arrange
        let target = Range {
            start: 10,
            length: 10,
        };
        let splitter = Range {
            start: 12,
            length: 5,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                Some(Range {
                    start: 10,
                    length: 2,
                }),
                Some(Range {
                    start: 12,
                    length: 5
                }),
                Some(Range {
                    start: 17,
                    length: 3,
                }),
            )
        )
    }

    #[test]
    fn test_split_other_wraps() {
        // arrange
        let target = Range {
            start: 12,
            length: 5,
        };
        let splitter = Range {
            start: 10,
            length: 10,
        };

        // act
        let result = target.split(splitter);

        // assert
        assert_eq!(
            result,
            (
                None,
                Some(Range {
                    start: 12,
                    length: 5
                }),
                None
            )
        )
    }
}

use lazy_static::lazy_static;
use regex::Regex;
use std::vec::Vec;

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

struct Range {
    destination_start: i64,
    source_start: i64,
    range_length: i64,
}

struct RangesIterator<'a> {
    lines: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> Iterator for RangesIterator<'a> {
    type Item = Vec<Range>;

    fn next(&mut self) -> Option<Vec<Range>> {
        // skip section header
        if self.lines.next() == None {
            return None;
        }

        let mut ranges: Vec<Range> = Vec::new();
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

            ranges.push(Range {
                destination_start: numbers[0],
                source_start: numbers[1],
                range_length: numbers[2],
            })
        }

        return Some(ranges);
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

    let ranges_iterator = RangesIterator {
        lines: Box::new(lines_iterator),
    };
    for ranges in ranges_iterator {
        for value in values.iter_mut() {
            for range in &ranges {
                if *value >= range.source_start && *value < range.source_start + range.range_length
                {
                    *value = *value - range.source_start + range.destination_start;
                    break;
                }
            }
        }
    }

    println!("lowest location: {}", values.iter().min().unwrap());
}

pub fn step_2(_lines: impl IntoIterator<Item = String>) {}

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

struct RangeMappingsIterator<'a> {
    lines: Box<dyn Iterator<Item = String> + 'a>,
}

impl<'a> Iterator for RangeMappingsIterator<'a> {
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

    let range_mappings_iterator = RangeMappingsIterator {
        lines: Box::new(lines_iterator),
    };
    for range_mappings in range_mappings_iterator {
        for value in values.iter_mut() {
            for range_mapping in &range_mappings {
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

pub fn step_2(_lines: impl IntoIterator<Item = String>) {}

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref SYMBOL_PATTERN: Regex = Regex::new(r"[^\.\d]").unwrap();
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
    static ref GEAR_PATTERN: Regex = Regex::new(r"\*").unwrap();
}

fn get_symbol_indices(line: &str) -> HashSet<usize> {
    return SYMBOL_PATTERN.find_iter(line).map(|m| m.start()).collect();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();

    let mut current_line_option = lines_iterator.next();
    let mut next_line_option = lines_iterator.next();

    let mut previous_symbol_indices: HashSet<usize> = HashSet::new();
    let mut current_symbol_indices: HashSet<usize> = match &current_line_option {
        Some(line) => get_symbol_indices(&line),
        None => HashSet::new(),
    };
    let mut next_symbol_indices: HashSet<usize> = match &next_line_option {
        Some(line) => get_symbol_indices(&line),
        None => HashSet::new(),
    };

    let mut sum = 0;

    while let Some(current_line) = current_line_option {
        for m in NUMBER_PATTERN.find_iter(&current_line) {
            let range_start = if m.start() == 0 { 0 } else { m.start() - 1 };
            for i in range_start..(m.end() + 1) {
                if previous_symbol_indices.contains(&i)
                    || current_symbol_indices.contains(&i)
                    || next_symbol_indices.contains(&i)
                {
                    sum += m.as_str().parse::<i32>().unwrap();
                    break;
                }
            }
        }

        current_line_option = next_line_option;
        next_line_option = lines_iterator.next();

        previous_symbol_indices = current_symbol_indices;
        current_symbol_indices = next_symbol_indices;
        next_symbol_indices = match &next_line_option {
            Some(line) => get_symbol_indices(&line),
            None => HashSet::new(),
        }
    }

    println!("sum: {}", sum);
}

struct Gear {
    index: usize,
    product: i32,
    count: i32,
}

fn get_gears(line: &str) -> Vec<Gear> {
    return GEAR_PATTERN
        .find_iter(line)
        .map(|m| Gear {
            index: m.start(),
            product: 1,
            count: 0,
        })
        .collect();
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();

    let mut current_line_option = lines_iterator.next();
    let mut next_line_option = lines_iterator.next();

    let mut previous_gears: Vec<Gear> = Vec::new();
    let mut current_gears: Vec<Gear> = match &current_line_option {
        Some(line) => get_gears(&line),
        None => Vec::new(),
    };
    let mut next_gears: Vec<Gear> = match &next_line_option {
        Some(line) => get_gears(&line),
        None => Vec::new(),
    };

    let mut sum = 0;

    while let Some(current_line) = current_line_option {
        for m in NUMBER_PATTERN.find_iter(&current_line) {
            let range_start = if m.start() == 0 { 0 } else { m.start() - 1 };
            for gear in previous_gears
                .iter_mut()
                .chain(current_gears.iter_mut())
                .chain(next_gears.iter_mut())
            {
                if gear.index >= range_start && gear.index <= m.end() {
                    gear.product *= m.as_str().parse::<i32>().unwrap();
                    gear.count += 1;
                    break;
                }
            }
        }

        for gear in previous_gears {
            if gear.count == 2 {
                sum += gear.product;
            }
        }

        current_line_option = next_line_option;
        next_line_option = lines_iterator.next();

        previous_gears = current_gears;
        current_gears = next_gears;
        next_gears = match &next_line_option {
            Some(line) => get_gears(&line),
            None => Vec::new(),
        };
    }

    println!("sum: {}", sum);
}

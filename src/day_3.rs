use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref SYMBOL_PATTERN: Regex = Regex::new(r"[^\.\d]").unwrap();
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

fn get_symbol_indices(line: &str) -> HashSet<usize> {
    return SYMBOL_PATTERN.find_iter(line).map(|m| m.start()).collect();
}

pub fn step_1(content: &str) {
    let mut lines = content.lines();

    let mut current_line_option = lines.next();
    let mut next_line_option = lines.next();

    let mut previous_symbol_indices: HashSet<usize> = HashSet::new();
    let mut current_symbol_indices: HashSet<usize> = match current_line_option {
        Some(line) => get_symbol_indices(line),
        None => HashSet::new(),
    };
    let mut next_symbol_indices: HashSet<usize> = match next_line_option {
        Some(line) => get_symbol_indices(line),
        None => HashSet::new(),
    };

    let mut sum = 0;

    while let Some(current_line) = current_line_option {
        for m in NUMBER_PATTERN.find_iter(current_line) {
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
        next_line_option = lines.next();

        previous_symbol_indices = current_symbol_indices;
        current_symbol_indices = next_symbol_indices;
        next_symbol_indices = match next_line_option {
            Some(line) => get_symbol_indices(line),
            None => HashSet::new(),
        }
    }

    println!("sum: {}", sum);
}

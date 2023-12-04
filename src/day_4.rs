use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref TOKEN_PATTERN: Regex = Regex::new(r"\d+|\|").unwrap();
}

pub fn step_1(content: &str) {
    let mut sum = 0;

    for line in content.lines() {
        let mut token_iterator = TOKEN_PATTERN.find_iter(line);
        // skip the card number
        token_iterator.next();

        let mut winning_numbers: HashSet<&str> = HashSet::new();
        while let Some(m) = token_iterator.next() {
            let token = m.as_str();
            if token == "|" {
                break;
            }
            winning_numbers.insert(token);
        }

        let mut points = 0;
        while let Some(m) = token_iterator.next() {
            if !winning_numbers.contains(m.as_str()) {
                continue;
            }

            points = if points == 0 { 1 } else { points * 2 }
        }

        sum += points;
    }

    println!("sum: {}", sum);
}

pub fn step_2(_content: &str) {}

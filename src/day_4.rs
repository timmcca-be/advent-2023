use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashSet, LinkedList};

lazy_static! {
    static ref TOKEN_PATTERN: Regex = Regex::new(r"\d+|\|").unwrap();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;

    for line in lines {
        let mut token_iterator = TOKEN_PATTERN.find_iter(&line);
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

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut num_cards = 0;
    let mut extra_copies_counts: LinkedList<i32> = LinkedList::new();

    for line in lines {
        let mut token_iterator = TOKEN_PATTERN.find_iter(&line);
        // skip the card number
        token_iterator.next();

        let num_extra_copies = match extra_copies_counts.pop_front() {
            Some(value) => value,
            None => 0,
        };
        let multiplier = num_extra_copies + 1;
        num_cards += multiplier;

        let mut winning_numbers: HashSet<&str> = HashSet::new();
        while let Some(m) = token_iterator.next() {
            let token = m.as_str();
            if token == "|" {
                break;
            }
            winning_numbers.insert(token);
        }

        let mut cards_earned = 0;
        while let Some(m) = token_iterator.next() {
            if winning_numbers.contains(m.as_str()) {
                cards_earned += 1;
            }
        }

        for extra_copies in extra_copies_counts.iter_mut() {
            if cards_earned == 0 {
                break;
            }

            cards_earned -= 1;
            *extra_copies += multiplier;
        }
        for _ in 0..cards_earned {
            extra_copies_counts.push_back(multiplier);
        }
    }

    println!("number of cards: {}", num_cards);
}

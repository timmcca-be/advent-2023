use lazy_static::lazy_static;
use std::collections::HashMap;
use std::vec::Vec;

lazy_static! {
    static ref STEP_1_CARD_VALUES: HashMap<u8, i32> =
        [b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A']
            .into_iter()
            .enumerate()
            .map(|(i, byte)| (byte, i as i32))
            .collect();
    static ref STEP_2_CARD_VALUES: HashMap<u8, i32> =
        [b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A']
            .into_iter()
            .enumerate()
            .map(|(i, byte)| (byte, i as i32))
            .collect();
}

struct Hand {
    value: i32,
    bid: i32,
}

fn get_hand_type_value(hand: &str, is_step_2: bool) -> i32 {
    let mut wildcards = 0;
    let mut counts: HashMap<u8, i32> = HashMap::new();
    for byte in hand.bytes() {
        if is_step_2 && byte == b'J' {
            wildcards += 1;
            continue;
        }
        let old_count = match counts.get(&byte) {
            Some(count) => *count,
            None => 0,
        };
        counts.insert(byte, old_count + 1);
    }

    let mut sorted_counts = counts
        .into_iter()
        .map(|(_, count)| count)
        .collect::<Vec<i32>>();
    sorted_counts.sort_unstable_by(|a, b| b.cmp(a));

    let highest = match sorted_counts.get(0) {
        Some(count) => *count,
        None => 0,
    } + wildcards;
    let second_highest = match sorted_counts.get(1) {
        Some(count) => *count,
        None => 0,
    };

    let is_five_of_kind = highest == 5;
    if is_five_of_kind {
        return 6;
    }

    let is_four_of_kind = highest == 4;
    if is_four_of_kind {
        return 5;
    }

    let is_full_house = highest == 3 && second_highest == 2;
    if is_full_house {
        return 4;
    }

    let is_three_of_kind = highest == 3;
    if is_three_of_kind {
        return 3;
    }

    let is_two_pair = highest == 2 && second_highest == 2;
    if is_two_pair {
        return 2;
    }

    let is_one_pair = highest == 2;
    if is_one_pair {
        return 1;
    }

    // high card
    return 0;
}

const CARDS_PER_SUIT: i32 = 13;

fn evaluate_hand_value(hand: &str, is_step_2: bool) -> i32 {
    return hand
        .bytes()
        .map(|byte| {
            if is_step_2 {
                STEP_2_CARD_VALUES[&byte]
            } else {
                STEP_1_CARD_VALUES[&byte]
            }
        })
        .fold(get_hand_type_value(hand, is_step_2), |a, b| {
            a * CARDS_PER_SUIT + b
        });
}

fn execute(lines: impl IntoIterator<Item = String>, is_step_2: bool) {
    let mut hands = lines
        .into_iter()
        .map(|line| Hand {
            value: evaluate_hand_value(&line[..5], is_step_2),
            bid: line[6..].parse::<i32>().unwrap(),
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable_by(|a, b| a.value.cmp(&b.value));

    let sum = hands
        .iter()
        .map(|hand| hand.bid)
        .enumerate()
        .map(|(i, bid)| bid * (i as i32 + 1))
        .fold(0, |a, b| a + b);

    println!("sum: {}", sum);
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    execute(lines, false);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    execute(lines, true);
}

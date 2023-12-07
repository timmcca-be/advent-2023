use std::collections::HashMap;
use std::vec::Vec;

struct Hand {
    value: i32,
    bid: i32,
}

fn evaluate_card(card: u8) -> i32 {
    return match card {
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("unexpected card"),
    };
}

fn get_hand_type_value(hand: &str) -> i32 {
    let mut counts: HashMap<u8, i32> = HashMap::new();
    for byte in hand.bytes() {
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

    let is_five_of_kind = sorted_counts[0] == 5;
    if is_five_of_kind {
        return 6;
    }

    let is_four_of_kind = sorted_counts[0] == 4;
    if is_four_of_kind {
        return 5;
    }

    let is_full_house = sorted_counts[0] == 3 && sorted_counts[1] == 2;
    if is_full_house {
        return 4;
    }

    let is_three_of_kind = sorted_counts[0] == 3;
    if is_three_of_kind {
        return 3;
    }

    let is_two_pair = sorted_counts[0] == 2 && sorted_counts[1] == 2;
    if is_two_pair {
        return 2;
    }

    let is_one_pair = sorted_counts[0] == 2;
    if is_one_pair {
        return 1;
    }

    // high card
    return 0;
}

const CARDS_PER_SUIT: i32 = 13;

fn evaluate_hand_value(hand: &str) -> i32 {
    return hand
        .bytes()
        .map(|byte| evaluate_card(byte))
        .fold(get_hand_type_value(hand), |a, b| a * CARDS_PER_SUIT + b);
}

pub fn step_1(lines: impl Iterator<Item = String>) {
    let mut hands = lines
        .map(|line| Hand {
            value: evaluate_hand_value(&line[..5]),
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

pub fn step_2(_lines: impl Iterator<Item = String>) {}

use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use regex::Regex;
use std::vec::Vec;

fn hash(label: &str) -> u64 {
    let mut current = 0;
    for byte in label.bytes() {
        current += byte as u64;
        current *= 17;
        current %= 256;
    }
    return current;
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let line = lines.into_iter().next().unwrap();

    let mut total = 0;
    for step in line.split(",") {
        total += hash(step);
    }

    println!("total: {}", total);
}

lazy_static! {
    static ref STEP_PATTERN: Regex =
        Regex::new(r"(?<label>\w+)(?:-|=(?<focal_length>\d+))$").unwrap();
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let line = lines.into_iter().next().unwrap();

    let mut boxes: Vec<LinkedHashMap<&str, u64>> = (0..256).map(|_| LinkedHashMap::new()).collect();
    for step in line.split(",") {
        let m = STEP_PATTERN.captures(step).unwrap();
        let label = m.name("label").unwrap().as_str();
        let box_index = hash(label) as usize;
        match m.name("focal_length") {
            Some(focal_length) => {
                let value = boxes[box_index].entry(label).or_default();
                *value = focal_length.as_str().parse().unwrap();
            }
            None => {
                boxes[box_index].remove(label);
            }
        }
    }

    let mut total = 0;
    for (box_index, box_map) in boxes.into_iter().enumerate() {
        for (slot_index, focal_length) in box_map.values().into_iter().enumerate() {
            total += (box_index as u64 + 1) * (slot_index as u64 + 1) * focal_length;
        }
    }

    println!("{}", total);
}

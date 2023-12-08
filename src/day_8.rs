use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Write;

lazy_static! {
    static ref NODE_PATTERN: Regex =
        Regex::new(r"(?<node>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
}

pub fn step_1(lines: impl Iterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let instructions_str = lines_iterator.next().unwrap();
    let instructions = instructions_str.as_bytes();
    // skip empty line
    lines_iterator.next();

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    while let Some(line) = lines_iterator.next() {
        let captures = NODE_PATTERN.captures(&line).unwrap();
        let node = captures.name("node").unwrap().as_str().to_owned();
        let left = captures.name("left").unwrap().as_str().to_owned();
        let right = captures.name("right").unwrap().as_str().to_owned();

        node_map.insert(node, (left, right));
    }

    let mut current = "AAA";
    let mut counter = 0;
    while current != "ZZZ" {
        let mapping = &node_map[current];
        let instruction = instructions[counter % instructions.len()];

        current = if instruction == b'L' {
            &mapping.0
        } else {
            &mapping.1
        };

        counter += 1;
    }

    println!("steps: {}", counter);
}

pub fn step_2(lines: impl Iterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let instructions_str = lines_iterator.next().unwrap();
    let instructions = instructions_str.as_bytes();
    // skip empty line
    lines_iterator.next();

    let mut current_nodes: Vec<String> = Vec::new();
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    while let Some(line) = lines_iterator.next() {
        let captures = NODE_PATTERN.captures(&line).unwrap();
        let node = captures.name("node").unwrap().as_str().to_owned();
        let left = captures.name("left").unwrap().as_str().to_owned();
        let right = captures.name("right").unwrap().as_str().to_owned();

        if node.ends_with("A") {
            current_nodes.push(node.clone());
        }

        node_map.insert(node, (left, right));
    }

    let mut counter: u64 = 0;
    while !current_nodes.iter().all(|node| node.ends_with("Z")) {
        if counter % 1000000 == 0 {
            print!("\rcounter crossed {}", counter);
            io::stdout().flush().unwrap();
        }
        let instruction = instructions[(counter % instructions.len() as u64) as usize];

        for node in current_nodes.iter_mut() {
            let mapping = &node_map[node];

            *node = if instruction == b'L' {
                mapping.0.clone()
            } else {
                mapping.1.clone()
            };
        }

        counter += 1;
    }

    println!();
    println!("steps: {}", counter);
}

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NODE_PATTERN: Regex =
        Regex::new(r"(?<node>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
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

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let instructions_str = lines_iterator.next().unwrap();
    let instructions = instructions_str.as_bytes();
    // skip empty line
    lines_iterator.next();

    let mut starting_nodes: Vec<String> = Vec::new();
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    while let Some(line) = lines_iterator.next() {
        let captures = NODE_PATTERN.captures(&line).unwrap();
        let node = captures.name("node").unwrap().as_str().to_owned();
        let left = captures.name("left").unwrap().as_str().to_owned();
        let right = captures.name("right").unwrap().as_str().to_owned();

        if node.ends_with("A") {
            starting_nodes.push(node.clone());
        }

        node_map.insert(node, (left, right));
    }

    for node in starting_nodes {
        let mut current = &node;
        let mut visited: HashMap<String, Vec<i32>> = HashMap::new();
        let mut loop_start = 0;
        let mut counter = 0;
        loop {
            let mut found_loop = false;
            if let Some(indices) = visited.get_mut(current) {
                for index in indices.iter() {
                    if index % instructions.len() as i32 == counter % instructions.len() as i32 {
                        loop_start = *index;
                        found_loop = true;
                        break;
                    }
                }
                if !found_loop {
                    indices.push(counter);
                }
            } else {
                visited.insert(current.to_string(), vec![counter]);
            }

            if found_loop {
                break;
            }

            if current.ends_with("Z") {
                println!("z at {}", counter);
            }

            let mapping = &node_map[current];
            let instruction = instructions[counter as usize % instructions.len()];

            current = if instruction == b'L' {
                &mapping.0
            } else {
                &mapping.1
            };

            counter += 1;
        }

        println!("loop from {} to {}", loop_start, counter);
        println!();
    }
}

// OUTPUT
// z at 20803
// loop from 2 to 20805
//
// z at 17873
// loop from 2 to 17875
//
// z at 23147
// loop from 3 to 23150
//
// z at 15529
// loop from 2 to 15531
//
// z at 17287
// loop from 2 to 17289
//
// z at 19631
// loop from 3 to 19634
//
// END_OUTPUT
//
// since there's only one z for each of these and loop_end - z == loop_start for each,
// we just need the LCM of the z's, which is 21003205388413.

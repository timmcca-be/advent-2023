use std::collections::HashMap;

fn count_possible_combinations(
    record: &[u8],
    groups: &[usize],
    memo: &mut HashMap<String, u64>,
) -> u64 {
    let memo_key = format!(
        "{}-{}",
        record
            .into_iter()
            .map(|byte| *byte as char)
            .collect::<String>(),
        groups.len()
    );
    if let Some(result) = memo.get(&memo_key) {
        return *result;
    }

    if groups.len() == 0 {
        let result = if record.contains(&b'#') { 0 } else { 1 };
        memo.insert(memo_key, result);
        return result;
    } else if record.len() == 0 {
        memo.insert(memo_key, 0);
        return 0;
    }

    let mut total = 0;
    let byte = record[0];
    if byte != b'#' {
        total += count_possible_combinations(&record[1..], groups, memo);
    }
    if byte != b'.' && record.len() >= groups[0] && !record[1..groups[0]].contains(&b'.') {
        if record.len() > groups[0] && record[groups[0]] != b'#' {
            total += count_possible_combinations(&record[(groups[0] + 1)..], &groups[1..], memo);
        } else if record.len() == groups[0] && groups.len() == 1 {
            total += 1;
        }
    }

    // println!(
    //     "{}, {}, {}",
    //     record
    //         .into_iter()
    //         .fold("".to_owned(), |a, b| format!("{}{}", a, *b as char)),
    //     groups.len(),
    //     total,
    // );

    memo.insert(memo_key, total);
    return total;
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let record = split[0];
        let groups: Vec<usize> = split[1]
            .split(",")
            .map(|group_str| group_str.parse::<usize>().unwrap())
            .collect();
        total += count_possible_combinations(record.as_bytes(), &groups, &mut HashMap::new());
    }

    println!("total: {}", total);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        let record = split[0];
        let groups: Vec<usize> = split[1]
            .split(",")
            .map(|group_str| group_str.parse::<usize>().unwrap())
            .collect();

        let mut joined_record = record.to_owned();
        let mut joined_groups = groups.clone();
        for _ in 0..4 {
            joined_record += "?";
            joined_record += record;
            joined_groups = joined_groups
                .iter()
                .chain(&groups)
                .map(|ptr| *ptr)
                .collect();
        }
        total += count_possible_combinations(
            joined_record.as_bytes(),
            &joined_groups,
            &mut HashMap::new(),
        );
    }

    println!("total: {}", total);
}

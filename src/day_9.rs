use std::vec::Vec;

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;

    for line in lines {
        let mut values: Vec<i32> = line
            .split(" ")
            .map(|string| string.parse::<i32>().unwrap())
            .collect();

        let mut end = values.len();
        while values[0..end].iter().any(|value| *value != 0) {
            end -= 1;
            for j in 0..end {
                values[j] = values[j + 1] - values[j];
            }
        }

        sum += values[end..].iter().fold(0, |a, b| a + b);
    }

    println!("sum: {}", sum);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;

    for line in lines {
        let mut values: Vec<i32> = line
            .split(" ")
            .map(|string| string.parse::<i32>().unwrap())
            .collect();

        let mut start = 0;
        while values[start..].iter().any(|value| *value != 0) {
            start += 1;
            for i in (start..values.len()).rev() {
                values[i] -= values[i - 1];
            }
        }

        sum += values[0..start].iter().rev().fold(0, |a, b| b - a);
    }

    println!("sum: {}", sum);
}

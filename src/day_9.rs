use std::vec::Vec;

pub fn step_1(lines: impl Iterator<Item = String>) {
    let mut sum = 0;

    for line in lines {
        let mut values: Vec<i32> = line
            .split(" ")
            .map(|string| string.parse::<i32>().unwrap())
            .collect();

        let mut start: i32 = 0;
        while values[(start as usize)..].iter().any(|value| *value != 0) {
            start += 1;
            for i in ((start as usize)..values.len()).rev() {
                values[i] -= values[i - 1];
            }
        }

        let mut result = 0;
        while start >= 0 {
            for i in (start as usize)..(values.len() - 1) {
                values[i + 1] += values[i];
            }
            result += values[values.len() - 1];
            start -= 1;
        }

        sum += result;
    }

    println!("sum: {}", sum);
}

pub fn step_2(lines: impl Iterator<Item = String>) {
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

        let mut result = 0;
        while end < values.len() {
            for i in (0..end).rev() {
                values[i] = values[i + 1] - values[i];
            }
            result = values[0] - result;
            end += 1;
        }

        sum += result;
    }

    println!("sum: {}", sum);
}

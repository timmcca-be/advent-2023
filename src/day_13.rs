use std::vec::Vec;

fn is_power_of_2(value: u64) -> bool {
    return value != 0 && value & (value - 1) == 0;
}

fn find_mirror(sequence: &Vec<u64>, is_step_2: bool) -> usize {
    for candidate in 1..sequence.len() {
        let mut is_mirror = true;
        let mut found_smudge = false;
        for distance in 0..candidate {
            let high_index = candidate + distance;
            if high_index >= sequence.len() {
                break;
            }
            let low_index = candidate - distance - 1;
            if sequence[low_index] == sequence[high_index] {
                continue;
            }
            if is_step_2
                && !found_smudge
                && is_power_of_2(sequence[low_index] ^ sequence[high_index])
            {
                found_smudge = true;
                continue;
            }
            is_mirror = false;
            break;
        }

        if is_mirror && (!is_step_2 || found_smudge) {
            return candidate;
        }
    }

    return 0;
}

fn execute(lines: impl IntoIterator<Item = String>, is_step_2: bool) {
    let mut total = 0;

    let mut lines_iterator = lines.into_iter();
    let mut reached_end = false;
    while !reached_end {
        let mut rows: Vec<u64> = Vec::new();
        let mut columns: Vec<u64> = Vec::new();

        reached_end = true;
        let mut line_index = 0;
        while let Some(line) = lines_iterator.next() {
            if line == "" {
                reached_end = false;
                break;
            }

            rows.push(0);
            while columns.len() < line.len() {
                columns.push(0);
            }
            for (byte_index, byte) in line.bytes().enumerate() {
                let byte_value = if byte == b'#' { 1 } else { 0 };
                rows[line_index] <<= 1;
                rows[line_index] += byte_value;
                columns[byte_index] <<= 1;
                columns[byte_index] += byte_value;
            }

            line_index += 1;
        }

        total += 100 * find_mirror(&rows, is_step_2) + find_mirror(&columns, is_step_2);
    }

    println!("total: {}", total);
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    execute(lines, false);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    execute(lines, true);
}

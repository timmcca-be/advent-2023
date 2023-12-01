use std::collections::HashSet;
use std::vec::Vec;

fn execute(lines: impl IntoIterator<Item = String>, empty_line_bonus: usize) {
    let mut non_empty_rows: HashSet<usize> = HashSet::new();
    let mut non_empty_columns: HashSet<usize> = HashSet::new();

    let mut coordinates: Vec<(usize, usize)> = Vec::new();

    let mut num_rows = 0;
    let mut num_columns = 0;

    for (line_index, line) in lines.into_iter().enumerate() {
        num_rows += 1;
        if num_columns == 0 {
            num_columns = line.len();
        }
        for (byte_index, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                non_empty_rows.insert(line_index);
                non_empty_columns.insert(byte_index);
                coordinates.push((line_index, byte_index));
            }
        }
    }

    let empty_rows: Vec<usize> = (0..num_rows)
        .filter(|row_index| !non_empty_rows.contains(&row_index))
        .collect();
    let empty_columns: Vec<usize> = (0..num_columns)
        .filter(|column_index| !non_empty_columns.contains(&column_index))
        .collect();

    let adjusted_coordinates: Vec<(usize, usize)> = coordinates
        .into_iter()
        .map(|(row, column)| {
            let mut result = (row, column);

            for row_index in &empty_rows {
                if *row_index > row {
                    break;
                }
                result.0 += empty_line_bonus;
            }

            for column_index in &empty_columns {
                if *column_index > column {
                    break;
                }
                result.1 += empty_line_bonus;
            }

            return result;
        })
        .collect();

    let sum_of_distances = adjusted_coordinates
        .iter()
        .enumerate()
        .map(|(index, a)| {
            adjusted_coordinates
                .iter()
                .skip(index + 1)
                .map(|b| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
                .fold(0, |m, n| m + n)
        })
        .fold(0, |m, n| m + n);

    println!("sum of distances: {}", sum_of_distances);
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    execute(lines, 1);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    execute(lines, 999999);
}

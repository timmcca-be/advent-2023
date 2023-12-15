use std::collections::HashMap;
use std::vec::Vec;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Group {
    column_index: usize,
    last_resting_point: usize,
    num_rocks: u64,
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut working_groups: Vec<Group> = Vec::new();
    let mut complete_groups: Vec<Group> = Vec::new();
    let mut line_index = 0;
    for line in lines {
        while working_groups.len() < line.len() {
            working_groups.push(Group {
                column_index: working_groups.len(),
                last_resting_point: 0,
                num_rocks: 0,
            })
        }

        for (byte_index, byte) in line.bytes().enumerate() {
            match byte {
                b'O' => working_groups[byte_index].num_rocks += 1,
                b'#' => {
                    if working_groups[byte_index].num_rocks > 0 {
                        complete_groups.push(working_groups[byte_index]);
                    }
                    working_groups[byte_index] = Group {
                        column_index: byte_index,
                        last_resting_point: line_index + 1,
                        num_rocks: 0,
                    }
                }
                _ => {}
            }
        }

        line_index += 1;
    }

    let num_lines = line_index as u64;
    let mut total = 0;
    for group in complete_groups.into_iter().chain(working_groups) {
        for rock_index in 0..group.num_rocks {
            total += num_lines - (group.last_resting_point as u64 + rock_index);
        }
    }

    println!("total: {}", total);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Dish {
    lines: Vec<Vec<u8>>,
}

impl Dish {
    fn convert_coordinates(
        &self,
        direction: Direction,
        row: usize,
        column: usize,
    ) -> (usize, usize) {
        let num_lines = self.lines.len();
        let num_bytes_per_line = self.lines[0].len();

        return match direction {
            Direction::North => (row, column),
            Direction::South => (num_lines - row - 1, num_bytes_per_line - column - 1),
            Direction::East => (column, num_bytes_per_line - row - 1),
            Direction::West => (num_lines - column - 1, row),
        };
    }

    fn get(&self, direction: Direction, row: usize, column: usize) -> u8 {
        let (line_index, byte_index) = self.convert_coordinates(direction, row, column);
        return self.lines[line_index][byte_index];
    }

    fn size(&self, direction: Direction) -> (usize, usize) {
        let num_lines = self.lines.len();
        let num_bytes_per_line = self.lines[0].len();
        if direction == Direction::North || direction == Direction::South {
            return (num_lines, num_bytes_per_line);
        }
        return (num_bytes_per_line, num_lines);
    }

    fn process_group(&mut self, direction: Direction, group: Group) {
        let (num_rows, _) = self.size(direction);
        for row_index in group.last_resting_point..num_rows {
            let (line_index, byte_index) =
                self.convert_coordinates(direction, row_index, group.column_index);
            if self.lines[line_index][byte_index] == b'#' {
                break;
            }
            if row_index - group.last_resting_point < group.num_rocks as usize {
                self.lines[line_index][byte_index] = b'O';
            } else {
                self.lines[line_index][byte_index] = b'.';
            }
        }
    }
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut dish = Dish {
        lines: lines.into_iter().map(|line| line.into_bytes()).collect(),
    };

    let mut visited_map: HashMap<String, u64> = HashMap::new();
    let mut visited_vec: Vec<String> = Vec::new();

    let mut i = 0;
    let loop_start: u64;
    let loop_end: u64;
    loop {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            let (num_rows, num_columns) = dish.size(direction);

            let mut working_groups: Vec<Group> = (0..num_columns)
                .map(|column_index| Group {
                    column_index,
                    last_resting_point: 0,
                    num_rocks: 0,
                })
                .collect();

            for row_index in 0..num_rows {
                for column_index in 0..num_columns {
                    let byte = dish.get(direction, row_index, column_index);
                    match byte {
                        b'O' => working_groups[column_index].num_rocks += 1,
                        b'#' => {
                            if working_groups[column_index].num_rocks > 0 {
                                dish.process_group(direction, working_groups[column_index]);
                            }
                            working_groups[column_index] = Group {
                                column_index,
                                last_resting_point: row_index + 1,
                                num_rocks: 0,
                            }
                        }
                        _ => {}
                    }
                }
            }

            for group in working_groups {
                if group.num_rocks > 0 {
                    dish.process_group(direction, group);
                }
            }
        }

        let mut as_string = "".to_owned();
        for line in &dish.lines {
            if as_string != "" {
                as_string += "\n";
            }
            as_string += &line
                .into_iter()
                .map(|byte| *byte as char)
                .collect::<String>();
        }

        if let Some(prior_index) = visited_map.get(&as_string) {
            loop_start = *prior_index;
            loop_end = i;
            break;
        }
        visited_map.insert(as_string.clone(), i);
        visited_vec.push(as_string);

        i += 1;
    }

    let loop_length = loop_end - loop_start;
    let final_index = ((1000000000 - 1 - loop_start) % loop_length) + loop_start;

    let mut load = 0;
    for (line_index, line) in visited_vec[final_index as usize].split("\n").enumerate() {
        for byte in line.bytes() {
            if byte == b'O' {
                load += dish.lines.len() - line_index;
            }
        }
    }

    println!("{}", load);
}

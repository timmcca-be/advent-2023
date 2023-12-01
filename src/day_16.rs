use std::collections::HashSet;
use std::vec::Vec;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn move_in_direction(
    line_index: i32,
    byte_index: i32,
    direction: Direction,
) -> (i32, i32, Direction) {
    let mut new_line_index = line_index;
    let mut new_byte_index = byte_index;
    match direction {
        Direction::North => new_line_index -= 1,
        Direction::South => new_line_index += 1,
        Direction::East => new_byte_index += 1,
        Direction::West => new_byte_index -= 1,
    }
    return (new_line_index, new_byte_index, direction);
}

fn execute(
    layout: &Vec<Vec<u8>>,
    start_line_index: i32,
    start_byte_index: i32,
    start_direction: Direction,
) -> usize {
    let mut beams: Vec<(i32, i32, Direction)> = Vec::new();
    beams.push((start_line_index, start_byte_index, start_direction));

    let mut energized: HashSet<(i32, i32, Direction)> = HashSet::new();

    while let Some(entry) = beams.pop() {
        let (line_index, byte_index, direction) = entry;
        if line_index < 0
            || line_index >= layout.len() as i32
            || byte_index < 0
            || byte_index >= layout[0].len() as i32
        {
            continue;
        }
        if energized.contains(&entry) {
            continue;
        }
        energized.insert(entry);
        match layout[line_index as usize][byte_index as usize] {
            b'.' => beams.push(move_in_direction(line_index, byte_index, direction)),
            b'|' => {
                if direction == Direction::North || direction == Direction::South {
                    beams.push(move_in_direction(line_index, byte_index, direction));
                } else {
                    beams.push(move_in_direction(line_index, byte_index, Direction::North));
                    beams.push(move_in_direction(line_index, byte_index, Direction::South));
                }
            }
            b'-' => {
                if direction == Direction::East || direction == Direction::West {
                    beams.push(move_in_direction(line_index, byte_index, direction));
                } else {
                    beams.push(move_in_direction(line_index, byte_index, Direction::East));
                    beams.push(move_in_direction(line_index, byte_index, Direction::West));
                }
            }
            b'\\' => match direction {
                Direction::North => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::West))
                }
                Direction::West => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::North))
                }
                Direction::South => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::East))
                }
                Direction::East => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::South))
                }
            },
            b'/' => match direction {
                Direction::North => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::East))
                }
                Direction::East => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::North))
                }
                Direction::South => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::West))
                }
                Direction::West => {
                    beams.push(move_in_direction(line_index, byte_index, Direction::South))
                }
            },
            _ => panic!("unexpected byte"),
        }
    }

    return energized
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>()
        .len();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let layout: Vec<Vec<u8>> = lines.into_iter().map(|line| line.into_bytes()).collect();

    println!("result: {}", execute(&layout, 0, 0, Direction::East));
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let layout: Vec<Vec<u8>> = lines.into_iter().map(|line| line.into_bytes()).collect();
    let mut max_value = 0;
    for (line_index, line) in layout.iter().enumerate() {
        max_value = std::cmp::max(
            max_value,
            execute(&layout, line_index as i32, 0, Direction::East),
        );
        max_value = std::cmp::max(
            max_value,
            execute(
                &layout,
                line_index as i32,
                line.len() as i32 - 1,
                Direction::West,
            ),
        );
    }
    for (byte_index, _) in layout[0].iter().enumerate() {
        max_value = std::cmp::max(
            max_value,
            execute(&layout, 0, byte_index as i32, Direction::South),
        );
        max_value = std::cmp::max(
            max_value,
            execute(
                &layout,
                layout.len() as i32 - 1,
                byte_index as i32,
                Direction::North,
            ),
        );
    }

    println!("result: {}", max_value);
}

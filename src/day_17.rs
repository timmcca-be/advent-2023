use std::vec::Vec;
use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn opposite_direction(direction: Direction) -> Direction {
    return match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    };
}

#[derive(Eq)]
struct Position {
    x: usize,
    y: usize,
    last_direction: Direction,
    last_direction_count: u8,
    cost: u64,
    // history: Vec<Direction>,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        let cost_cmp = self.cost.cmp(&other.cost);
        if cost_cmp != Ordering::Equal {
            return cost_cmp;
        }
        let x_cmp = other.x.cmp(&self.x);
        if x_cmp != Ordering::Equal {
            return x_cmp;
        }
        let y_cmp = other.y.cmp(&self.y);
        if y_cmp != Ordering::Equal {
            return y_cmp;
        }
        let direction_cmp = self.last_direction.cmp(&other.last_direction);
        if direction_cmp != Ordering::Equal {
            return direction_cmp;
        }
        return self.last_direction_count.cmp(&other.last_direction_count);
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let layout: Vec<Vec<u64>> = lines
        .into_iter()
        .map(|line| line.bytes().map(|byte| (byte - b'0') as u64).collect())
        .collect();

    let height = layout.len();
    let width = layout[0].len();

    let mut positions: BTreeSet<Position> = BTreeSet::new();
    positions.insert(Position {
        x: 0,
        y: 0,
        last_direction: Direction::East,
        last_direction_count: 0,
        cost: 0,
        // history: Vec::new(),
    });

    while let Some(position) = positions.pop_first() {
        if position.x == width - 1 && position.y == height - 1 {
            println!("result: {}", position.cost);
            // for direction in position.history {
            //     println!(
            //         "{}",
            //         match direction {
            //             Direction::North => "N",
            //             Direction::South => "S",
            //             Direction::East => "E",
            //             Direction::West => "W",
            //         }
            //     )
            // }
            return;
        }

        for (new_x, new_y, direction) in [
            (position.x as i64 - 1, position.y as i64, Direction::West),
            (position.x as i64 + 1, position.y as i64, Direction::East),
            (position.x as i64, position.y as i64 - 1, Direction::North),
            (position.x as i64, position.y as i64 + 1, Direction::South),
        ] {
            if new_x < 0
                || new_x as usize >= width
                || new_y < 0
                || new_y as usize >= height
                || (position.last_direction == direction && position.last_direction_count == 3)
                || direction == opposite_direction(position.last_direction)
            {
                continue;
            }
            // let mut history = position.history.clone();
            // history.push(direction);
            positions.insert(Position {
                x: new_x as usize,
                y: new_y as usize,
                last_direction: direction,
                last_direction_count: if position.last_direction == direction {
                    position.last_direction_count + 1
                } else {
                    1
                },
                cost: position.cost + layout[new_y as usize][new_x as usize],
                // history,
            });
        }
    }

    println!("did not get to end");
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let layout: Vec<Vec<u64>> = lines
        .into_iter()
        .map(|line| line.bytes().map(|byte| (byte - b'0') as u64).collect())
        .collect();

    let height = layout.len();
    let width = layout[0].len();

    let mut positions: BTreeSet<Position> = BTreeSet::new();
    positions.insert(Position {
        x: 0,
        y: 0,
        last_direction: Direction::East,
        last_direction_count: 0,
        cost: 0,
        // history: Vec::new(),
    });
    positions.insert(Position {
        x: 0,
        y: 0,
        last_direction: Direction::South,
        last_direction_count: 0,
        cost: 0,
        // history: Vec::new(),
    });

    while let Some(position) = positions.pop_first() {
        if position.x == width - 1 && position.y == height - 1 {
            println!("result: {}", position.cost);
            // for direction in position.history {
            //     println!(
            //         "{}",
            //         match direction {
            //             Direction::North => "N",
            //             Direction::South => "S",
            //             Direction::East => "E",
            //             Direction::West => "W",
            //         }
            //     )
            // }
            return;
        }

        for (new_x, new_y, direction) in [
            (position.x as i64 - 1, position.y as i64, Direction::West),
            (position.x as i64 + 1, position.y as i64, Direction::East),
            (position.x as i64, position.y as i64 - 1, Direction::North),
            (position.x as i64, position.y as i64 + 1, Direction::South),
        ] {
            if new_x < 0
                || new_x as usize >= width
                || new_y < 0
                || new_y as usize >= height
                || (position.last_direction == direction && position.last_direction_count == 10)
                || (position.last_direction != direction && position.last_direction_count < 4)
                || direction == opposite_direction(position.last_direction)
            {
                continue;
            }
            // let mut history = position.history.clone();
            // history.push(direction);
            positions.insert(Position {
                x: new_x as usize,
                y: new_y as usize,
                last_direction: direction,
                last_direction_count: if position.last_direction == direction {
                    position.last_direction_count + 1
                } else {
                    1
                },
                cost: position.cost + layout[new_y as usize][new_x as usize],
                // history,
            });
        }
    }

    println!("did not get to end");
}

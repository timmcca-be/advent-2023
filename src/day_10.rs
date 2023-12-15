use std::vec::Vec;

const NORTH: u8 = 1;
const SOUTH: u8 = 2;
const EAST: u8 = 4;
const WEST: u8 = 8;

fn move_position(position: (usize, usize), direction: u8) -> Option<(usize, usize)> {
    return match direction {
        NORTH => {
            if position.0 > 0 {
                Some((position.0 - 1, position.1))
            } else {
                None
            }
        }
        SOUTH => Some((position.0 + 1, position.1)),
        EAST => Some((position.0, position.1 + 1)),
        WEST => {
            if position.1 > 0 {
                Some((position.0, position.1 - 1))
            } else {
                None
            }
        }
        _ => panic!("unknown direction"),
    };
}

fn opposite_direction(direction: u8) -> u8 {
    return match direction {
        NORTH => SOUTH,
        SOUTH => NORTH,
        EAST => WEST,
        WEST => EAST,
        _ => panic!("unknown direction"),
    };
}

fn get_pipe_directions(pipe_map: &Vec<String>, position: (usize, usize)) -> u8 {
    return match pipe_map[position.0].as_bytes()[position.1] {
        b'-' => EAST | WEST,
        b'|' => NORTH | SOUTH,
        b'L' => NORTH | EAST,
        b'J' => NORTH | WEST,
        b'F' => SOUTH | EAST,
        b'7' => SOUTH | WEST,
        _ => 0,
    };
}

struct State {
    position: (usize, usize),
    direction: u8,
}

impl State {
    fn update(&mut self, pipe_map: &Vec<String>) {
        self.position = move_position(self.position, self.direction).unwrap();
        let directions = get_pipe_directions(&pipe_map, self.position);
        let opposite_current_direction = opposite_direction(self.direction);
        self.direction = directions ^ opposite_current_direction;
    }
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let pipe_map: Vec<String> = lines.into_iter().collect();
    let (start_line_index, start_line) = (&pipe_map)
        .into_iter()
        .enumerate()
        .find(|(_, line)| line.contains("S"))
        .unwrap();

    let start_position = (start_line_index, start_line.find("S").unwrap());

    let mut states: Vec<State> = Vec::new();
    for direction in [NORTH, SOUTH, EAST, WEST] {
        if let Some(moved_position) = move_position(start_position, direction) {
            if get_pipe_directions(&pipe_map, moved_position) & opposite_direction(direction) != 0 {
                states.push(State {
                    position: start_position,
                    direction,
                });
            }
        }
    }

    if states.len() != 2 {
        panic!("expected two connections to S");
    }

    let mut count = 0;
    loop {
        count += 1;
        states[0].update(&pipe_map);
        if states[0].position == states[1].position {
            panic!("even number loop");
        }
        states[1].update(&pipe_map);
        if states[0].position == states[1].position {
            break;
        }
    }

    println!("furthest distance: {}", count);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let pipe_map: Vec<String> = lines.into_iter().collect();
    let (start_line_index, start_line) = (&pipe_map)
        .into_iter()
        .enumerate()
        .find(|(_, line)| line.contains("S"))
        .unwrap();

    let start_position = (start_line_index, start_line.find("S").unwrap());

    let start_directions: Vec<u8> = [NORTH, SOUTH, EAST, WEST]
        .into_iter()
        .filter(|direction| {
            if let Some(moved_position) = move_position(start_position, *direction) {
                return get_pipe_directions(&pipe_map, moved_position)
                    & opposite_direction(*direction)
                    != 0;
            }
            return false;
        })
        .collect();

    if start_directions.len() != 2 {
        panic!("expected two connections to S");
    }

    let mut state = State {
        position: start_position,
        direction: start_directions[0],
    };

    let mut edge_map: Vec<Vec<u8>> = pipe_map
        .iter()
        .map(|line| line.bytes().map(|_| 0).collect::<Vec<u8>>())
        .collect();
    edge_map[start_position.0][start_position.1] = start_directions[0] | start_directions[1];

    loop {
        state.update(&pipe_map);
        if state.position == start_position {
            break;
        }
        edge_map[state.position.0][state.position.1] =
            get_pipe_directions(&pipe_map, state.position);
    }

    let area_inside_loop = edge_map
        .into_iter()
        .map(|edges| {
            let mut count = 0;
            let mut inside_loop = false;
            let mut last_boundary = 0;
            for pipe in edges {
                if pipe == 0 {
                    if inside_loop {
                        count += 1;
                    }
                    continue;
                }
                if pipe & (NORTH | SOUTH) == 0 {
                    continue;
                }
                if pipe != NORTH | SOUTH
                    && last_boundary & (NORTH | SOUTH) == opposite_direction(pipe & (NORTH | SOUTH))
                {
                    // handle the F--J case, where even though the J has a vertical component,
                    // we don't enter or exit the loop.
                    continue;
                }
                inside_loop = !inside_loop;
                last_boundary = pipe;
            }
            return count;
        })
        .fold(0, |a, b| a + b);

    println!("area inside loop: {}", area_inside_loop);
}

use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Debug)]
enum Rotation {
    L,
    R,
    // Identity - don't rotate
    I,
}

#[derive(Debug, Clone)]
enum Orientation {
    L,
    R,
    U,
    D,
}
impl Orientation {
    fn facing(&self) -> i32 {
        match self {
            Orientation::L => 2,
            Orientation::R => 0,
            Orientation::U => 3,
            Orientation::D => 1,
        }
    }

    fn rotate(&self, rotation: &Rotation) -> Orientation {
        match (self, rotation) {
            (Orientation::L, Rotation::L) => Orientation::D,
            (Orientation::L, Rotation::R) => Orientation::U,
            (Orientation::R, Rotation::L) => Orientation::U,
            (Orientation::R, Rotation::R) => Orientation::D,
            (Orientation::U, Rotation::L) => Orientation::L,
            (Orientation::U, Rotation::R) => Orientation::R,
            (Orientation::D, Rotation::L) => Orientation::R,
            (Orientation::D, Rotation::R) => Orientation::L,
            (_, Rotation::I) => self.clone(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    magnitude: i32,
    rotation: Rotation,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Coord {
    row: i32,
    col: i32,
}
impl Coord {
    fn add(&self, other: &Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }

    fn scale(&self, magnitude: i32) -> Self {
        Self {
            row: self.row * magnitude,
            col: self.col * magnitude,
        }
    }

    fn translate(&self, unit_vector: &Self, magnitude: i32) -> Self {
        let vector = unit_vector.scale(magnitude);
        self.add(&vector)
    }
}

#[derive(Clone, Debug)]
struct State {
    coordinate: Coord,
    orientation: Orientation,
}
impl State {
    fn new(board: &Board) -> State {
        let mut start_col = 0;
        while start_col < board.width {
            let coord = Coord {
                row: 0,
                col: start_col as i32,
            };
            let tile = board.map.get(&coord);
            if tile.is_some() && tile.unwrap().tile == '.' {
                return State {
                    coordinate: coord,
                    orientation: Orientation::R,
                };
            }
            start_col += 1;
        }
        panic!("Starting tile not found");
    }
}

#[derive(Debug)]
struct Node {
    position: Coord,
    tile: char,
    neighbor_up: Option<Coord>,
    neighbor_down: Option<Coord>,
    neighbor_left: Option<Coord>,
    neighbor_right: Option<Coord>,
}
impl Node {
    fn new_empty(position: Coord, tile: char) -> Self {
        Node {
            position,
            tile,
            neighbor_up: None,
            neighbor_down: None,
            neighbor_left: None,
            neighbor_right: None,
        }
    }
}

#[derive(Debug)]
struct Board {
    map: HashMap<Coord, Node>,
    width: i32,
    height: i32,
}
impl Board {
    fn get_tile(&self, coord: &Coord) -> Option<char> {
        self.map.get(coord).map(|n| n.tile)
    }
    fn try_move_one(
        &self,
        prev_position: Coord,
        current_position: Coord,
        orientation: &Orientation,
    ) -> Coord {
        let current_node = self
            .map
            .get(&current_position)
            .expect("Current node should be in board");
        let neighbor = match orientation {
            Orientation::L => current_node.neighbor_left.clone(),
            Orientation::R => current_node.neighbor_right.clone(),
            Orientation::U => current_node.neighbor_up.clone(),
            Orientation::D => current_node.neighbor_down.clone(),
        }
        .expect("Neighbor should always exist");

        let neighbor_tile = self
            .get_tile(&neighbor)
            .expect("Neighbor tile should exist");

        match neighbor_tile {
            ' ' => self.try_move_one(prev_position, neighbor, orientation),
            '.' => neighbor,
            '#' => prev_position,
            _ => panic!("Shouldn't be any other tile"),
        }
    }
}
impl From<Vec<String>> for Board {
    fn from(lines: Vec<String>) -> Self {
        let mut node_map = HashMap::new();
        let mut key_set = HashSet::new();

        let height = lines.len() as i32;
        let width = lines[0].chars().collect::<Vec<_>>().len() as i32;

        for row in 0..lines.len() {
            for col in 0..lines[0].len() {
                let coord = Coord {
                    row: row as i32,
                    col: col as i32,
                };
                let tile = lines[row].chars().collect::<Vec<_>>()[col];
                let node = Node::new_empty(coord.clone(), tile);
                key_set.insert(coord.clone());
                node_map.insert(coord, node);
            }
        }

        // Connect grid to neighbors
        for (coord, node) in node_map.iter_mut() {
            let upper = Coord {
                row: coord.row - 1,
                col: coord.col,
            };
            let upper_neighbor = key_set.get(&upper).map(|c| c.to_owned());
            node.neighbor_up = upper_neighbor;
            let lower = Coord {
                row: coord.row + 1,
                col: coord.col,
            };
            let lower_neighbor = key_set.get(&lower).map(|c| c.to_owned());
            node.neighbor_down = lower_neighbor;
            let left = Coord {
                row: coord.row,
                col: coord.col - 1,
            };
            let left_neighbor = key_set.get(&left).map(|c| c.to_owned());
            node.neighbor_left = left_neighbor;
            let right = Coord {
                row: coord.row,
                col: coord.col + 1,
            };
            let right_neighbor = key_set.get(&right).map(|c| c.to_owned());
            node.neighbor_right = right_neighbor;
        }

        // Connect edges - Left to right
        for row in 0..lines.len() {
            let head_coord = Coord {
                row: row as i32,
                col: 0,
            };
            let tail_coord = Coord {
                row: row as i32,
                col: width - 1,
            };

            let head_node = node_map
                .get_mut(&head_coord)
                .expect("Head node should exist");
            head_node.neighbor_left = Some(tail_coord.clone());

            let tail_node = node_map
                .get_mut(&tail_coord)
                .expect("Tail node should exist");
            tail_node.neighbor_right = Some(head_coord.clone());
        }

        // Connect edges - Up to down
        for col in 0..width {
            let head_coord = Coord {
                row: 0,
                col: col as i32,
            };
            let tail_coord = Coord {
                row: height - 1,
                col: col as i32,
            };

            let head_node = node_map
                .get_mut(&head_coord)
                .expect("Head node should exist");
            head_node.neighbor_up = Some(tail_coord.clone());

            let tail_node = node_map
                .get_mut(&tail_coord)
                .expect("Tail node should exist");
            tail_node.neighbor_down = Some(head_coord.clone());
        }

        Board {
            map: node_map,
            width,
            height,
        }
    }
}

fn walk(board: &Board, instructions: &Vec<Instruction>) -> State {
    // Find the startig point
    let mut current_state = State::new(board);

    // println!(
    //     "Initial state: {:#?}\n==============================",
    //     current_state
    // );

    // For each instruction, try and follow
    for instruction in instructions {
        current_state = follow_instruction(board, &current_state, instruction);
    }

    // Return final position and orientation
    current_state
}

fn follow_instruction(board: &Board, state: &State, instruction: &Instruction) -> State {
    // println!("Processing instruction: {:#?} - {:#?}", state, instruction);
    // stdin().read(&mut [0]).unwrap();

    let mut current_position = state.coordinate.clone();

    for _ in 0..instruction.magnitude {
        let new_position = board.try_move_one(
            current_position.clone(),
            current_position.clone(),
            &state.orientation,
        );
        if current_position == new_position {
            break;
        }
        let tile = board.map.get(&new_position);
        if tile.unwrap().tile == '#' {
            break;
        }
        current_position = new_position;
    }

    // Update orientation
    let orientation = state.orientation.rotate(&instruction.rotation);

    State {
        coordinate: current_position,
        orientation,
    }
}

struct Day22 {}
impl AoCProblem for Day22 {
    fn name(&self) -> String {
        "day-22".to_owned()
    }
}
impl Solution for Day22 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read path")
            .map(|line| line.expect("Should be able to read line"))
            .collect::<Vec<_>>();

        let last_line = format!(
            // Tack on an identity rotation at the end, just for symmetry
            "{}:I",
            lines.last().expect("Should have instructions").clone()
        );
        let instructions = last_line
            .replace('L', ":L:")
            .replace('R', ":R:")
            .split(':')
            .collect::<Vec<_>>()
            // Frustratingly, .array_chunks(..) isn't stable yet, so we window and skip
            // every odd numbered entry
            .windows(2)
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, v)| {
                let magnitude = v[0]
                    .parse::<i32>()
                    .expect("Should be able to parse magniture");
                let rotation = match v[1] {
                    "L" => Rotation::L,
                    "R" => Rotation::R,
                    "I" => Rotation::I,
                    _ => panic!("Unrecognized rotation"),
                };
                Instruction {
                    magnitude,
                    rotation,
                }
            })
            .collect::<Vec<_>>();

        let board: Board = lines
            .clone()
            .iter()
            .rev()
            .skip(2)
            .rev()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>()
            .into();

        // println!("{:#?}", board);

        println!("Instructions: {:#?}", instructions);

        let final_position = walk(&board, &instructions);
        println!(
            "Part one: {}",
            (1000 * (final_position.coordinate.row + 1))
                + (4 * (final_position.coordinate.col + 1))
                + (final_position.orientation.facing())
        )
    }
}

fn main() {
    Day22 {}.test_and_run()
}

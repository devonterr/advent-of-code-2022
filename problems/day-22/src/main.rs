use std::io::{stdin, Read};

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
    fn unit(&self) -> Coord {
        match self {
            Orientation::L => Coord { row: 0, col: -1 },
            Orientation::R => Coord { row: 0, col: 1 },
            Orientation::U => Coord { row: -1, col: 0 },
            Orientation::D => Coord { row: 1, col: 0 },
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

fn bounds(board: &[String]) -> (usize, usize) {
    (board[0].chars().collect::<Vec<_>>().len(), board.len())
}

#[derive(Debug)]
struct Instruction {
    magnitude: i32,
    rotation: Rotation,
}

#[derive(Clone, Debug)]
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

    fn out_of_bounds(&self, board: &[String]) -> bool {
        // Return true if coordinate out of bounds
        let (width, height) = bounds(board);
        if self.row >= height as i32 {
            return true;
        }

        if self.col >= width as i32 {
            return true;
        }

        false
    }

    fn wrap(&mut self, board: &[String], unit: &Coord) -> char {
        // Check if current coordinate is out of bounds of a given board
        // If it is, mod coordinates by board size
        // If at an empty space, add unit and recurse
        // If at a # stop
        let oob = self.out_of_bounds(board);
        if oob {
            println!("Oob!: {:#?}", self);
            let (width, height) = bounds(board);
            self.row = self.row % (height as i32);
            self.col = self.col % (width as i32);
        } else {
            println!("Not oob: {:#?}", self);
        }
        let current_tile = board[self.row as usize].chars().collect::<Vec<_>>()[self.col as usize];
        if current_tile == '#' {
            return current_tile;
        }
        if current_tile == ' ' {
            self.add(unit);
            return self.wrap(board, unit);
        }
        return current_tile;
    }
}

#[derive(Clone, Debug)]
struct State {
    coordinate: Coord,
    orientation: Orientation,
}
impl State {
    fn new(board: &[String]) -> State {
        let start_col = board[0].find('.').expect("First . should exist");
        State {
            coordinate: Coord {
                row: 0,
                col: start_col as i32,
            },
            orientation: Orientation::R,
        }
    }
}

fn walk(board: &Vec<String>, instructions: &Vec<Instruction>) -> State {
    // Find the startig point
    let mut current_state = State::new(board);

    // For each instruction, try and follow
    for instruction in instructions {
        current_state = follow_instruction(board, &current_state, instruction);
    }

    // Return final position and orientation
    current_state
}

fn follow_instruction(board: &Vec<String>, state: &State, instruction: &Instruction) -> State {
    println!("Processing instruction: {:#?} - {:#?}", instruction, state);
    stdin().read(&mut [0]).unwrap();
    let unit = state.orientation.unit();
    let mut current_position = state.coordinate.clone();

    for _ in 0..instruction.magnitude {
        // Check the board at (x,y)+unit
        let peek_position = next_position(board, &current_position, &unit);
        let peek_value = board[peek_position.row as usize]
            .clone()
            .chars()
            .collect::<Vec<_>>()[peek_position.col as usize];
        // If # break
        match peek_value {
            '#' => break,
            _ => {
                current_position = peek_position;
            }
        }
    }

    // Update orientation
    let orientation = state.orientation.rotate(&instruction.rotation);

    State {
        coordinate: current_position,
        orientation,
    }
}

fn next_position(board: &Vec<String>, current_position: &Coord, unit: &Coord) -> Coord {
    // Add unit to the current position
    // Wrap around if necessary

    let mut next_position = current_position.add(unit);
    let next_tile = next_position.wrap(board, unit);
    if next_tile == '#' {
        return current_position.clone();
    }
    next_position
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

        let board = lines
            .clone()
            .iter()
            .rev()
            .skip(2)
            .rev()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();

        println!("{:#?}", board);

        println!("Instructions: {:#?}", instructions);

        walk(&board, &instructions);
    }
}

fn main() {
    Day22 {}.test()
}

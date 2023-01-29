use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Debug)]
enum Rotation {
    L,
    R,
    // Identity - don't rotate
    I,
}

#[derive(Debug)]
enum Orientation {
    L,
    R,
    U,
    D,
}

#[derive(Debug)]
struct Instruction {
    magnitude: i32,
    rotation: Rotation,
}

struct Coord {
    row: i32,
    col: i32,
}

fn walk(board: &Vec<String>, instructions: &Vec<Instruction>) -> (Coord, Orientation) {
    let mut orientation = Orientation::R;
    // Find the startig point
    let initial_column = board[0].find(".").expect("Should have initial position") as i32;
    let mut current_coord = Coord {
        row: 0,
        col: initial_column,
    };

    // For each instruction, try and follow
    for instruction in instructions {
        (current_coord, orientation) = follow_instruction(
            board,
            &Coord {
                row: current_coord.row,
                col: current_coord.col,
            },
            orientation,
            instruction,
        );
    }

    // Return final position and orientation
    return (
        Coord {
            row: current_coord.row,
            col: current_coord.col,
        },
        orientation,
    );
}

fn follow_instruction(
    board: &Vec<String>,
    current_coord: &Coord,
    current_orientation: Orientation,
    instruction: &Instruction,
) -> (Coord, Orientation) {
    todo!()
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

        println!("Instructions: {:#?}", instructions);
    }
}

fn main() {
    Day22 {}.test()
}

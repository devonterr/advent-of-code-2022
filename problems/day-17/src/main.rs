use std::fmt::Display;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Copy, Clone, Debug)]
enum Shape {
    Dash = 0,
    Plus,
    BackwardsL,
    VerticalLine,
    Square,
}
impl From<usize> for Shape {
    fn from(value: usize) -> Self {
        match value % 5 {
            0 => Shape::Dash,
            1 => Shape::Plus,
            2 => Shape::BackwardsL,
            3 => Shape::VerticalLine,
            4 => Shape::Square,
            _ => panic!("Unrecognized shape enum value"),
        }
    }
}
impl Shape {
    fn to_bitfield(&self) -> Vec<u8> {
        match self {
            Shape::Dash => vec!["00111100"],
            Shape::Plus => vec!["00010000", "00111000", "00010000"],
            Shape::BackwardsL => vec!["00111000", "00001000", "00001000"],
            Shape::VerticalLine => vec!["00100000", "00100000", "00100000", "00100000"],
            Shape::Square => vec!["00110000", "00110000"],
        }
        .iter()
        .map(|s| u8::from_str_radix(s, 2).expect("Should be able to parse binary string"))
        .collect::<Vec<u8>>()
    }
    fn shift(bitfield: &Vec<u8>, op: Op) -> Vec<u8> {
        // Try to apply shift operation, respecting bounds
        let mut result = vec![];
        let left_bound =
            u8::from_str_radix("10000000", 2).expect("Should be able to parse left bound");
        for v in bitfield {
            match (v, op) {
                (underflow, Op::Left) if *underflow >= left_bound => {
                    return bitfield.clone();
                }
                (overflow, Op::Right) if (overflow >> 1) % 2 == 1 => {
                    return bitfield.clone();
                }
                (v, Op::Right) => result.push(v >> 1),
                (v, Op::Left) => result.push(v << 1),
            }
        }
        result
    }

    fn display(shape: &[u8]) -> String {
        shape
            .iter()
            .map(|v| format!("\n{:08b}", v))
            .map(|s| s.replace('0', "."))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

struct Grid(Vec<u8>);
impl Grid {
    fn new(rounds: usize) -> Grid {
        let mut result = vec![0, 0, 0]; // Start with extra space to spawn new lines
        for _ in 0..rounds {
            // Tallest piece is 4 rows; make the maximal grid
            result.extend(vec![0, 0, 0, 0]);
        }
        Grid(result)
    }

    fn highest(&self) -> usize {
        let mut result = 0;
        let mut i = 0;
        for v in self.0.iter() {
            i += 1;
            if *v > 0 {
                result = i;
            }
        }
        result
    }

    fn collides(&self, origin: usize, bitfield: &Vec<u8>) -> bool {
        // Returns false if a given bitfield collides with an existing shape
        for i in 0..bitfield.len() {
            let grid_row = self.0[i + origin];
            let shape_row = bitfield[i];
            if grid_row & shape_row != 0 {
                return true;
            }
        }
        false
    }

    fn round(
        &mut self,
        ops: &mut impl Iterator<Item = Op>,
        shapes: &mut impl Iterator<Item = Shape>,
    ) {
        let mut shape_origin = self.highest() + 3;
        let mut shape = shapes.next().expect("Should have a shape").to_bitfield();
        // println!("Round: {}", Shape::display(&shape));
        loop {
            // Get an op, transform shape, and check for overflow/collision
            let op = ops.next().expect("Should be another op");
            let shifted_shape = Shape::shift(&shape, op);
            if !self.collides(shape_origin, &shifted_shape) {
                // println!("Shift {:#?}, {}", op, Shape::display(&shifted_shape));
                shape = shifted_shape;
            }
            // Reduce origin, check for overflow/collision
            if shape_origin == 0 || self.collides(shape_origin - 1, &shape) {
                break;
            } else {
                shape_origin -= 1;
            }
        }
        self.render_shape(shape, shape_origin);
    }

    fn render_shape(&mut self, shape: Vec<u8>, origin: usize) {
        // println!("Rendering shape {:#?} as {}", shape, Shape::display(&shape));
        for i in 0..shape.len() {
            self.0[i + origin] |= shape[i];
        }
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let highest = self.highest();
        let mut parts = vec![];
        parts.push("-------".to_owned());
        for r in self.0.iter().take(highest + 3).rev() {
            let s = format!("{:08b}|", r).replace('0', ".");
            parts.push(s);
        }
        parts.push("-------".to_owned());
        write!(f, "{}", parts.join("\n"))
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Left,
    Right,
}
impl TryFrom<char> for Op {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Op::Left),
            '>' => Ok(Op::Right),
            _ => Err("Unrecognized operation".to_owned()),
        }
    }
}

struct Day17 {}
impl AoCProblem for Day17 {
    fn name(&self) -> String {
        "day-17".to_owned()
    }
}
impl Solution for Day17 {
    fn solution(&self, path: &str) {
        let line = read_lines(path)
            .expect("Should be able to read input file")
            .map(|line| line.expect("Should be able to read line"))
            .next()
            .expect("Should have one line");

        let mut ops = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes = (0..5).cycle().map(Shape::from);

        let mut grid = Grid::new(2022);

        for _ in 0..2022 {
            grid.round(&mut ops, &mut shapes);
            // println!("{}", grid);
        }
        println!("Part one: {}", grid.highest());

        // Can we truncate?
        // let highest = u8::from_str_radix("11111110", 2).expect("Should");
        // let mut i = 0;
        // for v in grid.0.iter() {
        //     i += 1;
        //     if *v >= highest {
        //         println!("Found one! {}", i);
        //     }
        // }
    }
}

fn main() {
    // Day17 {}.test();
    Day17 {}.test_and_run();
}

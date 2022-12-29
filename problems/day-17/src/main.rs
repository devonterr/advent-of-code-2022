use std::fmt::Display;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        // (initial_top, shape, end_top)
        let mut shape_origin = self.highest() + 3;
        let shape_type = shapes.next().expect("Should have a shape");
        let mut shape = shape_type.to_bitfield();
        let starting_top = self.top();
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
        self.render_shape(&shape, shape_origin);
        (starting_top, shape, self.top())
    }

    fn render_shape(&mut self, shape: &Vec<u8>, origin: usize) {
        // println!("Rendering shape {:#?} as {}", shape, Shape::display(&shape));
        for i in 0..shape.len() {
            self.0[i + origin] |= shape[i];
        }
    }

    fn top(&self) -> Vec<u8> {
        let track_last_n = 2000;
        let highest = self.highest();
        if highest < track_last_n {
            return self.0[0..highest].to_vec();
            // return vec![0];
        }
        // self.0[highest - 5..highest - 1].to_vec()
        self.0[highest - track_last_n..highest + 1].to_vec()
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

        for i in 0..2022 {
            grid.round(&mut ops, &mut shapes);
        }

        println!("Part one: {}", grid.highest());

        // Part 2
        // Tortoise and hare to do cycle detection

        // Setup two of everything
        let mut ops = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes = (0..5).cycle().map(Shape::from);

        let mut grid = Grid::new(2022);

        let mut ops2 = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes2 = (0..5).cycle().map(Shape::from);

        let mut grid2 = Grid::new(5 * 2022);

        let mut tortoise = grid.round(&mut ops, &mut shapes);
        grid2.round(&mut ops2, &mut shapes2);
        let mut hare = grid2.round(&mut ops2, &mut shapes2);

        // Loop till we detect a cycle
        loop {
            if tortoise == hare {
                break;
            }
            tortoise = grid.round(&mut ops, &mut shapes);
            grid2.round(&mut ops2, &mut shapes2);
            hare = grid2.round(&mut ops2, &mut shapes2);
        }

        // Reset the tortoise, loop until we hit start of cycle again, counting along the way.
        // This gives us the number of rounds and the total height of the prefix phase
        // let mut length_of_prefix = 0;
        let mut number_of_rounds_in_prefix: usize = 0;
        let mut ops = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes = (0..5).cycle().map(Shape::from);

        let mut grid = Grid::new(2022);
        tortoise = grid.round(&mut ops, &mut shapes);
        hare = grid2.round(&mut ops2, &mut shapes2);

        loop {
            if tortoise == hare {
                break;
            }
            tortoise = grid.round(&mut ops, &mut shapes);
            hare = grid2.round(&mut ops2, &mut shapes2);
            number_of_rounds_in_prefix += 1;
        }

        // Now we need to compute the number of rounds it takes to complete a cycle
        // and the height of a cycle
        // let mut length_of_cycle = 1;
        let mut number_of_rounds_per_cycle = 1;
        hare = grid.round(&mut ops, &mut shapes);
        loop {
            if tortoise == hare {
                break;
            }
            hare = grid.round(&mut ops, &mut shapes);
            number_of_rounds_per_cycle += 1;
        }
        println!(
            "Rounds in cycle: {}, Rounds in prefix {}",
            number_of_rounds_per_cycle, number_of_rounds_in_prefix
        );

        // Now, to compute the total height we need to find
        // 1. The number of rounds in the remainder
        // 2. The heigh of the remainder
        // 3. The height of each cycle
        // To do so, make a single pass up to the end of the first cycle
        // and record the heights along the way.

        let number_of_rounds_after_prefix = 1000000000000 - number_of_rounds_in_prefix;
        let number_of_rounds_in_remainder =
            number_of_rounds_after_prefix % number_of_rounds_per_cycle;
        let number_of_cycles = number_of_rounds_after_prefix / number_of_rounds_per_cycle;

        let mut ops = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes = (0..5).cycle().map(Shape::from);

        let mut grid = Grid::new(2022);

        // Run through the prefix; get the starting height
        for _ in 0..number_of_rounds_in_prefix {
            grid.round(&mut ops, &mut shapes);
        }
        // println!("PREFIX");
        // println!("{}", grid);
        // println!("~~~~~~~~~~~");
        let prefix_height = grid.highest();
        // Run through remainder; get the remainder height
        for _ in 0..number_of_rounds_in_remainder {
            grid.round(&mut ops, &mut shapes);
        }
        // println!("REMAINDER");
        // println!("{}", grid);
        // println!("~~~~~~~~~~~");
        let remainder_height = grid.highest() - prefix_height;
        // Finish cycle; get final height to get the total height of a cycle
        for _ in number_of_rounds_in_remainder..number_of_rounds_per_cycle {
            grid.round(&mut ops, &mut shapes);
        }
        let single_cycle_height = grid.highest() - prefix_height;
        // println!("CYCLE");
        // println!("{}", grid);
        // println!("~~~~~~~~~~~");

        // One more cycle for funsies
        for _ in 0..number_of_rounds_per_cycle {
            grid.round(&mut ops, &mut shapes);
        }
        // println!("CYCLE2");
        // println!("{}", grid);
        // println!("~~~~~~~~~~~");

        // Compute total height
        let total_height =
            prefix_height + remainder_height + (number_of_cycles * single_cycle_height);
        println!("Total Height {}", total_height);
    }
}

fn main() {
    Day17 {}.test_and_run();
    // Day17 {}.run();
}

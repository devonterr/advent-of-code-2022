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
        let highest = self.highest();
        if highest < 5 {
            return vec![0];
        }
        self.0[highest - 5..highest - 1].to_vec()
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
        let height_of_cycle = grid2.highest() - grid.highest();
        // println!(
        //     "{} - {} = {}",
        //     grid2.highest(),
        //     grid.highest(),
        //     height_of_cycle
        // );

        // Reset the tortoise, loop until we hit start of cycle again, counting along the way.
        // This gives us the number of rounds and the total height of the prefix phase
        let mut length_of_prefix = 0;
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
            length_of_prefix += 1;
        }
        let height_of_prefix = grid.highest();
        println!(
            "Prefix len {}, prefix height {}",
            length_of_prefix, height_of_prefix
        );

        // Now we need to compute the number of rounds it takes to complete a cycle
        // and the height of a cycle
        let mut length_of_cycle = 1;
        hare = grid.round(&mut ops, &mut shapes);
        loop {
            if tortoise == hare {
                break;
            }
            hare = grid.round(&mut ops, &mut shapes);
            length_of_cycle += 1;
        }
        println!(
            "Len of cycle: {}, height of cycle {}",
            length_of_cycle, height_of_cycle
        );

        // Finally, we can compute the total height
        let number_of_rounds_spent_cycling = 1000000000000 - length_of_prefix;
        let number_of_cycles = number_of_rounds_spent_cycling / length_of_cycle;
        let remainder_rounds =
            number_of_rounds_spent_cycling - (number_of_cycles * length_of_cycle);

        // TODO = need to compute remainder
        let height_of_remainder = 0;

        // GARBAGE
        // TODO - Something is off with the height calculation here
        // Appear to be off by one, though the top looks correct
        let mut ops = line
            .chars()
            .map(Op::try_from)
            .map(|mo| mo.expect("Should be able to parse op"))
            .cycle();

        let mut shapes = (0..5).cycle().map(Shape::from);

        let mut grid = Grid::new(2022);

        for _ in 0..length_of_prefix + 1 {
            grid.round(&mut ops, &mut shapes);
        }
        for _ in 0..length_of_cycle {
            grid.round(&mut ops, &mut shapes);
        }
        println!("After prefix {}", grid.highest());
        let initial_remainder_height = grid.highest();
        for _ in 0..remainder_rounds {
            grid.round(&mut ops, &mut shapes);
        }
        let final_remainder_height = grid.highest();
        println!("After remainder {}", grid.highest());
        let height_of_remainder = final_remainder_height - initial_remainder_height;
        println!("Total remainder {}", height_of_remainder);

        println!("{}", Shape::display(&grid.top()));

        //
        println!(
            "Remainder: {} rounds, total height {}",
            remainder_rounds, height_of_remainder
        );
        println!(
            "Total Rounds: {} (remainder {}) (num cycles {} * len_of_cycles)",
            remainder_rounds + length_of_prefix + (number_of_cycles * length_of_cycle),
            remainder_rounds,
            number_of_cycles,
        );
        let total_height =
            height_of_prefix + (number_of_cycles * height_of_cycle) + height_of_remainder;
        println!("Part 2: {}", total_height);
    }
}

fn main() {
    // Day17 {}.test();
    Day17 {}.test_and_run();
}

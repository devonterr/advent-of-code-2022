use shared::{read_lines, AoCProblem, AoCSolution, Solution};
use std::{collections::HashSet, iter::repeat};

#[derive(Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
struct Command(Direction, usize);
impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value
            .trim()
            .split_once(' ')
            .ok_or("Unable to split on ' '; each line should have a space in it")?;
        let direction = match parts.0 {
            "R" => Ok(Direction::RIGHT),
            "L" => Ok(Direction::LEFT),
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            _ => Err("Unrecognized direction"),
        }?;
        let distance = parts
            .1
            .parse::<usize>()
            .map_err(|_| "Unable to parse distance")?;
        Ok(Command(direction, distance))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Position(i32, i32);
impl Position {
    fn adjacent(&self, other: &Self) -> bool {
        let x_delta = other.0 - self.0;
        let y_delta = other.1 - self.1;

        (x_delta <= 1 && x_delta >= -1) && (y_delta <= 1 && y_delta >= -1)
    }
}

#[derive(Debug)]
struct VisitState {
    previous_head: Position,
    previous_tail: Position,
    visited: HashSet<Position>,
}
impl VisitState {
    fn new() -> Self {
        VisitState {
            previous_head: Position(0, 0),
            previous_tail: Position(0, 0),
            visited: HashSet::from_iter(vec![Position(0, 0)]),
        }
    }
    fn visit(&mut self, command: Command) {
        let new_head = match command {
            Command(Direction::UP, _) => Position(self.previous_head.0, self.previous_head.1 + 1),
            Command(Direction::DOWN, _) => Position(self.previous_head.0, self.previous_head.1 - 1),
            Command(Direction::LEFT, _) => Position(self.previous_head.0 - 1, self.previous_head.1),
            Command(Direction::RIGHT, _) => {
                Position(self.previous_head.0 + 1, self.previous_head.1)
            }
        };

        let new_tail = if new_head.adjacent(&self.previous_tail) {
            self.previous_tail.clone()
        } else {
            match command.0 {
                Direction::UP => Position(new_head.0, new_head.1 - 1),
                Direction::DOWN => Position(new_head.0, new_head.1 + 1),
                Direction::LEFT => Position(new_head.0 + 1, new_head.1),
                Direction::RIGHT => Position(new_head.0 - 1, new_head.1),
            }
        };

        self.visited.insert(new_tail.clone());
        self.previous_head = new_head;
        self.previous_tail = new_tail;
    }
}

struct Day9 {}
impl AoCProblem for Day9 {
    fn name(&self) -> String {
        "day-9".to_owned()
    }
}
impl Solution for Day9 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|l| l.expect("Should be able to read line"));
        let commands: Vec<Command> = lines
            .map(Command::try_from)
            .map(|c| c.expect("Should be able to parse command"))
            // Turn e.g. (R, 1) into [(R, 1), (R,1), (R, 1)] to make them easier to process
            .flat_map(|c| repeat(Command(c.0, 1)).take(c.1))
            .collect();
        let mut visit_state = VisitState::new();
        for command in commands {
            visit_state.visit(command);
        }
        println!("Part one: {:#?}", visit_state.visited.len());
    }
}

fn main() {
    Day9 {}.test_and_run()
}

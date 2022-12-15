use shared::{read_lines, AoCProblem, AoCSolution, Solution};
use std::{collections::HashSet, iter::repeat};

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Command(Direction, usize);
impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value
            .trim()
            .split_once(' ')
            .ok_or("Unable to split on ' '; each line should have a space in it")?;
        let direction = match parts.0 {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
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

        (-1..=1).contains(&x_delta) && (-1..=1).contains(&y_delta)
    }
}

#[derive(Debug)]
struct VisitState {
    nodes: Vec<Position>,
    visited: HashSet<Position>,
}
impl VisitState {
    fn new(node_count: usize) -> Self {
        VisitState {
            nodes: repeat(Position(0, 0))
                .take(node_count)
                .collect::<Vec<Position>>(),
            visited: HashSet::from_iter(vec![Position(0, 0)]),
        }
    }
    fn updated_position(lead: Position, tail: Position) -> Position {
        // If they are in the same row or column, move one to match
        // Otherwise move diagonally to touch
        if lead.0 == tail.0 || lead.1 == tail.1 {
            let delta_y = (lead.1 - tail.1) / 2;
            let delta_x = (lead.0 - tail.0) / 2;
            Position(tail.0 + delta_x, tail.1 + delta_y)
        } else {
            // We know we're moving diagonally, but we need to figure out signs.
            let delta_x = if lead.0 > tail.0 { 1 } else { -1 };
            let delta_y = if lead.1 > tail.1 { 1 } else { -1 };
            Position(tail.0 + delta_x, tail.1 + delta_y)
        }
    }
    fn visit(&mut self, command: Command) {
        let previous_head = self.nodes[0].clone();
        let mut new_head = match command {
            Command(Direction::Up, _) => Position(previous_head.0, previous_head.1 + 1),
            Command(Direction::Down, _) => Position(previous_head.0, previous_head.1 - 1),
            Command(Direction::Left, _) => Position(previous_head.0 - 1, previous_head.1),
            Command(Direction::Right, _) => Position(previous_head.0 + 1, previous_head.1),
        };
        let mut updated_nodes = vec![new_head.clone()];

        for i in 1..self.nodes.len() {
            let previous_tail = self.nodes[i].clone();
            let new_tail = if new_head.adjacent(&previous_tail) {
                previous_tail.clone()
            } else {
                Self::updated_position(new_head, previous_tail)
            };
            updated_nodes.push(new_tail.clone());
            new_head = new_tail;
        }

        self.visited.insert(
            updated_nodes
                .last()
                .expect("Should be a last element")
                .to_owned(),
        );
        self.nodes = updated_nodes;
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
        let mut visit_state = VisitState::new(2);
        for command in commands.clone() {
            visit_state.visit(command);
        }
        println!("Part one: {:#?}", visit_state.visited.len());

        let mut visit_state_2 = VisitState::new(10);
        for command in commands.iter() {
            visit_state_2.visit(command.to_owned());
        }
        println!("Part two: {:#?}", visit_state_2.visited.len());
    }
}

fn main() {
    Day9 {}.test();
    Day9 {}.solution("data/day-9/test2.txt");
    Day9 {}.run();
}

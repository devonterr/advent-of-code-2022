use std::collections::HashMap;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

struct Operation {
    count: u32,
    from: u32,
    to: u32,
}
impl TryFrom<&String> for Operation {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();
        if parts.len() != 6 {
            return Err("Should be able to split line".to_owned());
        }
        let count = parts[1]
            .parse::<u32>()
            .expect("Should be able to parse `count`");
        let from = parts[3]
            .parse::<u32>()
            .expect("Should be able to parse `from`");
        let to = parts[5]
            .parse::<u32>()
            .expect("Should be able to parse `to`");
        Ok(Operation { count, from, to })
    }
}

struct StackLayer(Vec<Option<char>>);
impl TryFrom<&String> for StackLayer {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let parts: Vec<Option<char>> = value
            .chars()
            .enumerate()
            .filter(|e| e.0 % 4 == 1)
            .map(|e| {
                if e.1.is_ascii_uppercase() {
                    Some(e.1)
                } else {
                    None
                }
            })
            .collect();
        Ok(StackLayer(parts))
    }
}

struct State {
    stacks: Vec<String>,
    operations: Vec<Operation>,
}
impl State {
    fn apply_moves(&self) -> State {
        let mut stacks = self.stacks;
        let operations = self.operations;

        for op in operations {
            let mut next_stacks = vec![];

            let from_key = op.from as usize;
            let to_key = op.to as usize;

            for (i, stack) in stacks.iter().enumerate() {
                if i == from_key {
                } else if i == to_key {
                } else {
                    next_stacks.push(stack);
                }
            }
        }

        todo!()
    }

    // Top of the stacks
    fn tots(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().map(|c| c.to_string()).unwrap_or("".to_owned()))
            .collect::<String>()
    }
}
impl TryFrom<(Vec<StackLayer>, Vec<Operation>)> for State {
    type Error = ();

    fn try_from(value: (Vec<StackLayer>, Vec<Operation>)) -> Result<Self, Self::Error> {
        // Essentially a transpose, which I'm sad Rust doesn't provide out of the box.
        // Make our empty stacks
        let mut stacks = value.0[0]
            .0
            .iter()
            .map(|_| vec![])
            .collect::<Vec<Vec<char>>>();
        // Populate them top-down with non-None chars
        for layer in &value.0 {
            let mut idx = 0;
            for v in &layer.0 {
                if v.is_some() {
                    stacks[idx].push(v.unwrap())
                }
                idx += 1
            }
        }
        // Reverse, so the tops are at the ends
        let reversed = stacks
            .iter()
            .map(|s| s.iter().rev().map(|c| c.clone()).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Ok(State {
            stacks: reversed,
            operations: value.1,
        })
    }
}

struct Day5 {}
impl AoCProblem for Day5 {
    fn name(&self) -> String {
        "day-5".to_owned()
    }
}
impl Solution for Day5 {
    fn solution(&self, path: &str) {
        // Trying to be somewhat efficient in parsing here, for no particular reason other than I want to.
        let mut lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"));
        let mut stack_layers = vec![];
        // Iterate through Lines once and adjust which parser is applied as we go through sections
        // Try to parse a stack layer
        while let Some(next_line) = lines.next() {
            let layer = StackLayer::try_from(&next_line);
            if layer.is_err() {
                break;
            } else {
                stack_layers.push(layer.unwrap());
            }
        }
        // Once that failes, indicating that we've hit a blank line, swap to parsing moves
        let mut operations = vec![];
        while let Some(next_line) = lines.next() {
            let next_op = Operation::try_from(&next_line);
            operations.push(next_op.expect("Should be able to parse moves"));
        }

        let state = State::try_from((stack_layers, operations));
        let new_state = state
            .expect("Should be able to construct initial state")
            .apply_moves();
        println!("Part one: {:#?}", new_state.tots())
    }
}

fn main() {
    Day5 {}.test_and_run()
}

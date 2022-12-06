use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Debug)]
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
    type Error = String;

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
        if parts.is_empty() {
            return Err("Failed to parse stack layer".to_owned());
        }
        Ok(StackLayer(parts))
    }
}

enum CraneModel {
    Model9000,
    Model9001,
}

struct State {
    stacks: Vec<String>,
    operations: Vec<Operation>,
}
impl State {
    fn apply_moves(&self, model: CraneModel) -> State {
        let mut stacks = self.stacks.clone();
        for op in &self.operations {
            let count = op.count as usize;
            let from_idx = op.from as usize - 1;
            let to_idx = op.to as usize - 1;
            let from_stack = stacks[from_idx].clone();
            let to_move = match &model {
                CraneModel::Model9000 => from_stack.chars().rev().take(count).collect::<String>(),
                CraneModel::Model9001 => from_stack
                    .chars()
                    .rev()
                    .take(count)
                    // Okay, this is actually annoying.
                    // You can't chain .rev().take(...).rev(), you have to explicitly materialize in between.
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>(),
            };
            stacks[to_idx].push_str(to_move.as_str());
            stacks[from_idx].truncate(from_stack.len() - count);
        }
        State {
            stacks,
            operations: vec![],
        }
    }

    // Top of the stacks
    fn tots(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| {
                stack
                    .chars()
                    .last()
                    .map(|c| c.to_string())
                    .unwrap_or("".to_owned())
            })
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
            .map(|s| s.iter().rev().copied().collect::<String>())
            .collect::<Vec<String>>();
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
        for next_line in lines.by_ref() {
            let layer = StackLayer::try_from(&next_line);
            if layer.is_err() {
                break;
            } else {
                stack_layers.push(layer.unwrap());
            }
        }
        // Once that failes, indicating that we've hit a blank line, swap to parsing moves
        let mut operations = vec![];
        for next_line in lines {
            let next_op = Operation::try_from(&next_line);
            operations.push(next_op.expect("Should be able to parse moves"));
        }

        let state = State::try_from((stack_layers, operations))
            .expect("Should be able to construct initial state");
        let new_state = state.apply_moves(CraneModel::Model9000);
        println!("Part one: {:#?}", new_state.tots());

        let new_state2 = state.apply_moves(CraneModel::Model9001);
        println!("Part two: {:#?}", new_state2.tots());
    }
}

fn main() {
    Day5 {}.test_and_run()
}

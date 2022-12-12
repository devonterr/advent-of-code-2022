use shared::{all_lcm, read_lines, AoCProblem, AoCSolution, Solution};

fn process_rounds<T>(states: &mut Vec<MonkeyState>, n: u64, normalize: T)
where
    T: Fn(u64) -> u64,
{
    for _ in 0..n {
        for i in 0..states.len() {
            let send_items_to = states[i].process_items(&normalize);
            apply_state_updates(states, send_items_to);
        }
    }
}

fn answer(states: Vec<MonkeyState>) -> u64 {
    let mut part_two_results = states.iter().map(|s| s.inspections).collect::<Vec<u64>>();
    part_two_results.sort();
    part_two_results.reverse();
    part_two_results.iter().take(2).product()
}

fn apply_state_updates(states: &mut Vec<MonkeyState>, items_to_send: Vec<(u64, u64)>) {
    for (item, address_u64) in items_to_send {
        let address = address_u64 as usize;
        states[address].receive(item);
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Mult(u64),
    Sum(u64),
    Square,
}
impl Operation {
    fn apply(&self, lhs: u64) -> u64 {
        match self {
            Operation::Mult(x) => lhs * x,
            Operation::Sum(x) => lhs + x,
            Operation::Square => lhs * lhs,
        }
    }
}
impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.eq("old * old") {
            Ok(Operation::Square)
        } else if value.starts_with("old * ") {
            Ok(Operation::Mult(
                value
                    .strip_prefix("old * ")
                    .unwrap()
                    .parse::<u64>()
                    .expect("Should be able to parse operand"),
            ))
        } else if value.starts_with("old + ") {
            Ok(Operation::Sum(
                value
                    .strip_prefix("old + ")
                    .unwrap()
                    .parse::<u64>()
                    .expect("Should be able to parse operand"),
            ))
        } else {
            Err("Should be able to parse operation".to_owned())
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyState {
    items: Vec<u64>,
    test: u64,
    pass: u64,
    fail: u64,
    operation: Operation,
    inspections: u64,
}
impl MonkeyState {
    fn receive(&mut self, item: u64) {
        self.items.push(item);
    }

    fn process_items<T>(&mut self, normalize: T) -> Vec<(u64, u64)>
    where
        T: Fn(u64) -> u64,
    {
        let mut send_items_to = vec![];
        for item in self.items.clone() {
            let new_item = normalize(self.operation.apply(item));
            let send_to = if new_item % self.test == 0 {
                self.pass
            } else {
                self.fail
            };
            send_items_to.push((new_item, send_to));
        }
        self.inspections += send_items_to.len() as u64;
        self.items.clear();
        send_items_to
    }
}
impl TryFrom<String> for MonkeyState {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lines = value.split('\n').collect::<Vec<&str>>();
        let items = lines[1]
            .trim()
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(|e| e.parse::<u64>().expect("Should parse"))
            .collect::<Vec<u64>>();
        let operation = Operation::try_from(
            lines[2]
                .trim()
                .trim_start_matches("Operation: new = ")
                .to_owned(),
        )
        .expect("Should be able to parse operation");
        let test = lines[3]
            .trim()
            .trim_start_matches("Test: divisible by ")
            .parse::<u64>()
            .expect("Should be able to parse test");
        let pass = lines[4]
            .trim()
            .trim_start_matches("If true: throw to monkey ")
            .parse::<u64>()
            .expect("Should be able to parse passing case");
        let fail = lines[5]
            .trim()
            .trim_start_matches("If false: throw to monkey ")
            .parse::<u64>()
            .expect("Should be able to parse fail case");
        Ok(MonkeyState {
            items,
            test,
            pass,
            fail,
            operation,
            inspections: 0,
        })
    }
}

struct Day11 {}
impl AoCProblem for Day11 {
    fn name(&self) -> String {
        "day-11".to_owned()
    }
}
impl Solution for Day11 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"));
        let initial_states: Vec<MonkeyState> = lines
            .collect::<Vec<String>>()
            .chunks(7)
            .map(|chunk| chunk.join("\n"))
            .map(MonkeyState::try_from)
            .map(|ms| ms.expect("Should be able to parse monkey state"))
            .collect();
        let mut states = initial_states.clone();

        process_rounds(&mut states, 20, |x| x / 3);
        println!("Part One: {:#?}", answer(states));

        let mut states_2 = initial_states.clone();
        let least_common_multiple = all_lcm(states_2.iter().map(|s| s.test).collect::<Vec<u64>>());
        process_rounds(&mut states_2, 10000, |x| x % least_common_multiple);
        println!("Part Two: {:#?}", answer(states_2));
    }
}

fn main() {
    Day11 {}.test_and_run();
}

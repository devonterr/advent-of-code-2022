use shared::{read_lines, AoCProblem, AoCSolution, Solution};

enum Command {
    Add(i64),
    Noop,
}
impl Command {
    fn to_cycles(&self) -> Vec<i64> {
        match self {
            Command::Noop => vec![0],
            Command::Add(x) => vec![0, x.to_owned()],
        }
    }
}
impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("noop") {
            return Ok(Command::Noop);
        }
        let parts = value.trim().split_once(' ').ok_or("Should have a space")?;
        let to_add = parts.1.parse::<i64>().map_err(|e| format!("{:#?}", e))?;
        Ok(Command::Add(to_add))
    }
}

#[derive(Debug)]
struct Processor {
    crt_cycle: i64,
    x_register: i64,
    next_delta: i64,
}
impl Processor {
    fn apply_cycle(&mut self, cycle_value: i64) -> String {
        self.x_register += self.next_delta;
        self.next_delta = cycle_value;
        let res = self.render();
        self.crt_cycle += 1;
        self.crt_cycle %= 40;
        res
    }
    fn render(&self) -> String {
        let diff = self.x_register - self.crt_cycle;
        if (-1..=1).contains(&diff) {
            "#".to_owned()
        } else {
            ".".to_owned()
        }
    }
    fn new() -> Self {
        Processor {
            x_register: 1,
            next_delta: 0,
            crt_cycle: 0,
        }
    }
}

struct Day10 {}
impl AoCProblem for Day10 {
    fn name(&self) -> String {
        "day-10".to_owned()
    }
}
impl Solution for Day10 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|l| l.expect("Should be able to read line"));
        let mut cycles = lines
            .map(Command::try_from)
            .flat_map(|c| c.expect("Should have a command").to_cycles())
            .take(220);
        let mut processor = Processor::new();
        let mut result = vec![];
        for _ in 0..20 {
            let cycle = cycles.next().expect("Should have more cycles");
            processor.apply_cycle(cycle);
        }
        result.push(20 * processor.x_register);
        for (chunk_count, chunk) in cycles.collect::<Vec<i64>>().chunks(40).enumerate() {
            for c in chunk {
                processor.apply_cycle(c.to_owned());
            }
            let cycle_number: i64 = (20 + (40 * (1 + chunk_count)))
                .try_into()
                .expect("Should be able to parse chunk as i64");
            result.push(cycle_number * processor.x_register);
        }

        println!("Part one: {:#?}", result.iter().sum::<i64>());

        let all_cycles = read_lines(path)
            .expect("Should be able to read file")
            .map(|l| l.expect("Should be able to read line"))
            .map(Command::try_from)
            .flat_map(|c| c.expect("Should have a command").to_cycles());
        let mut processor2 = Processor::new();
        let mut crt_line = vec![];
        for c in all_cycles {
            crt_line.push(processor2.apply_cycle(c));
        }
        println!("\nPart two:");
        for chunk in crt_line.chunks(40) {
            println!("{}", chunk.join(" "));
        }
    }
}

fn main() {
    Day10 {}.test_and_run();
}

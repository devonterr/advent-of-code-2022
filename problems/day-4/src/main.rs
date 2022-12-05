use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Debug)]
struct Interval {
    left: u32,
    right: u32,
}
impl Interval {
    fn contains(&self, interval: &Interval) -> bool {
        interval.left >= self.left && interval.right <= self.right
    }

    fn overlaps(&self, interval: &Interval) -> bool {
        (interval.left >= self.left && interval.left <= self.right)
            || (interval.right >= self.left && interval.right <= self.right)
            || interval.contains(&self)
    }

    fn new(a: u32, b: u32) -> Interval {
        if a < b {
            Interval { left: a, right: b }
        } else {
            Interval { left: b, right: a }
        }
    }
}
impl TryFrom<&str> for Interval {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<u32> = value
            .split('-')
            .map(|part| {
                part.parse::<u32>()
                    .expect("Should be able to parse each part")
            })
            .take(2)
            .collect();
        if parts.len() != 2 {
            return Err("Unexpected number of parts when parsing".to_owned());
        }
        Ok(Interval::new(parts[0], parts[1]))
    }
}

struct Day4 {}
impl AoCProblem for Day4 {
    fn name(&self) -> String {
        "day-4".to_owned()
    }
}
impl Solution for Day4 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path).expect("Should be able to read file");
        let intervals: Vec<(Interval, Interval)> = lines
            .map(|line| {
                let unwrapped_line = line.expect("Should be able to read line");
                let sub_parts = unwrapped_line
                    .split_once(',')
                    .expect("Line should contain one comma");
                (
                    Interval::try_from(sub_parts.0).expect("Should be able to parse first part"),
                    Interval::try_from(sub_parts.1).expect("Should be able to parse second part"),
                )
            })
            .collect();
        let part_one = intervals
            .iter()
            .filter(|pair| pair.0.contains(&pair.1) || pair.1.contains(&pair.0));

        println!("Part 1: {}", part_one.count());

        let part_two = intervals.iter().filter(|pair| pair.0.overlaps(&pair.1));
        println!("Part 2: {}", part_two.count());
    }
}

fn main() {
    Day4 {}.test_and_run()
}

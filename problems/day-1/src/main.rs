use itertools::Itertools;
use shared::{AoCProblem, AoCSolution, Solution};

struct Day1 {}
impl AoCProblem for Day1 {
    fn name(&self) -> String {
        "day-1".to_owned()
    }
}
impl Solution for Day1 {
    fn solution(&self, path: &str) {
        let lines = shared::read_lines(path)
            .unwrap_or_else(|_| panic!("Expect file to be present: {}", path));
        let groups = shared::segment_lines(lines, |s| s.is_empty());
        let processed: Vec<i32> = groups
            .iter()
            .map(|g| {
                g.iter()
                    .map(|i| {
                        let value: i32 = i.parse().expect("Should parse");
                        value
                    })
                    .sum::<i32>()
            })
            .sorted()
            .collect();
        let max = processed.last();
        let max_3 = processed.iter().rev().take(3).sum::<i32>();
        println!("{:#?}", processed);
        println!("Max: {:?}", max);
        println!("Max 3: {:?}", max_3);
    }
}

fn main() {
    Day1 {}.test_and_run();
}

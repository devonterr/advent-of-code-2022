use std::collections::{hash_map::RandomState, HashSet};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn find_run_of_distinct(line: String, run_length: usize) -> usize {
    line.chars()
        .collect::<Vec<char>>()
        .windows(run_length)
        .enumerate()
        .find(|w| {
            let s: HashSet<char, RandomState> = HashSet::from_iter(w.1.to_owned());
            s.len() == run_length
        })
        // We're tracking the _start_ of the marker by tracking the enumerate value,
        // here we adjust to the _end_ of the marker
        .map(|m| (run_length + m.0, m.1.iter().collect::<String>()))
        .expect("Should find start of packet marker")
        .0
}

struct Day6 {}
impl AoCProblem for Day6 {
    fn name(&self) -> String {
        "day-6".to_owned()
    }
}
impl Solution for Day6 {
    fn solution(&self, path: &str) {
        let line = read_lines(path)
            .expect("Should be able to read input file")
            .next()
            .expect("Input file should be non-empty")
            .expect("Should be able to read line");
        let marker = find_run_of_distinct(line.clone(), 4);
        println!("Part one: {:#?}", marker);

        let message_marker = find_run_of_distinct(line, 14);
        println!("Part two: {:#?}", message_marker);
    }
}

fn main() {
    Day6 {}.test_and_run()
}

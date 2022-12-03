use std::collections::HashSet;
use std::convert::From;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn priority(c: char) -> u32 {
    // Annoyingly the problem puts 'A' after 'a' in the code points, so we have to do some arithmetic to swap them back
    let basis = if c.is_ascii_lowercase() {
        ('a' as u32) - 1
    } else {
        ('A' as u32) - 1 - 26
    };
    (c as u32) - basis
}

struct Day3 {}
impl AoCProblem for Day3 {
    fn name(&self) -> String {
        "day-3".to_owned()
    }
}
impl Solution for Day3 {
    fn solution(&self, path: &str) {
        let shared_items: Vec<char> = read_lines(path)
            .expect("Should be able to read data")
            .map(|line| {
                let text = line.expect("Should be able to read line");
                let parts = text.split_at(text.len() / 2);
                let left = parts.0.chars().collect::<HashSet<char>>();
                let right = parts.1.chars().collect::<HashSet<char>>();
                left.intersection(&right)
                    .take(1)
                    .next()
                    .expect("There should be a single character shared")
                    .clone()
            })
            .collect();
        println!(
            "{:#?}",
            shared_items.iter().map(|i| priority(*i)).sum::<u32>()
        );
    }
}

fn main() {
    Day3 {}.test_and_run();
}

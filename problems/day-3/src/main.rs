use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn priority(c: char) -> u32 {
    // Annoyingly the problem puts 'A' after 'a' in the code points, so we have to do some arithmetic to swap them back
    // a-z is [1, 27)
    let basis = if c.is_ascii_lowercase() {
        ('a' as u32) - 1
    } else {
        // A-Z is [27, 57)
        ('A' as u32) - 1 - 26
    };
    (c as u32) - basis
}

fn find_overlap<'a, I>(vals: I) -> Option<char>
where
    I: Iterator<Item = &'a String>,
{
    vals.map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|p, n| {
            p.intersection(&n)
                .map(|c| c.clone())
                .collect::<HashSet<char>>()
        })
        .expect("Should reduce")
        .iter()
        .take(1)
        .next()
        .map(|c| c.to_owned())
}

fn group_compartments<'a, I>(vals: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = &'a String>,
{
    vals.map(|line| {
        let parts = line.split_at(line.len() / 2);
        vec![parts.0.to_owned(), parts.1.to_owned()]
    })
    .collect()
}

fn group_badges<'a, I>(vals: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = &'a String>,
{
    vals.map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|chunk| Vec::from(chunk))
        .collect::<Vec<Vec<String>>>()
}

struct Day3 {}
impl AoCProblem for Day3 {
    fn name(&self) -> String {
        "day-3".to_owned()
    }
}
impl Solution for Day3 {
    fn solution(&self, path: &str) {
        let lines: Vec<String> = read_lines(path)
            .expect("Should be able to read data")
            .map(|line| line.expect("Should be able to read line"))
            .collect();
        let compartments = group_compartments(lines.iter());
        let shared_item_prioities = compartments
            .iter()
            .map(|c| find_overlap(c.iter()))
            .map(|c| priority(c.expect("Should be an overlap")));
        println!("{:#?}", shared_item_prioities.sum::<u32>());

        let badge_groups = group_badges(lines.iter());
        let badge_priorities = badge_groups
            .iter()
            .map(|g| find_overlap(g.iter()))
            .map(|c| priority(c.expect("Should be an overlap")));
        println!("{:#?}", badge_priorities.sum::<u32>());
    }
}

fn main() {
    Day3 {}.test_and_run();
}

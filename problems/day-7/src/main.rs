use std::collections::HashMap;

use itertools::Itertools;
use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn fan_out<V>(path: String, value: V) -> Vec<(String, V)>
where
    V: Copy,
{
    let segments = path
        .split('/')
        .map(|h| h.to_owned())
        .collect::<Vec<String>>();
    let mut result = vec![];
    for i in 1..segments.len() {
        let key = segments[0..i].join("/");
        result.push((key, value));
    }
    result
}

struct Day7 {}
impl AoCProblem for Day7 {
    fn name(&self) -> String {
        "day-7".to_owned()
    }
}
impl Solution for Day7 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path).expect("Should be able to read file");
        let mut current_path_segments = vec![];
        let mut full_path_file_sizes = HashMap::new();
        for line in lines {
            let text = line.expect("Should be able to read line");
            if text.starts_with("$ ls") || text.starts_with("dir") {
                continue;
            }
            if text.starts_with("$ cd ..") {
                current_path_segments.pop();
            } else if text.starts_with("$ cd /") {
                current_path_segments.clear();
                current_path_segments.push("".to_owned());
            } else if text.starts_with("$ cd ") {
                let segment = text.trim()[5..].to_owned();
                current_path_segments.push(segment);
            } else {
                let (raw_file_size, file_name) = text
                    .trim()
                    .split_once(' ')
                    .expect("Should be able to parse file line");
                let mut key = current_path_segments.join("/");
                key.push('/');
                key.push_str(file_name);
                let file_size = raw_file_size
                    .parse::<u64>()
                    .expect("File size should be u64");
                full_path_file_sizes.insert(key, file_size);
            }
        }
        let all_entries = full_path_file_sizes
            .iter()
            .flat_map(|entry| fan_out(entry.0.to_owned(), entry.1))
            .sorted_by_key(|e| e.0.clone())
            .group_by(|e| e.0.clone());
        let mut sums = HashMap::new();
        for (key, group) in all_entries.into_iter() {
            let sum: u64 = group.map(|g| g.1).sum();
            sums.insert(key, sum);
        }
        let sums_under_100000 = sums
            .iter()
            .filter(|kv| *kv.1 <= 100000)
            .collect::<HashMap<&String, &u64>>();
        println!(
            "Part 1: {:#?}",
            sums_under_100000
                .iter()
                .map(|kv| kv.1.to_owned())
                .sum::<u64>()
        );

        let capacity = 70000000;
        let free = capacity - sums.get("").expect("Root size should exist");
        let update_size = 30000000;
        let to_free = update_size - free;
        let (_to_delete, will_free) = sums
            .iter()
            .sorted_by_key(|kv| kv.1).find(|kv| *kv.1 > to_free)
            .expect("Should be a directory with enough space to free");
        println!("Part 2: {:#?}", will_free);
    }
}

fn main() {
    Day7 {}.test_and_run();
}

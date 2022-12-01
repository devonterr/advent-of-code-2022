use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

static INPUT_FILE_NAME: &str = "data/day-1/input.txt";

fn main() {
    let lines = read_lines(INPUT_FILE_NAME)
        .expect(&format!("Expect file to be present: {}", INPUT_FILE_NAME));
    let groups = segment_lines(lines, |s| s.is_empty());
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn segment_lines<F>(lines: Lines<BufReader<File>>, mut cut_criteria: F) -> Vec<Vec<String>>
where
    F: FnMut(String) -> bool,
{
    let mut groups: Vec<Vec<String>> = vec![];
    let mut group: Vec<String> = vec![];
    for line in lines {
        let text = line.expect("Should be able to read a line");
        if cut_criteria(text.clone()) {
            groups.push(group.clone());
            group = vec![];
        } else {
            group.push(text);
        }
    }
    groups
}

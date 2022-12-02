use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn segment_lines<F>(lines: Lines<BufReader<File>>, mut cut_criteria: F) -> Vec<Vec<String>>
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
    if !group.is_empty() {
        groups.push(group);
    }
    groups
}

pub trait Problem {
    fn test_file(&self) -> String;
    fn input_file(&self) -> String;
}
pub trait Solution {
    fn solution(&self, path: &str);
}
pub trait AoCSolution {
    fn test(&self);
    fn run(&self);
    fn test_and_run(&self) {
        println!("Testing:");
        self.test();
        println!("\nRunning:");
        self.run();
    }
}
pub trait AoCProblem {
    fn name(&self) -> String;
}

impl<T> Problem for T
where
    T: AoCProblem,
{
    fn test_file(&self) -> String {
        format!("data/{}/test.txt", &self.name())
    }

    fn input_file(&self) -> String {
        format!("data/{}/input.txt", &self.name())
    }
}

impl<T> AoCSolution for T
where
    T: Problem + Solution,
{
    fn test(&self) {
        self.solution(&self.test_file())
    }

    fn run(&self) {
        self.solution(&self.input_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

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
    groups
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

use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn neighbors(origin: &(usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    vec![
        (origin.0 + 1, origin.1, origin.2),
        (origin.0, origin.1 + 1, origin.2),
        (origin.0, origin.1, origin.2 + 1),
        (origin.0 - 1, origin.1, origin.2),
        (origin.0, origin.1 - 1, origin.2),
        (origin.0, origin.1, origin.2 - 1),
    ]
}

struct Day18 {}
impl AoCProblem for Day18 {
    fn name(&self) -> String {
        "day-18".to_owned()
    }
}
impl Solution for Day18 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|l| l.expect("Should be able to read line"));

        let origins = lines
            .map(|l| {
                let parts = l
                    .trim()
                    .split(',')
                    .map(|p| {
                        // Shifting everything by one because it makes it easier to deal with usize and underflow.
                        1 + p
                            .parse::<usize>()
                            .expect("Should be able to parse coordinate")
                    })
                    .collect::<Vec<usize>>();
                (parts[0], parts[1], parts[2])
            })
            .collect::<HashSet<(usize, usize, usize)>>();

        let exposed_faces = origins
            .iter()
            .flat_map(|o| neighbors(o).into_iter().filter(|n| !origins.contains(n)))
            .count();

        println!("Part one: {}", exposed_faces);
    }
}

fn main() {
    Day18 {}.test_and_run();
}

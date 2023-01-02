use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

// struct Cube {
//     // Origin assumed to be bottom-left corner
//     origin: (usize, usize, usize),
// }
// impl Cube {
//     fn neighbors(&self) -> Vec<(usize, usize, usize)> {
//         vec![
//             (self.origin.0 + 1, self.origin.1, self.origin.2),
//             (self.origin.0, self.origin.1 + 1, self.origin.2),
//             (self.origin.0, self.origin.1, self.origin.2 + 1),
//             (self.origin.0 - 1, self.origin.1, self.origin.2),
//             (self.origin.0, self.origin.1 - 1, self.origin.2),
//             (self.origin.0, self.origin.1, self.origin.2 - 1),
//         ]
//     }
// }
// impl TryFrom<String> for Cube {
//     type Error = String;

//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         let parts = value
//             .trim()
//             .split(',')
//             .map(|c| {
//                 c.parse::<usize>()
//                     .expect("Should be able to parse origin coordinates")
//             })
//             .collect::<Vec<usize>>();

//         let parts = (parts[0], parts[1], parts[2]);
//         Ok(Cube { origin: parts })
//     }
// }

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

        let mut origins = lines
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
            .flat_map(|o| neighbors(o).into_iter().filter(|n| !origins.contains(&n)))
            .count();

        println!("Part one: {}", exposed_faces);
    }
}

fn main() {
    Day18 {}.test_and_run();
}

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

struct Cube {
    // Origin assumed to be bottom-left corner
    origin: (usize, usize, usize),
}
impl TryFrom<String> for Cube {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value
            .trim()
            .split(',')
            .map(|c| {
                c.parse::<usize>()
                    .expect("Should be able to parse origin coordinates")
            })
            .collect::<Vec<usize>>();

        let parts = (parts[0], parts[1], parts[2]);
        Ok(Cube { origin: parts })
    }
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
                        p.parse::<usize>()
                            .expect("Should be able to parse coordinate")
                    })
                    .collect::<Vec<usize>>();
                (parts[0], parts[1], parts[2])
            })
            .collect::<Vec<(usize, usize, usize)>>();
        origins.sort()

        // TODO - Loop through origins [i..] and [i+1..] and count how many are off-by-one
    }
}

fn main() {
    Day18 {}.test_and_run();
}

use itertools::Itertools;

static INPUT_FILE_NAME: &str = "data/day-1/input.txt";

fn main() {
    let lines = shared::read_lines(INPUT_FILE_NAME)
        .expect(&format!("Expect file to be present: {}", INPUT_FILE_NAME));
    let groups = shared::segment_lines(lines, |s| s.is_empty());
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
    println!("Max: {:?}", max);
    println!("Max 3: {:?}", max_3);
}

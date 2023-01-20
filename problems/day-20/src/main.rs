use cyclic_list::List;
use shared::{read_lines, AoCProblem, AoCSolution, Solution};

/*
    Basically the goal is to re-arrange a list while iterating through the elements.

    Some observations:
        - Indexes go both forwards and backawards
        - Some are larger than the size of the list in general, meaning they go around
          multiple times, which can be simplified
        - The list is circular, and some instructions are larger than the remaining
          portion, which is isomorphic to a transformation in the other direction
        -

    Let's start with the naive case, work on optimizing
*/

fn showv<'a>(value: &i64, list: &'a List<(usize, i64)>, debug: bool) -> &'a List<(usize, i64)> {
    if !debug {
        return list;
    }
    let to_print = list
        .iter()
        .map(|(_, v)| format!("{}", *v))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{} moves:", value);
    println!("{}\n", to_print);
    list
}

fn part_one(list: &mut List<(usize, i64)>) {
    let mut values = Vec::new();

    let mut cursor = list.cursor_start_mut();

    while !cursor.current().expect("Should init").1.eq(&0) {
        cursor.move_next_cyclic();
    }

    for i in 0..3001 {
        if i == 1000 {
            values.push(cursor.current().expect("Should have value at"));
        } else if i == 2000 {
            values.push(cursor.current().expect("Should have value at"));
        } else if i == 3000 {
            values.push(cursor.current().expect("Should have value at"));
        }
        cursor.move_next_cyclic();
        if cursor.current().is_none() {
            cursor.move_next_cyclic();
        }
    }
    println!(
        "{:#?}, {}",
        values.iter().map(|(_, v)| v).collect::<Vec<_>>(),
        values.iter().map(|(_, v)| v).sum::<i64>()
    );
}

fn mix(lines: Vec<(usize, i64)>, times: usize, debug: bool) -> List<(usize, i64)> {
    // let list_size = lines.len() as i64;

    let mut to_mix = List::from_iter(lines.clone().into_iter());
    showv(&0, &to_mix, debug);

    for _ in 0..times {
        for (original_index, value) in lines.iter() {
            // No need to move  if it's a 0
            if value.eq(&0) {
                continue;
            }

            let mut cursor = to_mix.cursor_start_mut();

            // Advance the cursor until we find the corresponding item
            let mut current = cursor.current().expect("Should exist");
            while !original_index.eq(&current.0) {
                cursor.move_next_cyclic();
                if cursor.current().is_none() {
                    cursor.move_next_cyclic();
                }
                current = cursor.current().expect("Should exist");
            }
            // Remove the item
            let item = cursor.remove().expect("Should have item");

            // Shift the cursor
            if *value < 0 {
                for _ in *value..0 {
                    cursor.move_prev_cyclic();
                    if cursor.current().is_none() {
                        cursor.move_prev_cyclic();
                    }
                }
            } else {
                for _ in 0..*value {
                    cursor.move_next_cyclic();
                    if cursor.current().is_none() {
                        cursor.move_next_cyclic();
                    }
                }
            }

            // Insert the item
            //// If we're at the start move back to the other side of the ghost node
            if cursor.index() == 0 {
                cursor.move_prev_cyclic();
            }
            cursor.insert(item);

            showv(value, &to_mix, debug);
        }
    }

    to_mix
}

struct Day20 {}
impl AoCProblem for Day20 {
    fn name(&self) -> String {
        "day-20".to_string()
    }
}

impl Solution for Day20 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file path")
            .map(|line| {
                line.expect("Should be able to read line")
                    .parse::<i64>()
                    .expect("Should parse")
            })
            .enumerate()
            .collect::<Vec<_>>();

        let mut to_mix = mix(lines, 1, false);

        part_one(&mut to_mix);
    }
}

fn main() {
    Day20 {}.test_and_run()
}

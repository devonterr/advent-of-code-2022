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

fn show<'a>(list: &'a List<&'a (usize, i32)>) -> &'a List<&'a (usize, i32)> {
    let to_print = list
        .iter()
        .map(|(i, v)| format!("{}", *v))
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}", to_print);
    list
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
                    .parse::<i32>()
                    .expect("Should parse")
            })
            .enumerate()
            .collect::<Vec<_>>();

        let list_size = lines.len() as i32;

        let mut to_mix = List::from_iter(lines.iter());
        println!("Original List: {:#?}", to_mix);
        println!("===================================");

        for (original_index, value) in lines.iter() {
            println!(
                "Starting iter: (idx, value) = ({}, {})",
                original_index, value
            );

            // No need to move  if it's a 0
            if value.eq(&0) {
                continue;
            }

            show(&to_mix);
            let mut cursor = to_mix.cursor_start_mut();

            // Advance the cursor until we find the corresponding item
            println!("\tGet current value...");
            let mut current = cursor.current().expect("Should exist");
            println!("\tCurrent Value: {:#?}", current);
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
            cursor.insert(item);

            println!("After Step {:#?}", cursor);
            println!("--------------------------------------------");
        }

        println!("{:#?}", lines);
    }
}

fn main() {
    Day20 {}.test()
}

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

fn mix(lines: Vec<i64>, times: usize, debug: bool) -> List<(usize, i64)> {
    let lines = lines.into_iter().enumerate().collect::<Vec<_>>();
    let list_size = lines.len() as i64;

    let mut to_mix = List::from_iter(lines.clone().into_iter());
    showv(&0, &to_mix, debug);

    for _ in 0..times {
        for (original_index, value) in lines.iter() {
            // Mod by list size
            // Don't mod, just deref
            // let value = *value;

            // Mod
            let value = value % list_size;

            // Via rem???
            // let value = value.rem_euclid(list_size);
            // let value = if value < 0 { list_size + value } else { value };

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
            //// If this puts us ON the ghost node, we need to move forward
            if cursor.current().is_none() {
                cursor.move_next_cyclic();
            }

            // Shift the cursor
            if value < 0 {
                for _ in value..0 {
                    cursor.move_prev_cyclic();
                    if cursor.current().is_none() {
                        cursor.move_prev_cyclic();
                    }
                }
            } else {
                for _ in 0..value {
                    cursor.move_next_cyclic();
                    if cursor.current().is_none() {
                        cursor.move_next_cyclic();
                    }
                }
            }

            // Insert the item
            //// If we're at the start move back to the other side of the ghost node
            // if cursor.index() == 0 {
            //     cursor.move_prev_cyclic();
            // }
            cursor.insert(item);

            showv(&value, &to_mix, debug);
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
            .collect::<Vec<_>>();

        let mut to_mix = mix(lines.clone(), 1, false);

        part_one(&mut to_mix);

        // let decrypted_lines = lines
        //     .iter()
        //     .map(|(i, v)| (i.clone(), v * 811589153))
        //     .collect::<Vec<_>>();

        // let mut to_mix = mix(decrypted_lines, 1, false);
    }
}

fn main() {
    Day20 {}.test_and_run()
    // Day20 {}.test()
}

#[cfg(test)]
mod tests {
    use cyclic_list::List;

    use crate::mix;

    fn match_all(expected: Vec<(usize, i64)>, actual: &mut List<(usize, i64)>) {
        let mut cursor = actual.cursor_start_mut();
        for (index, expected_value) in expected.into_iter() {
            let current = cursor.current().expect("Current should exist");
            assert_eq!(index, current.0, "Expected index");
            assert_eq!(expected_value, current.1, "Expected value");
            cursor.move_next_cyclic();
        }
        assert!(cursor.current().is_none());
    }

    #[test]
    fn mix_testcase() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 4], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (2, -3), (6, 4), (5, 0), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_testcase_plus_cycle() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 11], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (2, -3), (6, 11), (5, 0), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_testcase_plus_2_cycles() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 18], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (2, -3), (6, 18), (5, 0), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_testcase_plus_cycle_plus_one() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 12], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (2, -3), (5, 0), (6, 12), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_testcase_plus_2_cycles_plus_one() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 19], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (2, -3), (5, 0), (6, 19), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_testcase_plus_cycle_minus_one() {
        let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 10], 1, false);
        match_all(
            vec![(0, 1), (1, 2), (6, 10), (2, -3), (5, 0), (3, 3), (4, -2)],
            &mut to_mix,
        );
    }

    #[test]
    fn mix_edge_2() {
        let mut to_mix = mix(vec![0, 1], 1, false);
        match_all(vec![(0, 0), (1, 1)], &mut to_mix);
    }

    #[test]
    fn mix_edge_3() {
        let mut to_mix = mix(vec![0, 0, 1], 1, false);
        match_all(vec![(0, 0), (2, 1), (1, 0)], &mut to_mix);
    }

    // #[test]
    // fn mix_identity_zero() {
    //     let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 7], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (6, 7), (4, -2)],
    //         &mut to_mix,
    //     );
    // }

    // #[test]
    // fn mix_identity() {
    //     let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, -7], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (6, -7), (4, -2)],
    //         &mut to_mix,
    //     );
    // }

    // #[test]
    // fn mix_identity_plus_one() {
    //     let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 8], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (4, -2), (6, 8)],
    //         &mut to_mix,
    //     );
    // }

    // #[test]
    // fn mix_identity_minus_one() {
    //     let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, -6], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -3), (5, 0), (3, 3), (4, -2), (6, -6)],
    //         &mut to_mix,
    //     );
    // }

    // #[test]
    // fn rem_euclid() {
    //     assert_eq!(1i64, 1i64.rem_euclid(10));
    //     assert_eq!(2i64, 2i64.rem_euclid(10));
    //     assert_eq!(3i64, 3i64.rem_euclid(10));
    //     assert_eq!(4i64, 4i64.rem_euclid(10));
    //     assert_eq!(5i64, 5i64.rem_euclid(10));
    //     assert_eq!(6i64, 6i64.rem_euclid(10));
    //     assert_eq!(7i64, 7i64.rem_euclid(10));
    //     assert_eq!(8i64, 8i64.rem_euclid(10));
    //     assert_eq!(9i64, 9i64.rem_euclid(10));
    //     assert_eq!(0i64, 10i64.rem_euclid(10));
    //     assert_eq!(1i64, 11i64.rem_euclid(10));

    //     assert_eq!(0i64, 0i64.rem_euclid(10));
    //     assert_eq!(9i64, 10 + -1i64.rem_euclid(10));
    //     assert_eq!(8i64, 10 + -2i64.rem_euclid(10));
    //     assert_eq!(7i64, 10 + -3i64.rem_euclid(10));
    //     assert_eq!(6i64, 10 + -4i64.rem_euclid(10));
    //     assert_eq!(5i64, 10 + -5i64.rem_euclid(10));
    //     assert_eq!(4i64, 10 + -6i64.rem_euclid(10));
    //     assert_eq!(3i64, 10 + -7i64.rem_euclid(10));
    //     assert_eq!(2i64, 10 + -8i64.rem_euclid(10));
    //     assert_eq!(1i64, 10 + -9i64.rem_euclid(10));
    //     assert_eq!(0i64, -10i64.rem_euclid(10));
    //     assert_eq!(9i64, 10 + -11i64.rem_euclid(10));
    // }

    // #[test]
    // fn mix_overflow_lap() {
    //     let mut to_mix = mix(vec![1, 2, -3, 3, -2, 0, 18], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -3), (6, 18), (5, 0), (3, 3), (4, -2)],
    //         &mut to_mix,
    //     );
    // }

    // #[test]
    // fn mix_underflow_lap() {
    //     let mut to_mix = mix(vec![1, 2, -10, 3, -2, 0, 4], 1, false);
    //     match_all(
    //         vec![(0, 1), (1, 2), (2, -10), (6, 4), (5, 0), (3, 3), (4, -2)],
    //         &mut to_mix,
    //     );
    // }
}

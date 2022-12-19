use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

const SOURCE: (i64, i64) = (500, 0);
type Point = (i64, i64);

fn interpolate(start: Point, to: Point) -> Vec<Point> {
    let diff_x = (to.0 as i64) - (start.0 as i64);
    let diff_y = (to.1 as i64) - (start.1 as i64);
    let delta_x = match diff_x {
        0 => 0,
        x => x / x.abs(),
    };
    let delta_y = match diff_y {
        0 => 0,
        y => y / y.abs(),
    };
    let mut result = vec![start];
    let mut next = start;
    while next != to {
        next.0 += delta_x;
        next.1 += delta_y;
        result.push(next);
    }
    result
}

fn settle(lower_bound: i64, occupied: &HashSet<Point>, sand: Point) -> Option<Point> {
    let one_down = (sand.0, sand.1 + 1);
    let down_left = (sand.0 - 1, sand.1 + 1);
    let down_right = (sand.0 + 1, sand.1 + 1);
    if one_down.1 > lower_bound {
        None
    } else if !occupied.contains(&one_down) {
        settle(lower_bound, occupied, one_down)
    } else if !occupied.contains(&down_left) {
        settle(lower_bound, occupied, down_left)
    } else if !occupied.contains(&down_right) {
        settle(lower_bound, occupied, down_right)
    } else {
        Some(sand)
    }
}

fn settle2(lower_bound: i64, occupied: &HashSet<Point>, sand: Point) -> Option<Point> {
    let one_down = (sand.0, sand.1 + 1);
    let down_left = (sand.0 - 1, sand.1 + 1);
    let down_right = (sand.0 + 1, sand.1 + 1);
    if one_down.1 == lower_bound {
        Some(sand)
    } else if !occupied.contains(&one_down) {
        settle2(lower_bound, occupied, one_down)
    } else if !occupied.contains(&down_left) {
        settle2(lower_bound, occupied, down_left)
    } else if !occupied.contains(&down_right) {
        settle2(lower_bound, occupied, down_right)
    } else {
        Some(sand)
    }
}

struct Day14 {}
impl AoCProblem for Day14 {
    fn name(&self) -> String {
        "day-14".to_owned()
    }
}
impl Solution for Day14 {
    fn solution(&self, path: &str) {
        {
            let mut occupied_points = read_lines(path)
                .expect("Should be able to read")
                .map(|line| line.expect("Should be able to read lines"))
                .map(|line| {
                    line.clone()
                        .split(" -> ")
                        .map(|p| {
                            let parts = p.split_once(',').expect("Should have one comma");
                            let x = parts
                                .0
                                .parse::<i64>()
                                .expect("Should be able to parse x coord");
                            let y = parts
                                .1
                                .parse::<i64>()
                                .expect("Should be able to parse y coord");
                            (x, y)
                        })
                        .collect::<Vec<Point>>()
                })
                .flat_map(|path_segments| {
                    path_segments
                        .windows(2)
                        .flat_map(|segment_pair| interpolate(segment_pair[0], segment_pair[1]))
                        .collect::<Vec<Point>>()
                })
                .collect::<HashSet<Point>>();

            let lower_bound = occupied_points
                .iter()
                .map(|kv| kv.1)
                .max()
                .expect("Should have a max value");

            let mut counter = 0;
            loop {
                let res = settle(lower_bound, &occupied_points, (500, 0));
                if res.is_none() {
                    break;
                } else {
                    occupied_points.insert(res.unwrap());
                }
                counter += 1;
            }
            println!("Part one: {}", counter);
        }
        {
            let mut occupied_points = read_lines(path)
                .expect("Should be able to read")
                .map(|line| line.expect("Should be able to read lines"))
                .map(|line| {
                    line.clone()
                        .split(" -> ")
                        .map(|p| {
                            let parts = p.split_once(',').expect("Should have one comma");
                            let x = parts
                                .0
                                .parse::<i64>()
                                .expect("Should be able to parse x coord");
                            let y = parts
                                .1
                                .parse::<i64>()
                                .expect("Should be able to parse y coord");
                            (x, y)
                        })
                        .collect::<Vec<Point>>()
                })
                .flat_map(|path_segments| {
                    path_segments
                        .windows(2)
                        .flat_map(|segment_pair| interpolate(segment_pair[0], segment_pair[1]))
                        .collect::<Vec<Point>>()
                })
                .collect::<HashSet<Point>>();

            let lower_bound = occupied_points
                .iter()
                .map(|kv| kv.1)
                .max()
                .expect("Should have a max value")+2;

            let mut counter = 0;
            loop {
                let res = settle2(lower_bound, &occupied_points, (500, 0));
                match res {
                    Some(p) => {
                        occupied_points.insert(p);
                        if (500, 0) == p {
                            counter += 1;
                            break;
                        }
                    },
                    _ => {
                        println!("Shouldn't happen");
                        break;
                    }
                }
                counter += 1;
            }
            println!("Part two: {}", counter);
        }
    }
}

fn main() {
    Day14 {}.test_and_run();
}

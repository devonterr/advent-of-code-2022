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

fn settle<F>(
    lower_bound: i64,
    occupied: &HashSet<Point>,
    sand: Point,
    handle_bounds: F,
) -> Option<Point>
where
    F: Fn(Point) -> Option<Point>,
{
    let mut next_node = sand;
    loop {
        let one_down = (next_node.0, next_node.1 + 1);
        let down_left = (next_node.0 - 1, next_node.1 + 1);
        let down_right = (next_node.0 + 1, next_node.1 + 1);

        if one_down.1 > lower_bound {
            return handle_bounds(next_node);
        } else if !occupied.contains(&one_down) {
            next_node = one_down;
        } else if !occupied.contains(&down_left) {
            next_node = down_left;
        } else if !occupied.contains(&down_right) {
            next_node = down_right;
        } else {
            return Some(next_node)
        }
    }
}

fn part_solution<F, G>(prefix: String, path: &str, bounds_finder: F, bounds_handler: G)
where
    F: Fn(i64) -> i64,
    G: Fn(Point) -> Option<Point>,
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

    let lower_bound = bounds_finder(
        occupied_points
            .iter()
            .map(|kv| kv.1)
            .max()
            .expect("Should have a max value"),
    );

    let mut counter = 0;
    loop {
        let res = settle(lower_bound, &occupied_points, SOURCE, &bounds_handler);
        // If settle returns None, then we've gone off the deep end
        // (Part 1)
        if res.is_none() {
            break;
        } else {
            let inner = res.unwrap();
            occupied_points.insert(inner);
            // If settle returns SOURCE then that means we've backed up
            // to the source point. (Part 2)
            if SOURCE == inner {
                counter += 1;
                break;
            }
        }
        counter += 1;
    }
    println!("Part {}: {}", prefix, counter);
}

struct Day14 {}
impl AoCProblem for Day14 {
    fn name(&self) -> String {
        "day-14".to_owned()
    }
}
impl Solution for Day14 {
    fn solution(&self, path: &str) {
        part_solution("one".to_owned(), path, |i| i, |_point| None);
        part_solution("two".to_owned(), path, |i| i + 1, |point| Some(point));
    }
}

fn main() {
    Day14 {}.test_and_run();
}

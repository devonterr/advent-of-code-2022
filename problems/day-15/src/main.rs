use std::{collections::{HashSet, BTreeSet}, cmp::Ordering};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

struct Interval {
    lo: i64,
    hi: i64,
}
impl Interval {}
struct IntervalSet {
    intervals: Vec<Interval>,
}

// Returns Center, Distance, and Beacon
fn line_to_points(line: &str) -> (Point, usize, Point) {
    let (x0, rest) = line
        .trim_start_matches("Sensor at x=")
        .split_once(',')
        .expect("Should follow format");
    let (y0, rest) = rest
        .trim_start_matches(" y=")
        .split_once(':')
        .expect("Shuld follow format 2");
    let (x1, rest) = rest
        .trim_start_matches(" closest beacon is at x=")
        .split_once(',')
        .expect("Should follow format 3");
    let (_, y1) = rest.split_once('=').expect("Should follow format 4");

    let start = Point {
        x: x0.parse::<i64>().expect("Should be able to parse x0"),
        y: y0.parse::<i64>().expect("Should be able to parse y0"),
    };
    let beacon = Point {
        x: x1.parse::<i64>().expect("Should be able to parse x1"),
        y: y1.parse::<i64>().expect("Should be able to parse y1"),
    };
    (start, start.distance(&beacon), beacon)
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn distance(&self, to: &Point) -> usize {
        let x_distance = (to.x - self.x).abs();
        let y_distance = (to.y - self.y).abs();
        (x_distance + y_distance) as usize
    }

    fn line(&self, distance: usize, on_line: i64) -> (i64, Interval<i64>) {
        let y = on_line;
        let remaining_distance = (distance as i64) - (on_line - self.y).abs();
        (y, Interval::new(self.x - remaining_distance, self.x + remaining_distance))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_computeDistance() {
        let p = Point { x: 8, y: 7 };
        let b = Point { x: 2, y: 10 };
        let result = p.distance(&b);
        assert_eq!(result, 9, "Distance should be 9");
    }

    // #[test]
    // fn should_computeLine() {
    //     let a = Point { x: 8, y: 7 };
    //     let d = 1;
    //     let result = a.line(d, 7);
    //     assert_eq!(result.len(), 3);

    //     let a = Point { x: 8, y: 7 };
    //     let d = 2;
    //     let result = a.line(d, 8);
    //     assert_eq!(result.len(), 3);
    // }
}

struct Day15 {}
impl AoCProblem for Day15 {
    fn name(&self) -> String {
        "day-15".to_owned()
    }
}
impl Solution for Day15 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"))
            .collect::<Vec<String>>();

        let on_line = if lines.len() > 20 { 2000000 } else { 10 };

        {
            let circles_and_beacons = lines
                .iter()
                .map(|line| line_to_points(line))
                .collect::<Vec<(Point, usize, Point)>>();
            let mut beacons = HashSet::new();
            let mut covered_intervals = vec![];
            for (center, distance, beacon) in circles_and_beacons {
                let covered_line = center.line(distance, on_line);
                covered_intervals.push(covered_line.1);
                beacons.insert(beacon);
            }
            let all_covered = -(beacons.len() as i64);
            let mut interval_set = vec![].to_interval_set();
            interval_set.extend(covered_intervals);
            // for interval in interval_set.intervals {
            //     all_covered += interval.hi - interval.lo;
            // }
            for interval in interval_set {
            }
            println!("Part 1: {}", all_covered);
        }

    //     {
    //         let circles_and_beacons = lines
    //             .iter()
    //             .map(|line| line_to_points(line))
    //             .collect::<Vec<(Point, usize, Point)>>();
    //         let max_bound = 1 + if lines.len() > 20 { 4000000 } else { 20 };
    //         for x in 0..max_bound {
    //             for y in 0..max_bound {
    //                 let candidate = Point{x, y};
    //                 let mut any_cover = false;
    //                 for (center, distance, _beacon) in &circles_and_beacons[0..] {
    //                     if center.distance(&candidate) <= *distance {
    //                         any_cover = true;
    //                         break;
    //                     }
    //                 }
    //                 if !any_cover {
    //                     println!("Part 2: {},{} - {}", x, y, (4000000 * x) + y );
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    }
}

fn main() {
    Day15 {}.test_and_run();
    // Day15 {}.test();
    // Day15 {}.run();
}

use std::{
    collections::{BTreeSet, HashSet},
};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Interval {
    lo: i64,
    hi: i64,
}
impl Interval {
    fn overlaps(&self, other: &Interval) -> bool {
        (self.lo <= other.lo && self.hi >= other.lo)
            || (self.lo <= other.hi && self.hi >= other.hi)
            || (other.lo <= self.lo && other.hi >= self.lo)
            || (other.lo <= self.hi && other.hi >= self.hi)
    }

    fn merge(&mut self, other: &Interval) {
        self.hi = self.hi.max(other.hi);
        self.lo = self.lo.min(other.lo);
    }
    
    fn size(&self) -> i64 {
        1 + (self.hi - self.lo).abs()
    }
}
#[derive(Debug)]
struct IntervalSet {
    intervals: BTreeSet<Interval>,
}
impl IntervalSet {
    fn new() -> IntervalSet {
        IntervalSet{intervals: BTreeSet::new()}
    }
    fn add(&mut self, mut interval: Interval) {
        let (overlaps, independent): (Vec<Interval>, Vec<Interval>) =
            self.intervals.iter().partition(|c| c.overlaps(&interval));
        for i in overlaps {
            interval.merge(&i);
        }
        let mut result: BTreeSet<Interval> = BTreeSet::new();
        result.insert(interval.to_owned());
        result.extend(&independent);
        self.intervals = result;
    }
    
    fn size(&self) -> i64 {
        self.intervals.iter().map(|i| i.size()).sum::<i64>()
    }
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

    fn line(&self, distance: usize, on_line: i64) -> (i64, Interval) {
        let y = on_line;
        let remaining_distance = (distance as i64) - (on_line - self.y).abs();
        let v1 = self.x - remaining_distance;
        let v2 = self.x + remaining_distance;
        (
            y,
            Interval{lo: v1.min(v2), hi: v1.max(v2)},
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_distance() {
        let p = Point { x: 8, y: 7 };
        let b = Point { x: 2, y: 10 };
        let result = p.distance(&b);
        assert_eq!(result, 9, "Distance should be 9");
    }

    #[test]
    fn should_compute_line() {
        let a = Point { x: 8, y: 7 };
        let d = 1;
        let line = a.y;
        let (y, interval) = a.line(d, line);
        assert_eq!(y, line);
        assert_eq!(interval.size(), 3);

        let a = Point { x: 8, y: 7 };
        let d = 2;
        let line = a.y + 1;
        let (y, interval) = a.line(d, line);
        assert_eq!(y, line);
        assert_eq!(interval.size(), 3);
    }
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

        let circles_and_beacons = lines
            .iter()
            .map(|line| line_to_points(line))
            .collect::<Vec<(Point, usize, Point)>>();
        let mut beacons = HashSet::new();
        let mut interval_set = IntervalSet::new();
        for (center, distance, beacon) in circles_and_beacons.clone() {
            let (_y, interval) = center.line(distance, on_line);
            interval_set.add(interval);
            beacons.insert(beacon);
        }
        let all_covered = interval_set.size();
        println!("Part 1: {}", all_covered);
            
        // let max_bound = 1 + if lines.len() > 20 { 4000000 } else { 20 };
        // for on_line in 0..max_bound {
        //     let mut interval_set = IntervalSet::new();
        //     for (center, distance, beacon) in circles_and_beacons.clone() {
        //         let (_y, interval) = center.line(distance, on_line);
        //         interval_set.add(interval);
        //     }
        //     if on_line == 11 {
        //         println!("{} - {:#?}", on_line, interval_set);
        //     }
        // }

    }
}

fn main() {
    Day15 {}.test_and_run();
    // Day15 {}.test();
    // Day15 {}.run();
}

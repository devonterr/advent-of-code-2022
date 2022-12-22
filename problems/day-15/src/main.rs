use std::collections::{BTreeSet, HashSet};

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

    fn adjacent(&self, other: &Interval) -> bool {
        (self.hi + 1 == other.lo)
            || (self.lo == other.hi + 1)
            || (other.hi + 1 == self.lo)
            || (other.lo == self.hi + 1)
    }

    fn merge(&mut self, other: &Interval) {
        self.hi = self.hi.max(other.hi);
        self.lo = self.lo.min(other.lo);
    }

    fn size(&self) -> i64 {
        1 + (self.hi - self.lo).abs()
    }

    fn contains(&self, value: i64) -> bool {
        self.lo <= value && value <= self.hi
    }

    fn split(&self, value: i64) -> Vec<Interval> {
        if self.contains(value) {
            vec![
                Interval {
                    lo: self.lo,
                    hi: value - 1,
                },
                Interval {
                    lo: value + 1,
                    hi: self.hi,
                },
            ]
        } else {
            vec![self.to_owned()]
        }
    }
}
#[derive(Debug)]
struct IntervalSet {
    on_line: i64,
    intervals: BTreeSet<Interval>,
}
impl IntervalSet {
    fn new(on_line: i64) -> IntervalSet {
        IntervalSet {
            on_line,
            intervals: BTreeSet::new(),
        }
    }
    fn add(&mut self, mut interval: Interval) {
        let (overlaps, independent): (Vec<Interval>, Vec<Interval>) = self
            .intervals
            .iter()
            .partition(|c| c.overlaps(&interval) || c.adjacent(&interval));
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

    fn contains(&self, point: &Point) -> bool {
        if self.on_line != point.y {
            return false;
        }
        for interval in self.intervals.iter() {
            if interval.lo <= point.x && interval.hi >= point.x {
                return true;
            }
        }
        false
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
        let result = (x_distance + y_distance) as usize;
        result
    }

    fn line(&self, distance: usize, on_line: i64, beacon: &Point) -> Vec<(i64, Interval)> {
        let y = on_line;
        let remaining_distance = (distance as i64) - (on_line - self.y).abs();
        if remaining_distance < 0 {
            return vec![];
        }
        let v1 = self.x - remaining_distance;
        let v2 = self.x + remaining_distance;
        let candidate_interval = Interval {
            lo: v1.min(v2),
            hi: v1.max(v2),
        };
        if beacon.y == on_line && candidate_interval.contains(beacon.x) {
            candidate_interval
                .split(beacon.x)
                .iter()
                .map(|i| (y, i.to_owned()))
                .collect::<Vec<(i64, Interval)>>()
        } else {
            vec![(y, candidate_interval)]
        }
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
        let beacon = Point { x: 8, y: 20 };
        let a = Point { x: 8, y: 7 };
        let d = 1;
        let line = a.y;
        let intervals = a.line(d, line, &beacon);
        assert_eq!(intervals.len(), 1);
        let (y, interval) = intervals[0];
        assert_eq!(y, line);
        assert_eq!(interval.size(), 3);
        assert_eq!(interval.lo, 7);
        assert_eq!(interval.hi, 9);

        let a = Point { x: 8, y: 7 };
        let d = 2;
        let line = a.y + 1;
        let intervals = a.line(d, line, &beacon);
        assert_eq!(intervals.len(), 1);
        let (y, interval) = intervals[0];
        assert_eq!(y, line);
        assert_eq!(interval.size(), 3);
        assert_eq!(interval.lo, 7);
        assert_eq!(interval.hi, 9);

        let a = Point { x: 8, y: 7 };
        let d = 3;
        let line = a.y + 1;
        let intervals = a.line(d, line, &beacon);
        assert_eq!(intervals.len(), 1);
        let (y, interval) = intervals[0];
        assert_eq!(y, line);
        assert_eq!(interval.size(), 5);
        assert_eq!(interval.lo, 6);
        assert_eq!(interval.hi, 10);
    }

    #[test]
    fn should_compute_split_line() {
        let beacon = Point { x: 8, y: 8 };
        let a = Point { x: 8, y: 7 };
        let d = 3;
        let line = a.y + 1;
        let intervals = a.line(d, line, &beacon);
        assert_eq!(intervals.len(), 2);
    }

    #[test]
    fn should_combine_intervals() {
        let mut s = IntervalSet::new(0);
        s.add(Interval { lo: 0, hi: 10 });
        s.add(Interval { lo: 2, hi: 12 });
        s.add(Interval { lo: 13, hi: 14 });

        assert!(s.contains(&Point { y: 0, x: 0 }));
        assert!(s.contains(&Point { y: 0, x: 14 }));
        assert_eq!(s.intervals.len(), 1);
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
        let mut interval_set = IntervalSet::new(on_line);
        for (center, distance, beacon) in circles_and_beacons.clone() {
            let intervals = center.line(distance, on_line, &beacon);
            for (_y, interval) in intervals {
                interval_set.add(interval);
                beacons.insert(beacon);
            }
        }
        let all_covered = interval_set.size();
        println!("Part 1: {}", all_covered);

        let max_bound = 1 + if lines.len() > 20 { 4000000 } else { 20 };
        for on_line in 0..max_bound {
            let mut interval_set = IntervalSet::new(on_line);
            for (center, distance, beacon) in circles_and_beacons.clone() {
                let intervals = center.line(distance, on_line, &beacon);
                for (_y, interval) in intervals {
                    interval_set.add(interval);
                }
            }
            for beacon in beacons.clone() {
                if beacon.y == on_line {
                    interval_set.add(Interval {
                        lo: beacon.x,
                        hi: beacon.x,
                    })
                }
            }
            if interval_set.intervals.len() > 1 {
                let x_val = 1 + interval_set
                                .intervals
                                .iter()
                                .map(|i| i.to_owned())
                                .collect::<Vec<Interval>>()[0]
                                .hi;
                if x_val < 0 || x_val > max_bound {
                    continue;
                }
                println!(
                    "Part 2: {:#?}\n\n{}\n\n\n",
                    interval_set,
                    interval_set.on_line
                        + (4000000
                            * x_val)
                );
            }
        }
    }
}

fn main() {
    Day15 {}.test_and_run();
}

use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

// TODO
// Let's try a different strategy
// Instead of instantiating points, on a line or no, lets construct
// all of the circle centers, distances, and a hashset of beacons.
// The solutions will iterate over candidate points and check distances

fn line_to_points(line: &str, on_line: i64) -> (HashSet<Point>, Point) {
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
    let till = Point {
        x: x1.parse::<i64>().expect("Should be able to parse x1"),
        y: y1.parse::<i64>().expect("Should be able to parse y1"),
    };
    (
        start.circle_part(start.distance(till.clone()), on_line),
        till,
    )
}

fn count_covered_points(points: &HashSet<Point>, beacons: &HashSet<Point>) -> usize {
    points.difference(&beacons).into_iter().count()
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn distance(&self, to: Point) -> usize {
        let x_distance = (to.x - self.x).abs();
        let y_distance = (to.y - self.y).abs();
        (x_distance + y_distance) as usize
    }

    fn circle_part(&self, distance: usize, line: i64) -> HashSet<Point> {
        let distance = distance as i64;
        // Check if line is within distance of point
        let is_line_within_distance = (line - self.y).abs() <= distance;
        // If not, return empty set
        if !is_line_within_distance {
            HashSet::new()
        } else {
            // If so, iterate through all options that include that line
            let mut result: Vec<Point> = vec![];
            let y = line;
            let remaining_distance = (distance as i64) - (line - self.y).abs();
            for x in (-remaining_distance)..(remaining_distance + 1) {
                let new_point = Point {
                    x: self.x + x,
                    y,
                };
                result.push(new_point);
            }
            HashSet::from_iter(result)
        }
    }

    fn circle(&self, distance: usize) -> HashSet<Point>
    {
        let distance = distance as i64;
        let mut result: Vec<Point> = vec![];
        for x in (-distance)..(distance + 1) {
            let remaining_distance = (distance as i64) - x.abs();
            for y in (-remaining_distance)..(remaining_distance + 1) {
                result.push(Point {
                    x: self.x + x,
                    y: self.y + y,
                })
            }
        }
        HashSet::from_iter(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_computeDistance() {
        let p = Point { x: 8, y: 7 };
        let b = Point { x: 2, y: 10 };
        let result = p.distance(b);
        assert_eq!(result, 9, "Distance should be 9");
    }

    #[test]
    fn should_computeCircle() {
        let p = Point { x: 8, y: 7 };
        let distance = 9;
        let circle = p.circle(distance);

        assert!(circle.contains(&Point { x: -1, y: 7 }));
        assert!(circle.contains(&Point { x: 17, y: 7 }));
        assert!(circle.contains(&Point { x: 8, y: -1 }));
        assert!(circle.contains(&Point { x: 8, y: 16 }));

        assert_eq!(circle.len(), 181, "Should have 221 elements");
    }

    #[test]
    fn should_computePartialCircle() {
        let p = Point { x: 8, y: 7 };
        let distance = 3;
        let circle = p.circle_part(distance, 9);

        assert_eq!(circle.len(), 3);
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
        // {
        //     let mut covered_points = HashSet::new();
        //     let mut beacons = HashSet::new();

        //     for (cp, b) in lines.iter().map(|line| line_to_points(line, 10)) {
        //         covered_points.extend(cp);
        //         beacons.insert(b);
        //     }
        //     let non_beacon_points_at_10 = count_covered_points(&covered_points, &beacons);
        //     println!("Part 1 - test: {}", non_beacon_points_at_10);

        //     // Part 2
        //     let mut covered_points = HashSet::new();
        //     let mut beacons = HashSet::new();

        //     //// Populate occupied positions and beacons
        //     for to_check in 0..21 {
        //         for (cp, b) in lines.iter().map(|line| line_to_points(line, to_check)) {
        //             covered_points.extend(cp);
        //             beacons.insert(b);
        //         }
        //     }

        //     // Check every point in the range
        //     for x in 0..21 {
        //         for y in 0..21 {
        //             let candidate = Point{x, y};
        //             if !covered_points.contains(&candidate) && !beacons.contains(&candidate) {
        //                 println!("Part 2: {:#?} - {}", candidate, (4000000 * candidate.x) + candidate.y);
        //                 return;
        //             }
        //         }
        //     }
        // }

        {
            // let mut covered_points = HashSet::new();
            // let mut beacons = HashSet::new();

            // for (cp, b) in lines.iter().map(|line| line_to_points(line, 2000000)) {
            //     covered_points.extend(cp);
            //     beacons.insert(b);
            // }
            // let non_beacon_points_at_2000000 = count_covered_points(&covered_points, &beacons);
            // println!("Part 1 - run: {}", non_beacon_points_at_2000000);

            // Part 2
            let mut covered_points = HashSet::new();
            let mut beacons = HashSet::new();

            //// Populate occupied positions and beacons
            for to_check in 0..4000000 {
                for (cp, b) in lines.iter().map(|line| line_to_points(line, to_check)) {
                    covered_points.extend(cp);
                    beacons.insert(b);
                }
            }

            // Check every point in the range
            for x in 0..4000000 {
                for y in 0..4000000 {
                    let candidate = Point{x, y};
                    if !covered_points.contains(&candidate) && !beacons.contains(&candidate) {
                        println!("Part 2: {:#?} - {}", candidate, (4000000 * candidate.x) + candidate.y);
                        return;
                    }
                }
            }
        }
    }
}

fn main() {
    // Day15 {}.test_and_run();
    // Day15 {}.test();
    Day15 {}.run();
}

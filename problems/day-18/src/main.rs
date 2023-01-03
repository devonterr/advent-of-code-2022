use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn parse_line(line: String) -> Coordinate {
    let parts = line
        .trim()
        .split(',')
        .map(|p| {
            // Shifting everything by two because it makes it easier to deal with usize and underflow.
            // Shifting one for neighbor calculation, and another one for producing a shell with valid neighbors
            2 + p
                .parse::<usize>()
                .expect("Should be able to parse coordinate")
        })
        .collect::<Vec<usize>>();
    (parts[0], parts[1], parts[2])
}

fn neighbors(origin: &Coordinate) -> Vec<Coordinate> {
    vec![
        (origin.0 + 1, origin.1, origin.2),
        (origin.0, origin.1 + 1, origin.2),
        (origin.0, origin.1, origin.2 + 1),
        (origin.0 - 1, origin.1, origin.2),
        (origin.0, origin.1 - 1, origin.2),
        (origin.0, origin.1, origin.2 - 1),
    ]
}

type Coordinate = (usize, usize, usize);

fn exposed_faces(origins: &HashSet<Coordinate>) -> Vec<(Coordinate, Coordinate)> {
    origins
        .iter()
        .flat_map(|o| {
            neighbors(o)
                .into_iter()
                .filter(|n| !origins.contains(n))
                .map(|n| (*o, n))
        })
        .collect::<Vec<_>>()
}

fn bounds(origins: Vec<Coordinate>) -> (usize, usize, usize, usize, usize, usize) {
    let min_x = origins
        .iter()
        .map(|o| o.0)
        .min()
        .expect("Should have min x");
    let min_y = origins
        .iter()
        .map(|o| o.1)
        .min()
        .expect("Should have min x");
    let min_z = origins
        .iter()
        .map(|o| o.2)
        .min()
        .expect("Should have min x");

    let max_x = origins
        .iter()
        .map(|o| o.0)
        .max()
        .expect("Should have max x");
    let max_y = origins
        .iter()
        .map(|o| o.1)
        .max()
        .expect("Should have max x");
    let max_z = origins
        .iter()
        .map(|o| o.2)
        .max()
        .expect("Should have max x");

    (min_x, min_y, min_z, max_x, max_y, max_z)
}

fn shell_2d(min_0: usize, min_1: usize, max_0: usize, max_1: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for c0 in min_0 - 1..max_0 + 2 {
        for c1 in min_1 - 1..max_1 + 2 {
            res.push((c0, c1));
        }
    }
    res
}

fn shell(origins: &HashSet<Coordinate>) -> Vec<Coordinate> {
    let (min_x, min_y, min_z, max_x, max_y, max_z) =
        bounds(origins.clone().into_iter().collect::<Vec<_>>());
    vec![
        shell_2d(min_x, min_y, max_x, max_y)
            .into_iter()
            .map(|o| (o.0, o.1, min_z - 1))
            .collect::<Vec<Coordinate>>(),
        shell_2d(min_x, min_z, max_x, max_z)
            .into_iter()
            .map(|o| (o.0, min_y - 1, o.1))
            .collect::<Vec<Coordinate>>(),
        shell_2d(min_y, min_z, max_y, max_z)
            .into_iter()
            .map(|o| (min_x - 1, o.0, o.1))
            .collect::<Vec<Coordinate>>(),
        shell_2d(min_x, min_y, max_x, max_y)
            .into_iter()
            .map(|o| (o.0, o.1, max_z + 1))
            .collect::<Vec<Coordinate>>(),
        shell_2d(min_x, min_z, max_x, max_z)
            .into_iter()
            .map(|o| (o.0, max_y + 1, o.1))
            .collect::<Vec<Coordinate>>(),
        shell_2d(min_y, min_z, max_y, max_z)
            .into_iter()
            .map(|o| (max_x + 1, o.0, o.1))
            .collect::<Vec<Coordinate>>(),
    ]
    .into_iter()
    .flatten()
    .collect::<HashSet<Coordinate>>()
    .into_iter()
    .collect::<Vec<Coordinate>>()
}

fn flood(shell: Vec<Coordinate>, origins: HashSet<Coordinate>) -> HashSet<Coordinate> {
    let (minx, miny, minz, maxx, maxy, maxz) = bounds(shell.clone());
    let mut res = HashSet::new();
    res.extend(shell.clone());
    res.extend(origins.iter());
    let mut candidates = shell
        .iter()
        .flat_map(neighbors)
        .filter(|c| {
            c.0 < maxx
                && c.0 > minx
                && c.1 < maxy
                && c.1 > miny
                && c.2 < maxz
                && c.2 > minz
                && !res.contains(c)
        })
        .collect::<Vec<_>>();
    loop {
        if candidates.is_empty() {
            break;
        }
        res.extend(&candidates);
        let new_candidates = candidates
            .iter()
            .flat_map(neighbors)
            .filter(|c| !res.contains(c))
            .collect::<Vec<_>>();
        candidates = new_candidates;
    }
    res.into_iter()
        .filter(|c| !origins.contains(c))
        .collect::<HashSet<_>>()
}

struct Day18 {}
impl AoCProblem for Day18 {
    fn name(&self) -> String {
        "day-18".to_owned()
    }
}
impl Solution for Day18 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|l| l.expect("Should be able to read line"));

        let origins = lines.map(parse_line).collect::<HashSet<Coordinate>>();

        let exposed_faces = exposed_faces(&origins);

        println!("Part one: {}", exposed_faces.len());

        let shell = shell(&origins);
        let flooded = flood(shell, origins);
        let exterior_exposed_faces = exposed_faces
            .iter()
            .filter(|(_origin, face)| flooded.contains(face))
            .collect::<Vec<_>>();
        println!("Part two: {}", exterior_exposed_faces.len());
    }
}

fn main() {
    Day18 {}.test_and_run();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn exposed_faces_should() {
        let mut input = HashSet::new();
        input.insert((1, 1, 1));
        input.insert((2, 1, 1));
        input.insert((1, 1, 2));

        let res = exposed_faces(&input);
        assert_eq!(14, res.len());

        input.insert((2, 1, 2));
        let res = exposed_faces(&input);
        assert_eq!(16, res.len());
    }

    #[test]
    fn parse_line_should_inc_values() {
        let input = "1,1,0";
        let res = parse_line(input.to_owned());
        assert_eq!(res.0, 3);
        assert_eq!(res.1, 3);
        assert_eq!(res.2, 2);
    }

    #[test]
    fn shell_should_make_face() {
        let res = shell_2d(2, 2, 2, 2);
        assert_eq!(9, res.len());
        assert!(res.contains(&(1, 1)));
        assert!(res.contains(&(1, 2)));
        assert!(res.contains(&(1, 3)));
        assert!(res.contains(&(2, 1)));
        assert!(res.contains(&(2, 2)));
        assert!(res.contains(&(2, 3)));
        assert!(res.contains(&(3, 1)));
        assert!(res.contains(&(3, 2)));
        assert!(res.contains(&(3, 3)));

        let res = shell_2d(2, 2, 3, 3);
        assert_eq!(16, res.len());

        let res = shell_2d(2, 2, 3, 2);
        assert_eq!(12, res.len());
    }

    #[test]
    fn shell_should_encase() {
        let mut input = HashSet::new();
        input.insert((2, 2, 2));
        let res = shell(&input);

        assert_eq!(26, res.len());

        assert!(res.contains(&(1, 1, 1)));
        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(1, 1, 3)));
        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(1, 2, 2)));
        assert!(res.contains(&(1, 2, 3)));
        assert!(res.contains(&(1, 3, 1)));
        assert!(res.contains(&(1, 3, 2)));
        assert!(res.contains(&(1, 3, 3)));

        assert!(res.contains(&(1, 1, 1)));
        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(1, 1, 3)));
        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 1, 2)));
        assert!(res.contains(&(2, 1, 3)));
        assert!(res.contains(&(3, 1, 1)));
        assert!(res.contains(&(3, 1, 2)));
        assert!(res.contains(&(3, 1, 3)));

        assert!(res.contains(&(1, 1, 1)));
        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(1, 3, 1)));
        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 2, 1)));
        assert!(res.contains(&(2, 3, 1)));
        assert!(res.contains(&(3, 1, 1)));
        assert!(res.contains(&(3, 2, 1)));
        assert!(res.contains(&(3, 3, 1)));

        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 1, 2)));
        assert!(res.contains(&(2, 1, 3)));
        assert!(res.contains(&(2, 2, 1)));
        assert!(res.contains(&(2, 2, 3)));
        assert!(res.contains(&(2, 3, 1)));
        assert!(res.contains(&(2, 3, 2)));
        assert!(res.contains(&(2, 3, 3)));

        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(1, 2, 2)));
        assert!(res.contains(&(1, 2, 3)));
        assert!(res.contains(&(2, 2, 1)));
        assert!(res.contains(&(2, 2, 3)));
        assert!(res.contains(&(3, 2, 1)));
        assert!(res.contains(&(3, 2, 2)));
        assert!(res.contains(&(3, 2, 3)));

        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(1, 2, 2)));
        assert!(res.contains(&(1, 3, 2)));
        assert!(res.contains(&(2, 1, 2)));
        assert!(res.contains(&(2, 3, 2)));
        assert!(res.contains(&(3, 1, 2)));
        assert!(res.contains(&(3, 2, 2)));
        assert!(res.contains(&(3, 3, 2)));

        assert!(res.contains(&(3, 1, 1)));
        assert!(res.contains(&(3, 1, 2)));
        assert!(res.contains(&(3, 1, 3)));
        assert!(res.contains(&(3, 2, 1)));
        assert!(res.contains(&(3, 2, 2)));
        assert!(res.contains(&(3, 2, 3)));
        assert!(res.contains(&(3, 3, 1)));
        assert!(res.contains(&(3, 3, 2)));
        assert!(res.contains(&(3, 3, 3)));

        assert!(res.contains(&(1, 3, 1)));
        assert!(res.contains(&(1, 3, 2)));
        assert!(res.contains(&(1, 3, 3)));
        assert!(res.contains(&(2, 3, 1)));
        assert!(res.contains(&(2, 3, 2)));
        assert!(res.contains(&(2, 3, 3)));
        assert!(res.contains(&(3, 3, 1)));
        assert!(res.contains(&(3, 3, 2)));
        assert!(res.contains(&(3, 3, 3)));

        assert!(res.contains(&(1, 1, 3)));
        assert!(res.contains(&(1, 2, 3)));
        assert!(res.contains(&(1, 3, 3)));
        assert!(res.contains(&(2, 1, 3)));
        assert!(res.contains(&(2, 2, 3)));
        assert!(res.contains(&(2, 3, 3)));
        assert!(res.contains(&(3, 1, 3)));
        assert!(res.contains(&(3, 2, 3)));
        assert!(res.contains(&(3, 3, 3)));

        let mut input = HashSet::new();
        input.insert((2, 2, 2));
        input.insert((3, 2, 2));
        let res = shell(&input);

        assert_eq!(34, res.len());

        let mut input = HashSet::new();
        input.insert((2, 2, 2));
        input.insert((3, 2, 2));
        input.insert((2, 2, 3));
        let res = shell(&input);

        // Missing 3 for the actual blocks, plus one pocket because of the L-shaped configuration
        assert_eq!(44, res.len());
    }

    #[test]
    fn flood_should_fill_l() {
        let mut input = HashSet::new();
        input.insert((2, 2, 2));
        input.insert((3, 2, 2));
        input.insert((2, 2, 3));
        let outer_shell = shell(&input);
        let flooded = flood(outer_shell, input);
        assert_eq!(45, flooded.len());
    }
}

use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

fn parse_line(line: String) -> (i32, i32, i32) {
    let parts = line
        .trim()
        .split(',')
        .map(|p| {
            // Shifting everything by one because it makes it easier to deal with usize and underflow.
            1 + p
                .parse::<i32>()
                .expect("Should be able to parse coordinate")
        })
        .collect::<Vec<i32>>();
    (parts[0], parts[1], parts[2])
}

fn neighbors(origin: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![
        (origin.0 + 1, origin.1, origin.2),
        (origin.0, origin.1 + 1, origin.2),
        (origin.0, origin.1, origin.2 + 1),
        (origin.0 - 1, origin.1, origin.2),
        (origin.0, origin.1 - 1, origin.2),
        (origin.0, origin.1, origin.2 - 1),
    ]
}

fn exposed_faces(origins: &HashSet<(i32, i32, i32)>) -> Vec<((i32, i32, i32), (i32, i32, i32))> {
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

fn bounds(origins: Vec<(i32, i32, i32)>) -> (i32, i32, i32, i32, i32, i32) {
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

fn shell_2d(min_0: i32, min_1: i32, max_0: i32, max_1: i32) -> Vec<(i32, i32)> {
    let mut res = vec![];
    for c0 in min_0 - 1..max_0 + 2 {
        for c1 in min_1 - 1..max_1 + 2 {
            res.push((c0, c1));
        }
    }
    res
}

fn shell(origins: &HashSet<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let (min_x, min_y, min_z, max_x, max_y, max_z) =
        bounds(origins.clone().into_iter().collect::<Vec<_>>());
    vec![
        shell_2d(min_x, min_y, max_x, max_y)
            .into_iter()
            .map(|o| (o.0, o.1, min_z - 1))
            .collect::<Vec<(i32, i32, i32)>>(),
        shell_2d(min_x, min_z, max_x, max_z)
            .into_iter()
            .map(|o| (o.0, min_y - 1, o.1))
            .collect::<Vec<(i32, i32, i32)>>(),
        shell_2d(min_y, min_z, max_y, max_z)
            .into_iter()
            .map(|o| (min_x - 1, o.0, o.1))
            .collect::<Vec<(i32, i32, i32)>>(),
        shell_2d(min_x, min_y, max_x, max_y)
            .into_iter()
            .map(|o| (o.0, o.1, max_z + 1))
            .collect::<Vec<(i32, i32, i32)>>(),
        shell_2d(min_x, min_z, max_x, max_z)
            .into_iter()
            .map(|o| (o.0, max_y + 1, o.1))
            .collect::<Vec<(i32, i32, i32)>>(),
        shell_2d(min_y, min_z, max_y, max_z)
            .into_iter()
            .map(|o| (max_x + 1, o.0, o.1))
            .collect::<Vec<(i32, i32, i32)>>(),
    ]
    .into_iter()
    .flatten()
    .collect::<HashSet<(i32, i32, i32)>>()
    .into_iter()
    .collect::<Vec<(i32, i32, i32)>>()
}

fn flood(
    shell: Vec<(i32, i32, i32)>,
    origins: HashSet<(i32, i32, i32)>,
) -> HashSet<(i32, i32, i32)> {
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

        let origins = lines.map(parse_line).collect::<HashSet<(i32, i32, i32)>>();

        let exposed_faces = exposed_faces(&origins);

        println!("Part one: {}", exposed_faces.len());

        let shell = shell(&origins);
        let flooded = flood(shell, origins.clone());
        let exterior_exposed_faces = exposed_faces
            .iter()
            .filter(|(_origin, face)| flooded.contains(face))
            .collect::<Vec<_>>();
        println!("Part two: {}", exterior_exposed_faces.len());
    }
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
        assert_eq!(res.0, 2);
        assert_eq!(res.1, 2);
        assert_eq!(res.2, 1);
    }

    #[test]
    fn shell_should_make_face() {
        let res = shell_2d(1, 1, 1, 1);
        assert_eq!(9, res.len());
        assert!(res.contains(&(0, 0)));
        assert!(res.contains(&(0, 1)));
        assert!(res.contains(&(0, 2)));
        assert!(res.contains(&(1, 0)));
        assert!(res.contains(&(1, 1)));
        assert!(res.contains(&(1, 2)));
        assert!(res.contains(&(2, 0)));
        assert!(res.contains(&(2, 1)));
        assert!(res.contains(&(2, 2)));

        let res = shell_2d(1, 1, 2, 2);
        assert_eq!(16, res.len());

        let res = shell_2d(1, 1, 2, 1);
        assert_eq!(12, res.len());
    }

    #[test]
    fn shell_should_encase() {
        let mut input = HashSet::new();
        input.insert((1, 1, 1));
        let res = shell(&input);

        assert_eq!(26, res.len());

        assert!(res.contains(&(0, 0, 0)));
        assert!(res.contains(&(0, 0, 1)));
        assert!(res.contains(&(0, 0, 2)));
        assert!(res.contains(&(0, 1, 0)));
        assert!(res.contains(&(0, 1, 1)));
        assert!(res.contains(&(0, 1, 2)));
        assert!(res.contains(&(0, 2, 0)));
        assert!(res.contains(&(0, 2, 1)));
        assert!(res.contains(&(0, 2, 2)));

        assert!(res.contains(&(0, 0, 0)));
        assert!(res.contains(&(0, 0, 1)));
        assert!(res.contains(&(0, 0, 2)));
        assert!(res.contains(&(1, 0, 0)));
        assert!(res.contains(&(1, 0, 1)));
        assert!(res.contains(&(1, 0, 2)));
        assert!(res.contains(&(2, 0, 0)));
        assert!(res.contains(&(2, 0, 1)));
        assert!(res.contains(&(2, 0, 2)));

        assert!(res.contains(&(0, 0, 0)));
        assert!(res.contains(&(0, 1, 0)));
        assert!(res.contains(&(0, 2, 0)));
        assert!(res.contains(&(1, 0, 0)));
        assert!(res.contains(&(1, 1, 0)));
        assert!(res.contains(&(1, 2, 0)));
        assert!(res.contains(&(2, 0, 0)));
        assert!(res.contains(&(2, 1, 0)));
        assert!(res.contains(&(2, 2, 0)));

        assert!(res.contains(&(1, 0, 0)));
        assert!(res.contains(&(1, 0, 1)));
        assert!(res.contains(&(1, 0, 2)));
        assert!(res.contains(&(1, 1, 0)));
        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(1, 2, 0)));
        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(1, 2, 2)));

        assert!(res.contains(&(0, 1, 0)));
        assert!(res.contains(&(0, 1, 1)));
        assert!(res.contains(&(0, 1, 2)));
        assert!(res.contains(&(1, 1, 0)));
        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(2, 1, 0)));
        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 1, 2)));

        assert!(res.contains(&(0, 0, 1)));
        assert!(res.contains(&(0, 1, 1)));
        assert!(res.contains(&(0, 2, 1)));
        assert!(res.contains(&(1, 0, 1)));
        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(2, 0, 1)));
        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 2, 1)));

        assert!(res.contains(&(2, 0, 0)));
        assert!(res.contains(&(2, 0, 1)));
        assert!(res.contains(&(2, 0, 2)));
        assert!(res.contains(&(2, 1, 0)));
        assert!(res.contains(&(2, 1, 1)));
        assert!(res.contains(&(2, 1, 2)));
        assert!(res.contains(&(2, 2, 0)));
        assert!(res.contains(&(2, 2, 1)));
        assert!(res.contains(&(2, 2, 2)));

        assert!(res.contains(&(0, 2, 0)));
        assert!(res.contains(&(0, 2, 1)));
        assert!(res.contains(&(0, 2, 2)));
        assert!(res.contains(&(1, 2, 0)));
        assert!(res.contains(&(1, 2, 1)));
        assert!(res.contains(&(1, 2, 2)));
        assert!(res.contains(&(2, 2, 0)));
        assert!(res.contains(&(2, 2, 1)));
        assert!(res.contains(&(2, 2, 2)));

        assert!(res.contains(&(0, 0, 2)));
        assert!(res.contains(&(0, 1, 2)));
        assert!(res.contains(&(0, 2, 2)));
        assert!(res.contains(&(1, 0, 2)));
        assert!(res.contains(&(1, 1, 2)));
        assert!(res.contains(&(1, 2, 2)));
        assert!(res.contains(&(2, 0, 2)));
        assert!(res.contains(&(2, 1, 2)));
        assert!(res.contains(&(2, 2, 2)));

        let mut input = HashSet::new();
        input.insert((1, 1, 1));
        input.insert((2, 1, 1));
        let res = shell(&input);

        assert_eq!(34, res.len());

        let mut input = HashSet::new();
        input.insert((1, 1, 1));
        input.insert((2, 1, 1));
        input.insert((1, 1, 2));
        let res = shell(&input);

        // Missing 3 for the actual blocks, plus one pocket because of the L-shaped configuration
        assert_eq!(44, res.len());
    }

    #[test]
    fn flood_should_fill_L() {
        let mut input = HashSet::new();
        input.insert((1, 1, 1));
        input.insert((2, 1, 1));
        input.insert((1, 1, 2));
        let outer_shell = shell(&input);
        let flooded = flood(outer_shell, input);
        assert_eq!(45, flooded.len());
    }
}

fn main() {
    // Day18 {}.test();
    Day18 {}.test_and_run();
}

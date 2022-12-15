use std::collections::{HashMap, HashSet};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

// (x, y)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coordinate(usize, usize);

fn can_reach(from: char, to: char) -> bool {
    let normalized_from = if from == 'S' { 'a' } else { from };
    let normalized_to = if to == 'E' { 'z' } else { to };
    (normalized_to as i32) - (normalized_from as i32) <= 1
}

#[derive(Clone, Debug)]
struct Grid {
    nodes: Vec<Vec<char>>,
}
impl Grid {
    fn get(&self, a: Coordinate) -> char {
        self.nodes[a.1][a.0]
    }

    fn find_first(&self, value: char) -> Option<Coordinate> {
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                if self.nodes[y][x] == value {
                    return Some(Coordinate(x, y));
                }
            }
        }
        None
    }
    fn find_start(&self) -> Option<Coordinate> {
        self.find_first('S')
    }
    fn find_end(&self) -> Option<Coordinate> {
        self.find_first('E')
    }
    fn find_all<T>(&self, predicate: T) -> Vec<Coordinate>
    where
        T: Fn(char) -> bool,
    {
        let mut results = vec![];
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                if predicate(self.nodes[y][x]) {
                    results.push(Coordinate(x, y))
                }
            }
        }
        results
    }
    fn find_starts(&self) -> Vec<Coordinate> {
        self.find_all(|v| v == 'S' || v == 'a')
    }

    fn one_step_condition(&self, start: &Coordinate, end: &Coordinate) -> bool {
        let start_value = self.get(start.clone());
        let end_value = self.get(end.clone());
        let normalized_start_value = if start_value == 'S' { 'a' } else { start_value };
        let normalized_end_value = if end_value == 'E' { 'z' } else { end_value };
        can_reach(normalized_start_value, normalized_end_value)
    }

    fn in_bounds(&self, c: Coordinate) -> bool {
        let Coordinate(x, y) = c;
        x >= 0 && y >= 0 && y < self.nodes.len() && x < self.nodes[0].len()
    }

    fn neighbors<T>(
        &self,
        from: Coordinate,
        visited: &HashSet<Coordinate>,
        predicate: &T,
    ) -> Vec<Coordinate>
    where
        T: Fn(&Coordinate, &Coordinate) -> bool,
    {
        // Valid neighbors are neighboring indexes which are in bounds, have not been visited yet, and satisfy the predicate
        let Coordinate(x, y) = from;
        let mut candidates = vec![Coordinate(x + 1, y), Coordinate(x, y + 1)];
        if x > 0 {
            candidates.push(Coordinate(x - 1, y));
        }
        if y > 0 {
            candidates.push(Coordinate(x, y - 1));
        }
        candidates
            .into_iter()
            .filter(|c| self.in_bounds(c.to_owned()) && !visited.contains(c) && predicate(&from, c))
            .collect()
    }

    fn shortest_paths<T>(
        &self,
        starts: Vec<Coordinate>,
        end: Coordinate,
        condition: T,
    ) -> HashMap<Coordinate, usize>
    where
        T: Fn(&Coordinate, &Coordinate) -> bool,
    {
        let mut visited: HashSet<Coordinate> = HashSet::new();
        let mut results: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();
        let mut candidates: Vec<Vec<Coordinate>> = vec![];
        for start in starts {
            candidates.push(vec![start]);
        }
        let mut is_empty: bool = candidates.is_empty();
        while !is_empty {
            let mut next_candidates: Vec<Vec<Coordinate>> = vec![];
            for i in 0..candidates.len() {
                let candidate = &candidates[i];
                if results.contains_key(&candidate[0]) {
                    continue;
                }
                if candidate[candidate.len() - 1] == end {
                    results.insert(candidate[0].clone(), candidate.clone());
                } else {
                    let candidate_neighbors = self.neighbors(
                        candidate[candidate.len() - 1].clone(),
                        &visited,
                        &condition,
                    );
                    for candidate_neighbor in candidate_neighbors {
                        let mut next_candidate = vec![];
                        next_candidate.extend(candidate.clone());
                        next_candidate.push(candidate_neighbor.clone());
                        visited.insert(candidate_neighbor.clone());
                        next_candidates.push(next_candidate);
                    }
                }
            }
            candidates.clear();
            candidates.extend(next_candidates);
            is_empty = candidates.is_empty();
        }
        results
            .iter()
            .map(|kv| (kv.0.clone(), kv.1.len() - 1))
            .collect::<HashMap<Coordinate, usize>>()
    }
}

struct Day12 {}
impl AoCProblem for Day12 {
    fn name(&self) -> String {
        "day-12".to_owned()
    }
}
impl Solution for Day12 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"))
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let grid = Grid { nodes: lines };
        let start = grid.find_start().expect("Should have a start");
        let end = grid.find_end().expect("Should have an end");
        println!(
            "Part one: {:#?}",
            grid.shortest_paths(vec![start], end.clone(), |c1, c2| grid
                .one_step_condition(c1, c2))
        );

        let starts = grid.find_starts();
        println!(
            "Part two: {:#?}",
            grid.shortest_paths(starts, end, |c1, c2| grid
                .one_step_condition(c1, c2))
        );
    }
}

fn main() {
    Day12 {}.test_and_run()
}

use std::collections::{HashMap, HashSet};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

const STARTING_POINT: &str = "AA";

#[derive(Clone, Debug)]
enum Operation {
    Open(String),
    Follow(String),
}

#[derive(Clone)]
struct Edge {
    to: String,
    weight: usize,
}

// (CurrentNode, weight, Visisted, Opened, path, (start, rate))
type BfsState = (
    String,
    usize,
    HashSet<String>,
    HashSet<String>,
    Vec<Operation>,
    Vec<(usize, usize)>,
);

struct AdjacencyList {
    edges: HashMap<String, Vec<Edge>>,
}
impl AdjacencyList {
    fn merge(&mut self, other: &AdjacencyList) {
        for (key, value) in other.edges.iter() {
            if let Some(self_edges) = self.edges.get_mut(key) {
                self_edges.extend(value.to_owned());
            } else {
                self.edges.insert(key.to_owned(), value.clone());
            }
        }
    }

    fn bfs_single_path(&self, steps: usize, state: BfsState) -> Vec<BfsState> {
        if steps == 0 {
            return vec![state];
        }
        let (current, current_weight, visited, opened, path, weights) = state;
        let should_open = !opened.contains(&current);
        let to_visit = self.edges.get(&current);
        let mut next_paths: Vec<BfsState> = if to_visit.is_none() {
            vec![]
        } else {
            to_visit
                .unwrap()
                .iter()
                .map(|e| {
                    let next_to_visit = e.clone().to;
                    let mut next_visited = HashSet::new();
                    next_visited.extend(visited.clone());
                    next_visited.insert(next_to_visit.clone());
                    let mut next_path = path.clone();
                    next_path.push(Operation::Follow(next_to_visit.clone()));
                    let mut next_weights = weights.clone();
                    next_weights.push((steps - 1, e.weight));
                    (
                        next_to_visit.clone(),
                        e.weight,
                        next_visited,
                        opened.clone(),
                        next_path,
                        next_weights,
                    )
                })
                .collect()
        };
        if should_open {
            let mut next_opened = opened.clone();
            next_opened.insert(current.clone());
            let mut next_weights = weights.clone();
            next_weights.push((steps - 1, current_weight));
            next_paths.push((
                current.clone(),
                current_weight,
                visited.clone(),
                next_opened,
                path.clone(),
                next_weights,
            ));
        }
        return next_paths;
    }

    fn bfs(&self, steps: usize) -> Vec<BfsState> {
        let mut initial_visited = HashSet::new();
        initial_visited.insert(STARTING_POINT.to_owned());
        let initial_state: BfsState = (
            STARTING_POINT.to_owned(),
            0,
            initial_visited,
            HashSet::new(),
            vec![],
            vec![],
        );
        self.bfs_single_path(1 + steps, initial_state)
    }
}
impl TryFrom<String> for AdjacencyList {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value
            .split(' ')
            .map(|p| p.to_owned())
            .collect::<Vec<String>>();
        let from = parts[1].clone();
        let rate = parts[4]
            .split_once('=')
            .expect("Should be able to split rate")
            .1
            .strip_suffix(';')
            .expect("Should be able to remove trailing ';'")
            .parse::<usize>()
            .expect("Should be able to parse rate as usize");
        let destinations = parts[9..]
            .iter()
            .map(|p| p.trim_end_matches(',').to_owned());
        let mut edges = HashMap::new();
        let edge_list = destinations
            .map(|d| Edge {
                to: d,
                weight: rate,
            })
            .collect::<Vec<Edge>>();
        edges.insert(from, edge_list);
        Ok(AdjacencyList { edges })
    }
}

struct Day16 {}
impl AoCProblem for Day16 {
    fn name(&self) -> String {
        "day-16".to_owned()
    }
}
impl Solution for Day16 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"));

        let adjacencies = lines
            .map(AdjacencyList::try_from)
            .map(|maybe| maybe.expect("Should be able to parse adjacencies"))
            .reduce(|mut p, n| {
                p.merge(&n);
                p
            })
            .expect("Should be able to reduce adjacencies");
        
        let result = adjacencies.bfs(30);
        println!("{:#?}", result);
    }
}

fn main() {
    Day16 {}.test();
    // Day16 {}.test_and_run();
}

use itertools::{Itertools, Permutations};
use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

const STARTING_POINT: &str = "AA";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Node {
    label: String,
    rate: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Edge {
    to: String,
    cost: usize,
}

#[derive(Debug)]
struct AdjacencyList {
    edges: HashMap<Node, HashSet<Edge>>,
}
impl AdjacencyList {
    fn merge(&mut self, other: &AdjacencyList) {
        for (key, value) in other.edges.iter() {
            if let Some(self_edges) = self.edges.get_mut(key) {
                for e in value {
                    self_edges.insert(e.to_owned());
                }
            } else {
                self.edges.insert(key.to_owned(), value.clone());
            }
        }
    }

    fn paths(&self) -> Permutations<IntoIter<String>> {
        // Find all paths that visit each node _except_ the starting node
        let to_visit = self
            .edges
            .keys()
            .map(|k| k.label.to_owned())
            // .filter(|l| l != STARTING_POINT)
            .collect::<Vec<String>>();
        let to_visit_len = to_visit.len();
        to_visit.into_iter().permutations(to_visit_len)
    }

    fn _shortest_path(
        &self,
        from: String,
        to: String,
        visited: HashSet<String>,
        cost: usize,
    ) -> Option<usize> {
        if from.eq(&to) {
            return Some(cost);
        }
        let candidates = self
            .edges
            .iter()
            .filter(|kv| kv.0.label.eq(&from))
            .next()
            .expect("Node should be in graph")
            .1
            .iter()
            .filter(|e| !visited.contains(&e.to))
            .collect::<Vec<&Edge>>();
        if candidates.len() == 0 {
            return None;
        }
        candidates
            .into_iter()
            .map(|c| {
                let mut new_visited = visited.clone();
                new_visited.insert(c.to.clone());
                return self._shortest_path(c.to.clone(), to.clone(), new_visited, cost + c.cost);
            })
            .filter(|c| c.is_some())
            .min()
            .flatten()
    }

    fn shortest_path(&self, from: String, to: String) -> Option<usize> {
        self._shortest_path(from, to, HashSet::new(), 0)
            .map(|v| 1 + v) // +1 for cost to activate - no point in visiting a node directly if we're not activating it
    }

    fn sources_of(&self, target: String) -> Vec<(Node, Edge)> {
        self.edges
            .iter()
            .flat_map(|kv| {
                kv.1.iter()
                    .map(|edge| (kv.0, edge))
                    .collect::<Vec<(&Node, &Edge)>>()
            })
            .filter(|&(_, edge)| edge.to.eq(&target))
            .map(|(n, e)| (n.to_owned(), e.to_owned()))
            .collect::<Vec<(Node, Edge)>>()
    }

    fn score(&self, path: Vec<String>) -> usize {
        // Given a path, compute the score of following that path for as long as possible
        // or None if not possible at all
        let mut budget = 30;
        let mut score = 0;
        for segment in path.windows(2) {
            let from = &segment[0];
            let to = &segment[1];
            let path_cost = self.shortest_path(from.to_owned(), to.to_owned());
            if path_cost.is_none() {
                return score;
            }
            let path_cost = path_cost.unwrap();
            if path_cost > budget {
                return score;
            }
            let node_rate = self
                .edges
                .keys()
                .filter(|k| k.label.eq(to))
                .next()
                .expect("Node should exist")
                .rate;
            budget -= path_cost;
            score += (node_rate * budget);
        }
        score
    }

    fn part_one(&self, paths: Vec<Vec<String>>) -> usize {
        // Given a bunch of paths, score each path and return the highest score
        let mut max_score = 0;
        for path in paths {
            let path_score = self.score(path);
            max_score = max_score.max(path_score);
        }
        max_score
    }

    fn compact(&mut self) {
        // A bunch of nodes have rate equal to zero. We never have a reason to turn them on or visit them, other than
        // in passing to another node. Lets get rid of them to trim up the search space
        // Keep the starting node, for convenience
        loop {
            // Find a node with rate 0
            let to_remove = self
                .edges
                .iter()
                .filter(|&(node, _)| node.rate == 0 && !node.label.eq(STARTING_POINT))
                .map(|(n, e)| (n.to_owned(), e.to_owned()))
                .next();
            if to_remove.is_none() {
                break;
            }
            let (node_to_remove, destinations_to_induce) =
                to_remove.expect("Should have a node to remove");

            // Find all nodes with edges to that node
            let sources = self.sources_of(node_to_remove.label.clone());

            // Induce new edges between those sources to the destiations of that node
            for (source_node, source_edge) in sources {
                let source_node_edges = self
                    .edges
                    .get_mut(&source_node)
                    .expect("Source node should exist in edgelist");
                for destination in destinations_to_induce.iter() {
                    if destination.to.eq(&source_node.label) {
                        continue;
                    }
                    let edge_to_add = Edge {
                        to: destination.to.clone(),
                        cost: source_edge.cost + destination.cost,
                    };
                    source_node_edges.insert(edge_to_add);
                }
                source_node_edges.remove(&source_edge);
            }

            // Remove the zero rate node
            self.edges.remove(&node_to_remove);
        }
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
        let mut edges: HashMap<Node, HashSet<Edge>> = HashMap::new();
        let edge_list = destinations
            .map(|d| Edge { to: d, cost: 1 })
            .collect::<HashSet<Edge>>();
        let node = Node { label: from, rate };
        edges.insert(node, edge_list);
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

        let mut adjacencies = lines
            .map(AdjacencyList::try_from)
            .map(|maybe| maybe.expect("Should be able to parse adjacencies"))
            .reduce(|mut p, n| {
                p.merge(&n);
                p
            })
            .expect("Should be able to reduce adjacencies");

        adjacencies.compact();
        let mut best_score = (vec![], 0);
        for path in adjacencies.paths() {
            let score = adjacencies.score(path.to_owned());
            if score > best_score.1 {
                best_score = (path, score);
            }
        }

        println!("Part 1: {} :{}", best_score.0.join("-"), best_score.1);
    }
}

fn main() {
    // Day16 {}.test();
    Day16 {}.test_and_run();
}

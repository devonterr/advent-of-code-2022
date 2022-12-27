use itertools::{Itertools, Permutations};
use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

const STARTING_POINT: &str = "AA";

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    label: String,
    rate: usize,
    edges: HashSet<Edge>,
}
impl TryFrom<String> for Node {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value
            .split(' ')
            .map(|p| p.to_owned())
            .collect::<Vec<String>>();
        let label = parts[1].clone();
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
        let mut edges: HashSet<Edge> = HashSet::new();
        let edges = destinations
            .map(|d| Edge { to: d, cost: 1 })
            .collect::<HashSet<Edge>>();
        Ok(Node { label, rate, edges })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Edge {
    to: String,
    cost: usize,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
    shortest_paths: HashMap<(String, String), Option<usize>>,
}
impl Graph {
    fn paths(&self) -> Permutations<IntoIter<String>> {
        // Find all paths that visit each node _except_ the starting node
        let to_visit = self
            .nodes
            .keys()
            .filter(|&l| !l.eq(STARTING_POINT))
            .map(|l| l.to_owned())
            .collect::<Vec<String>>();
        let to_visit_len = to_visit.len();
        to_visit.into_iter().permutations(to_visit_len)
    }

    fn _shortest_path(&mut self, from: String, to: String) -> Option<usize> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut distances = self
            .nodes
            .keys()
            .map(|k| (k.to_owned(), usize::max_value()))
            .collect::<HashMap<String, usize>>();
        let mut current_node = self.nodes.get(&from).expect("Should have from node");
        distances.insert(current_node.label.to_owned(), 0);
        loop {
            let neighbors = self
                .nodes
                .get(&current_node.label)
                .expect("Should be able to get current node")
                .edges
                .iter()
                .filter(|edge| !visited.contains(&edge.to))
                .map(|e| e.to_owned())
                .collect::<Vec<Edge>>();
            let current_distance = distances
                .get(&current_node.label)
                .expect("Current node should have known distance")
                .to_owned();
            for neighbor in neighbors {
                let distance_through_current = current_distance + neighbor.cost;
                let current_distance_to_neighbor = distances
                    .get(&neighbor.to)
                    .expect("Should have a distance to the current neighbor")
                    .to_owned();
                distances.insert(
                    neighbor.to,
                    current_distance_to_neighbor.min(distance_through_current),
                );
            }
            visited.insert(current_node.label.to_owned());
            if visited.contains(&to) {
                break;
            }
            let unvisited = self.nodes.keys().filter(|&label| !visited.contains(label));
            let mut smallest = usize::max_value();
            for u in unvisited {
                let distance_to_u = distances
                    .get(u)
                    .expect("Should have a distance for unvisited")
                    .to_owned();
                if distance_to_u < smallest {
                    smallest = distance_to_u;
                    current_node = self.nodes.get(u).expect("Next unvisited should exist");
                }
            }
            if smallest == usize::max_value() {
                println!("Stopping because smallest");
                return None;
            }
        }
        Some(
            distances
                .get(&to)
                .expect("Should have distance to the target node")
                .to_owned(),
        )
    }

    fn shortest_path(&mut self, from: String, to: String) -> Option<usize> {
        let key = (from.clone(), to.clone());
        if self.shortest_paths.contains_key(&key) {
            return self
                .shortest_paths
                .get(&key)
                .expect("Should contain")
                .to_owned();
        } else {
            let res = self._shortest_path(from, to).map(|v| 1 + v); // +1 for cost to activate - no point in visiting a node directly if we're not activating it
                                                                    // self.shortest_paths.insert(key, res);
            res
        }
    }

    fn sources_of(&self, target: String) -> Vec<(String, Edge)> {
        self.nodes
            .iter()
            .flat_map(|kv| {
                kv.1.edges
                    .iter()
                    .map(|edge| (kv.0, edge))
                    .collect::<Vec<(&String, &Edge)>>()
            })
            .filter(|&(_, edge)| edge.to.eq(&target))
            .map(|(n, e)| (n.to_owned(), e.to_owned()))
            .collect::<Vec<(String, Edge)>>()
    }

    fn score(&mut self, path: Vec<String>) -> usize {
        // Given a path, compute the score of following that path for as long as possible
        // or None if not possible at all

        let mut path_with_start = vec![STARTING_POINT.to_owned()];
        path_with_start.extend(path);
        let mut budget = 30;
        let mut score = 0;
        for segment in path_with_start.windows(2) {
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
                .nodes
                .iter()
                .filter(|(k, _)| k.to_owned().eq(to))
                .map(|kv| kv.1.rate.to_owned())
                .next()
                .expect("Node should exist");
            budget -= path_cost;
            score += node_rate * budget;
        }
        score
    }

    fn part_one(&mut self, paths: Vec<Vec<String>>) -> usize {
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
                .nodes
                .iter()
                .filter(|&(label, node)| node.rate == 0 && !label.eq(STARTING_POINT))
                .map(|(n, e)| (n.to_owned(), e.to_owned()))
                .next();
            if to_remove.is_none() {
                break;
            }
            let (node_to_remove, destinations_to_induce) =
                to_remove.expect("Should have a node to remove");

            // Find all nodes with edges to that node
            let sources = self.sources_of(node_to_remove.to_owned());

            // Induce new edges between those sources to the destiations of that node
            for (source_node, source_edge) in sources {
                let source_node_edges = self
                    .nodes
                    .get_mut(&source_node)
                    .expect("Source node should exist in edgelist");
                for destination in destinations_to_induce.edges.iter() {
                    if destination.to.eq(&source_node) {
                        continue;
                    }
                    let edge_to_add = Edge {
                        to: destination.to.clone(),
                        cost: source_edge.cost + destination.cost,
                    };
                    source_node_edges.edges.insert(edge_to_add);
                }
                source_node_edges.edges.remove(&source_edge);
            }

            // Remove the zero rate node
            self.nodes.remove(&node_to_remove);
        }
    }

    fn new(nodes: HashMap<String, Node>) -> Graph {
        let mut graph = Graph {
            nodes,
            shortest_paths: HashMap::new(),
        };
        graph.compact();
        graph
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

        let nodes = lines
            .map(Node::try_from)
            .map(|maybe| maybe.expect("Should be able to parse Node"))
            .map(|node| (node.label.to_owned(), node))
            .collect::<HashMap<String, Node>>();

        let mut graph = Graph::new(nodes);

        let mut best_score = (vec![], 0);
        for path in graph.paths() {
            let score = graph.score(path.to_owned());
            if score > best_score.1 {
                best_score = (path, score);
            }
        }

        println!("Part 1: {} :{}", best_score.0.join("-"), best_score.1);
    }
}

fn main() {
    // Day16 {}.run();
    // Day16 {}.test();
    Day16 {}.test_and_run();
}

use std::collections::HashSet;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

// fn find_shortest_path(
//     nodes: &Grid,
//     edges: &Vec<Edge>,
//     visited: HashSet<Coordinate>,
//     // cost_cache: HashMap<Coordinate, usize>,
//     path: Vec<Coordinate>,
//     from: Coordinate,
//     to: &Coordinate,
// ) -> Option<Vec<Coordinate>> {
//     // If we're at the end ,return so_far
//     if from == *to {
//         return Some(path);
//     }
//     // Otherwrise the shortest path is the minimum of the shortest paths from each unvisited neighbor to the end
//     nodes
//         .neighbors(from.0, from.1)
//         .into_iter()
//         .filter(|n| !visited.contains(n))
//         .filter(|n| {
//             edges
//                 .iter()
//                 .filter(|e| e.from == from && e.to == to)
//                 .next()
//                 .is_some()
//         })
//         .map(|n| {
//             let mut new_visited = visited.clone();
//             new_visited.insert(n.clone());
//             let mut new_path = path.clone();
//             new_path.push(n.clone());
//             find_shortest_path(nodes, edges, new_visited, new_path, n, &to)
//         })
//         .filter(|o| o.is_some())
//         .map(|o| o.unwrap())
//         .min_by_key(|p| p.len())
// }

// (x, y)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Coordinate(usize, usize);

#[derive(Clone, Debug)]
struct Edge {
    from: Coordinate,
    to: Coordinate,
}

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
    fn bounds(&self) -> (usize, usize) {
        let max_y = self.nodes.len();
        let max_x = self.nodes[0].len();
        (max_x, max_y)
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<Coordinate> {
        let (max_x, max_y) = self.bounds();
        let mut results = vec![(x + 1, y), (x, y + 1)]
            .iter()
            .filter(|(x, y)| x >= &0 && y >= &0 && x < &max_x && y < &max_y)
            .map(|c| c.to_owned())
            .map(|c| Coordinate(c.0, c.1))
            .collect::<Vec<Coordinate>>();
        if x > 0 {
            results.push(Coordinate(x - 1, y));
        }
        if y > 0 {
            results.push(Coordinate(x, y - 1));
        }
        results
    }

    fn iter_indexes(&self) -> Vec<Coordinate> {
        let mut result = vec![];
        for y in 0..self.nodes.len() {
            for x in 0..self.nodes[0].len() {
                let c = Coordinate(x, y);
                result.push(c);
            }
        }
        result
    }

    fn get(&self, coordinate: Coordinate) -> char {
        self.nodes[coordinate.1][coordinate.0]
    }

    fn start(&self) -> Coordinate {
        self.iter_indexes()
            .into_iter()
            .filter(|i| self.get(i.to_owned()) == 'S')
            .next()
            .expect("Should have a start")
    }

    fn end(&self) -> Coordinate {
        self.iter_indexes()
            .into_iter()
            .filter(|i| self.get(i.to_owned()) == 'E')
            .next()
            .expect("Should have a start")
    }
    fn starts(&self) -> Vec<Coordinate> {
        self.iter_indexes()
            .into_iter()
            .filter(|i| {
                let value = self.get(i.to_owned());
                value == 'S' || value == 'a'
            })
            .collect::<Vec<Coordinate>>()
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: Grid,
    edges: Vec<Edge>,
}
impl Graph {
    fn one_up_graph(grid: Grid) -> Graph {
        let edges = grid
            .iter_indexes()
            .iter()
            .flat_map(|c| {
                grid.neighbors(c.0.clone(), c.1.clone())
                    .iter()
                    .map(|n| Edge {
                        from: c.clone(),
                        to: Coordinate(n.0, n.1).clone(),
                    })
                    .collect::<Vec<Edge>>()
            })
            .filter(|candidate_edge| {
                can_reach(
                    grid.get(candidate_edge.from.clone()),
                    grid.get(candidate_edge.to.clone()),
                )
            })
            .collect();
        Graph { nodes: grid, edges }
    }

    fn find_shortest_path(&self) -> Option<Vec<Coordinate>> {
        self.find_shortest_path_from(self.nodes.start())
    }

    fn find_shortest_path_from(&self, start: Coordinate) -> Option<Vec<Coordinate>> {
        let end = self.nodes.end();
        let mut candidate_paths: Vec<(usize, Vec<Coordinate>)> = vec![(0, vec![start])];
        let mut visited: HashSet<Coordinate> = HashSet::new();

        loop {
            // Ensure the lowest weight is at the end
            candidate_paths.sort_by_key(|e| e.0);
            candidate_paths.reverse();

            // Pick the lightest-weight path, from the end
            if candidate_paths.is_empty() {
                return None;
            }
            let (next_to_consider_weight, next_to_consider_path) =
                candidate_paths.pop().expect("Should be non-empty");
            let next_to_consider_tail = &next_to_consider_path[next_to_consider_path.len() - 1];
            if *next_to_consider_tail == end {
                // Drop the first item, the start of the path
                return Some(next_to_consider_path.to_owned()[1..].to_vec());
            }
            let next_candidate_nodes = self
                .edges
                .iter()
                .filter(|edge| edge.from == next_to_consider_tail.clone())
                .map(|e| e.to.clone())
                .filter(|c| !visited.contains(c))
                .collect::<Vec<Coordinate>>();
            for ncn in next_candidate_nodes {
                visited.insert(ncn.clone());
                let mut new_path = next_to_consider_path.clone();
                new_path.push(ncn);
                candidate_paths.push((next_to_consider_weight.clone() + 1, new_path));
            }
        }
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
        let bounds = grid.bounds();
        let graph = Graph::one_up_graph(grid);
        let path = graph.find_shortest_path().expect("Should have an answer");
        println!("part one: {:#?}", path.len());
        // println!(
        //     "Nani? {:#?}",
        //     find_shortest_path(
        //         &graph.nodes,
        //         graph.edges,
        //         HashSet::new(),
        //         vec![grid.start()],
        //         grid.start(),
        //         &grid.end()
        //     )
        // );

        // Lots of starts.
        // Regardless of where we start, once we've reached a given node
        // the cost to get from there to E is the same. We can cache that
        // BETWEEN runs.
        let starts = graph.nodes.starts();
        let upper_limit = bounds.0 * bounds.1;
        let shortest_from_starts = starts
            .iter()
            .map(|s| {
                graph
                    .find_shortest_path_from(s.to_owned())
                    .map(|p| p.len())
                    .unwrap_or(upper_limit)
            })
            .min();
        println!(
            "part two: {:#?}",
            shortest_from_starts.expect("Should have an answer")
        );
    }
}

fn main() {
    Day12 {}.test()
    // Day12 {}.test_and_run()
}

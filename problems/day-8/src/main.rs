use std::collections::{HashMap, HashSet};

use shared::{read_lines, AoCProblem, AoCSolution, Solution};

// For a given direction, an index is visible if no prior value is greater
fn visible_from_outside(run: Vec<((usize, usize), i32)>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut max_in_run: i32 = -1;

    for (idx, value) in run {
        if value > max_in_run {
            result.push(idx);
            max_in_run = value;
        }
    }

    result
}

fn visible_from_any_side(grid: Vec<Vec<((usize, usize), i32)>>) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for run in grid {
        let visible_in_run = visible_from_outside(run);
        result.extend(visible_in_run);
    }
    result
}

fn rotated<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut result: Vec<Vec<T>> = vec![];
    for _ in 0..grid[0].len() {
        result.push(vec![]);
    }
    for row in grid {
        for (i, e) in row.iter().enumerate() {
            result[i].push(e.clone());
        }
    }
    result
}

fn flipped<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut result = vec![];
    for row in grid {
        result.push(row.iter().rev().map(|c| c.clone()).collect());
    }
    result
}

fn index<T>(grid: Vec<Vec<T>>) -> Vec<Vec<((usize, usize), T)>>
where
    T: Clone,
{
    let mut result = vec![];
    for (i, row) in grid.iter().enumerate() {
        let mut new_row = vec![];
        for (j, col) in row.iter().enumerate() {
            new_row.push(((i, j), col.clone()));
        }
        result.push(new_row);
    }
    result
}

fn view_score(run: Vec<((usize, usize), i32)>) -> Vec<((usize, usize), usize)> {
    let mut result = vec![];
    // Look at sliding windows, starting from the head of the run to the end, then the second element to the end, etc
    for start in 0..run.len() {
        let (start_position, start_height) = run[start];
        let mut score = 0;
        for idx_to_check in start + 1..run.len() {
            score += 1;
            let height_to_check = run[idx_to_check].1;
            if height_to_check >= start_height {
                break;
            }
        }
        result.push((start_position, score));
    }
    result
}

fn all_view_scores(grid: Vec<Vec<((usize, usize), i32)>>) -> Vec<((usize, usize), usize)> {
    let mut result = vec![];
    for row in grid {
        result.extend(view_score(row));
    }
    result
}

struct Day8 {}
impl AoCProblem for Day8 {
    fn name(&self) -> String {
        "day-8".to_owned()
    }
}
impl Solution for Day8 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path).expect("Should be able to read file");
        let grid: Vec<Vec<i32>> = lines.fold(vec![], |mut acc, line| {
            let row: Vec<i32> = line
                .expect("Should be able to read line")
                .chars()
                .map(|c| c.to_string().parse::<i32>().expect("Should parse"))
                .collect();
            acc.push(row);
            acc
        });
        let right_to_left = flipped(index(grid.clone()));
        let left_to_right = index(grid.clone());
        let top_to_bottom = rotated(index(grid.clone()));
        let bottom_to_top = flipped(rotated(index(grid.clone())));

        let mut visible_trees = HashSet::new();
        visible_trees.extend(visible_from_any_side(right_to_left.clone()));
        visible_trees.extend(visible_from_any_side(left_to_right.clone()));
        visible_trees.extend(visible_from_any_side(top_to_bottom.clone()));
        visible_trees.extend(visible_from_any_side(bottom_to_top.clone()));

        println!("Part One: {:#?}", visible_trees.len());

        let mut view_scores = vec![];
        view_scores.extend(all_view_scores(right_to_left.clone()));
        view_scores.extend(all_view_scores(left_to_right.clone()));
        view_scores.extend(all_view_scores(top_to_bottom.clone()));
        view_scores.extend(all_view_scores(bottom_to_top.clone()));

        let mut aggregated_view_scores: HashMap<(usize, usize), usize> = HashMap::new();
        for view_score in view_scores {
            let previous_score = aggregated_view_scores.get(&view_score.0).unwrap_or(&1);
            aggregated_view_scores.insert(view_score.0, previous_score * view_score.1);
        }
        let mut highest_view_scores: Vec<((usize, usize), usize)> =
            aggregated_view_scores.into_iter().collect();
        highest_view_scores.sort_by_key(|e| e.1);
        highest_view_scores.reverse();
        println!("Part Two: {:#?}", highest_view_scores[0].1);
    }
}

fn main() {
    Day8 {}.test_and_run();
}

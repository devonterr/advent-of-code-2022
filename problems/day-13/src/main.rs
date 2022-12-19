use std::cmp::Ordering;

use shared::{read_lines, AoCProblem, AoCSolution, Solution};
use serde_json::Value;

// If both values are integers, the lower integer should come first. If the left integer is lower than the
// right integer, the inputs are in the right order. If the left integer is higher than the right integer, the
// inputs are not in the right order. Otherwise, the inputs are the same integer; continue checking the next
// part of the input.

// If both values are lists, compare the first value of each list, then the second value, and so on. If the
// left list runs out of items first, the inputs are in the right order. If the right list runs out of items
//first, the inputs are not in the right order. If the lists are the same length and no comparison makes a
//decision about the order, continue checking the next part of the input.

// If exactly one value is an integer, convert the integer to a list which contains that integer as its only
// value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2]
// (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].

fn cmp(a: Value, b: Value) -> Option<bool> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) if a.as_i64() < b.as_i64() => Some(true),
        (Value::Number(a), Value::Number(b)) if a.as_i64() > b.as_i64() => Some(false),
        (Value::Number(a), Value::Number(b)) => None,
        (Value::Array(ays), Value::Array(bs)) => {
            let bs_len = bs.len();
            let ays_len = ays.len();
            for (a, b) in ays.iter().zip(bs) {
                let res = cmp(a.to_owned(), b);
                if let Some(result) = res {
                    return Some(result);
                }
            }
            match (ays_len, bs_len) {
                (a, b) if b < a => Some(false),
                (a, b) if a < b => Some(true),
                _ => None,
            }
        },
        (Value::Number(a), Value::Array(bs)) => cmp(Value::Array(vec![Value::Number(a)]), Value::Array(bs)),
        (Value::Array(ays), Value::Number(b)) => cmp(Value::Array(ays), Value::Array(vec![Value::Number(b)])),
        _ => None
    }
}

#[derive(Debug)]
struct Packet {
    left: Value,
    right: Value,
}
impl Packet {
    fn is_valid(&self) -> bool {
        cmp(self.left.to_owned(), self.right.to_owned()).expect("Should produce a result")
    }
}
impl TryFrom<&[String]> for Packet {
    type Error = String;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        let n1 = value[0].clone();
        let n2 = value[1].clone();
        let left = serde_json::from_str(&n1).or(Err("Couldn't unwrap n1"))?;
        let right = serde_json::from_str(&n2).or(Err("Couldn't unwrap n2"))?;
        Ok(Packet { left, right })
    }
}

struct Day13 {}
impl AoCProblem for Day13 {
    fn name(&self) -> String {
        "day-13".to_owned()
    }
}
impl Solution for Day13 {
    fn solution(&self, path: &str) {
        let valid_packets = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"))
            .collect::<Vec<String>>()
            .chunks(3)
            .map(|chunk| Packet::try_from(chunk).expect("Should be able to parse packet"))
            .enumerate()
            .filter(|e_packet| e_packet.1.is_valid())
            .map(|(i, p)| (i+1, p))
            .collect::<Vec<(usize, Packet)>>();
        println!(
            "Part One: {}",
            valid_packets.iter().map(|ep| ep.0).sum::<usize>()
        );

        let mut individual_packets = read_lines(path)
            .expect("Should be able to read file")
            .map(|line| line.expect("Should be able to read line"))
            .filter(|line| !line.trim().is_empty())
            .map(|chunk| serde_json::from_str(chunk.trim()).expect("Should be able to parse packet"))
            .collect::<Vec<Value>>();
        individual_packets.push(serde_json::from_str("[[2]]").expect("Should deser"));
        individual_packets.push(serde_json::from_str("[[6]]").expect("Should deser"));
        individual_packets.sort_by(|a, b| cmp(a.to_owned(), b.to_owned()).map(|r| if r { Ordering::Less} else {Ordering::Greater}).unwrap_or(Ordering::Equal));
        let dividers = individual_packets.iter().enumerate().filter(|pair| {
            let value = pair.1;
            match value {
                Value::Array(v) => {
                    if v.len() != 1 {
                        return false;
                    }
                    let first = &v[0];
                    match first {
                        Value::Array(w) => {
                            if w.len() != 1 {
                                return false;
                            }
                            let first = &w[0];
                            if !first.is_number() {
                                return false;
                            }
                            let maybe_num = first.as_i64();
                            if maybe_num.is_none() {
                                return false;
                            }
                            let num =  maybe_num.unwrap();
                            return num == 2 || num == 6;
                        },
                        _ => false
                    }
                },
                _ => false,
            }
        }).map(|pair| 1 + pair.0)
        .collect::<Vec<usize>>();
        println!("Part 2: {:#?}", dividers[0] * dividers[1]);
    }
}

fn main() {
    Day13 {}.test_and_run();
}

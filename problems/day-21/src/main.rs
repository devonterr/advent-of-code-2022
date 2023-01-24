use std::collections::HashMap;

use itertools::Itertools;
use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Debug, Clone)]
enum Expression {
    Value(i64),
    Sym(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
}
impl Expression {
    fn evaluate(&self) -> Result<i64, String> {
        match self {
            Expression::Value(i) => Ok(*i),
            Expression::Add(lhs, rhs) => Ok((*lhs).evaluate()? + (*rhs).evaluate()?),
            Expression::Sub(lhs, rhs) => Ok((*lhs).evaluate()? - (*rhs).evaluate()?),
            Expression::Mul(lhs, rhs) => Ok((*lhs).evaluate()? * (*rhs).evaluate()?),
            Expression::Div(lhs, rhs) => Ok((*lhs).evaluate()? / (*rhs).evaluate()?),
            Expression::Eq(lhs, rhs) => {
                Expression::Sub(Box::new(*lhs.clone()), Box::new(*rhs.clone())).solve()
            }
            Expression::Sym(s) => Err(format!("Cannot evaluate unknown symbol: {}", s)),
        }
    }

    fn invert(&self, result: &Expression) -> (Expression, Expression) {
        // Split up each side, try to evaluate each side
        // If evaluation fails with "Cannot evalute unknown symbol: humn" then we need to invert the op and apply it, returning the remaining expression to recurse
        match self {
            Expression::Add(lhs, rhs) => {
                let (to_apply, to_solve) = lhs.evaluate().map(|r| (r, rhs)).unwrap_or_else(|_| {
                    (
                        rhs.evaluate().expect(
                            "Unable to evaluate either side of Add expression when solving",
                        ),
                        lhs,
                    )
                });
                return (
                    *to_solve.clone(),
                    Expression::Sub(
                        Box::new(result.clone()),
                        Box::new(Expression::Value(to_apply)),
                    ),
                );
            }
            Expression::Sub(lhs, rhs) => lhs
                .evaluate()
                .map(|r| {
                    (
                        *rhs.clone(),
                        Expression::Sub(Box::new(Expression::Value(r)), Box::new(result.clone())),
                    )
                })
                .unwrap_or_else(|_| {
                    (
                        *lhs.clone(),
                        Expression::Add(
                            Box::new(result.clone()),
                            Box::new(Expression::Value(rhs.evaluate().expect(
                                "Unable to evaluate either side of Sub expression while solving",
                            ))),
                        ),
                    )
                }),
            Expression::Mul(lhs, rhs) => {
                let (to_apply, to_solve) = lhs.evaluate().map(|r| (r, rhs)).unwrap_or_else(|_| {
                    (
                        rhs.evaluate().expect(
                            "Unable to evaluate either side of Add expression when solving",
                        ),
                        lhs,
                    )
                });
                return (
                    *to_solve.clone(),
                    Expression::Div(
                        Box::new(result.clone()),
                        Box::new(Expression::Value(to_apply)),
                    ),
                );
            }
            Expression::Div(lhs, rhs) => lhs
                .evaluate()
                .map(|r| {
                    (
                        *rhs.clone(),
                        Expression::Div(Box::new(Expression::Value(r)), Box::new(result.clone())),
                    )
                })
                .unwrap_or_else(|_| {
                    (
                        *lhs.clone(),
                        Expression::Mul(
                            Box::new(result.clone()),
                            Box::new(Expression::Value(rhs.evaluate().expect(
                                "Unable to evaluate either side of Sub expression while solving",
                            ))),
                        ),
                    )
                }),
            _ => panic!("Cannot invert non-binary operation"),
        }
    }

    fn solve_rec(remainder: &Expression, result: &Expression) -> Result<i64, String> {
        match remainder {
            Expression::Sym(_) => result.evaluate(),
            Expression::Value(_) => Err("No symbol found".to_owned()),
            Expression::Eq(_, _) => Err("Multiple equivalnces found".to_owned()),
            op => {
                let (remainder, result) = remainder.invert(result);
                Self::solve_rec(&remainder, &result)
            }
        }
    }

    fn solve(&self) -> Result<i64, String> {
        let (remainder, result) = self.invert(&Expression::Value(0));
        Self::solve_rec(&remainder, &result)
    }

    fn from(lines: &HashMap<String, String>, key: String) -> Result<Expression, String> {
        let subexpression = lines.get(&key);
        if subexpression.is_none() {
            return Ok(Expression::Sym(key));
        }
        let subexpression = subexpression.unwrap();

        // Try to parse literal value
        let literal_result = subexpression.parse::<i64>();
        if literal_result.is_ok() {
            let literal_value = literal_result.unwrap();
            return Ok(Expression::Value(literal_value));
        }

        // Otherwise it must be an operation
        let (lhs, op, rhs) = subexpression
            .split(' ')
            .collect_tuple()
            .ok_or(format!("Unexpected number of parts in {}", subexpression))?;

        let left_subexpression = Expression::from(&lines, lhs.to_owned())
            .map_err(|e| format!("Failed to parse lhs: {}", e))?;
        let right_subexpression = Expression::from(&lines, rhs.to_owned())
            .map_err(|e| format!("Failed to parses rhs: {}", e))?;

        match op {
            "+" => Ok(Expression::Add(
                Box::new(left_subexpression),
                Box::new(right_subexpression),
            )),
            "-" => Ok(Expression::Sub(
                Box::new(left_subexpression),
                Box::new(right_subexpression),
            )),
            "*" => Ok(Expression::Mul(
                Box::new(left_subexpression),
                Box::new(right_subexpression),
            )),
            "/" => Ok(Expression::Div(
                Box::new(left_subexpression),
                Box::new(right_subexpression),
            )),
            "=" => Ok(Expression::Eq(
                Box::new(left_subexpression),
                Box::new(right_subexpression),
            )),
            _ => Err(format!("Unrecognized op: {}", op)),
        }
    }
}
impl TryFrom<HashMap<String, String>> for Expression {
    type Error = String;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        Expression::from(&value, "root".to_owned())
    }
}

struct Day21 {}
impl AoCProblem for Day21 {
    fn name(&self) -> String {
        "day-21".to_owned()
    }
}
impl Solution for Day21 {
    fn solution(&self, path: &str) {
        let lines = read_lines(path)
            .expect("Should be able to read lines")
            .map(|line| line.expect("Should be able to read line"))
            .collect::<Vec<_>>();

        let mut line_map = HashMap::new();
        for line in lines.clone() {
            let (key, subexpression) = line.split(": ").collect_tuple().unwrap();
            line_map.insert(key.to_owned(), subexpression.to_owned());
        }

        let expression = Expression::try_from(line_map).unwrap();
        println!("Part 1: {}", expression.evaluate().unwrap());

        // Part 2
        let mut line_map = HashMap::new();
        for line in lines {
            let (key, subexpression) = line.split(": ").collect_tuple().unwrap();
            if key.eq("humn") {
                continue;
            }
            line_map.insert(key.to_owned(), subexpression.to_owned());
        }

        let updated_root = line_map
            .get("root")
            .unwrap()
            .replace('+', "=")
            .replace('-', "=")
            .replace('*', "=")
            .replace('/', "=");
        line_map.insert("root".to_owned(), updated_root);
        let expression = Expression::try_from(line_map).unwrap();
        // println!("{:#?}", expression);
        println!("Part 2: {}", expression.evaluate().unwrap());
    }
}

fn main() {
    Day21 {}.test_and_run();
}

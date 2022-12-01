# Advent of Code 2022 - Rust

Rust solutions to Advent of Code 2022

## Project structure

The project is structured using Cargo's Workspaces feature. A single shared lib crate contains common shared functionality. Each day's solution is implemented as a separate binary crate.

- `data/day-{n}`: Input data for daily problems
- `shared`: A shared library of common functionality across problems
- `problems/day-{n}`: Various binary crates with daily solutions

## Build and Run

Assumes that you have `cargo` installed and available on your path.

- `cargo run -p day-{n}`: Run the solution for day `n`

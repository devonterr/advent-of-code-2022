# Create new binary package
cargo new --bin problems/day-$1

# Setup data dir
mkdir data/day-$1
touch data/day-$1/test.txt
touch data/day-$1/input.txt

# Append shared lib to new package
echo "shared = { path = \"../../shared\"}" >> problems/day-$1/Cargo.toml

# Prepend imports to new main.rs
echo "use shared::{read_lines, AoCProblem, AoCSolution, Solution};\n\n$(cat problems/day-$1/src/main.rs)" > problems/day-$1/src/main.rs

# Build to validate
cargo build -p day-$1

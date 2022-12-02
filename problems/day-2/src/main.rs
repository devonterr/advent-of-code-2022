use shared::read_lines;

static INPUT_FILE_NAME: &str = "data/day-2/input.txt";
static TEST_INPUT_FILE_NAME: &str = "data/day-2/test.txt";

fn normalize(play: char) -> String {
    match play {
        'A' | 'X' => "Rock".to_owned(),
        'B' | 'Y' => "Paper".to_owned(),
        'C' | 'Z' => "Scissors".to_owned(),
        _ => panic!("Bad data"),
    }
}

fn score_outcome(play: &str, opponent: &str) -> i32 {
    match (play, opponent) {
        ("Rock", "Paper") => 0,
        ("Paper", "Scissors") => 0,
        ("Scissors", "Rock") => 0,
        ("Rock", "Scissors") => 6,
        ("Paper", "Rock") => 6,
        ("Scissors", "Paper") => 6,
        (x, y) if x == y => 3,
        _ => panic!("Should never happen!"),
    }
}

fn score_play(play: &str) -> i32 {
    match play {
        "Rock" => 1,
        "Paper" => 2,
        "Scissors" => 3,
        _ => panic!("Shouldn't happen"),
    }
}

fn score(opponent: char, play: char) -> i32 {
    let normal_play = normalize(play);
    let normal_opponent = normalize(opponent);
    let scored_outcome = score_outcome(&normal_play, &normal_opponent);
    let scored_play = score_play(&normal_play);
    return scored_outcome + scored_play;
}

fn outcome_to_play(opponent: char, outcome: char) -> char {
    match (opponent, outcome) {
        (p, 'Y') => p,
        ('A', 'X') => 'C',
        ('B', 'X') => 'A',
        ('C', 'X') => 'B',
        ('A', 'Z') => 'B',
        ('B', 'Z') => 'C',
        ('C', 'Z') => 'A',
        _ => panic!("Shouldn't happen"),
    }
}

fn main() {
    let lines: Vec<(String, String)> = read_lines(INPUT_FILE_NAME)
        .expect("Should be able to read input file")
        .map(|line| {
            line.expect("Should read")
                .split(" ")
                .map(|p| p.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|ps| (ps[0].clone(), ps[1].clone()))
        .collect();
    let scores: Vec<i32> = lines
        .iter()
        .map(|line| {
            score(
                line.0.chars().next().unwrap(),
                line.1.chars().next().unwrap(),
            )
        })
        .collect();
    let total_score: i32 = scores.iter().sum();
    println!("Total by Strategy: {}", total_score);

    let scores2: Vec<i32> = lines
        .iter()
        .map(|pair| {
            (
                pair.0.chars().next().unwrap(),
                outcome_to_play(
                    pair.0.chars().next().unwrap(),
                    pair.1.chars().next().unwrap(),
                ),
            )
        })
        .map(|line| score(line.0, line.1))
        .collect();
    let total_score2: i32 = scores2.iter().sum();
    println!("Total by Strategy 2: {}", total_score2);
}

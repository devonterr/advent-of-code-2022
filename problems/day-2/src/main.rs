use shared::{read_lines, AoCProblem, AoCSolution, Solution};

#[derive(Clone)]
enum Outcome {
    Win,
    Draw,
    Loss,
}
impl Outcome {
    fn score(&self) -> i32 {
        match &self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

#[derive(PartialEq, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    fn score(&self) -> i32 {
        match &self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    fn beats(&self) -> Play {
        match &self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }
    fn loses_to(&self) -> Play {
        match &self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }
}

struct Round {
    player_play: Play,
    outcome: Outcome,
}
impl Round {
    fn score(&self) -> i32 {
        &self.player_play.score() + &self.outcome.score()
    }
}

impl TryFrom<char> for Play {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Play::Rock),
            'B' | 'Y' => Ok(Play::Paper),
            'C' | 'Z' => Ok(Play::Scissors),
            _ => Err("Unrecognized play".to_owned()),
        }
    }
}

impl TryFrom<(Outcome, Play)> for Play {
    type Error = String;

    fn try_from(value: (Outcome, Play)) -> Result<Self, Self::Error> {
        match value {
            (Outcome::Win, k) => Ok(k.loses_to()),
            (Outcome::Loss, k) => Ok(k.beats()),
            (Outcome::Draw, k) => Ok(k),
        }
    }
}

// (Opponent, Player)
impl TryFrom<&(Play, Play)> for Outcome {
    type Error = String;

    fn try_from(value: &(Play, Play)) -> Result<Self, Self::Error> {
        match value {
            (x, y) if x == y => Ok(Outcome::Draw),
            (x, y) if x.beats() == *y => Ok(Outcome::Loss),
            (x, y) if x.loses_to() == *y => Ok(Outcome::Win),
            _ => Err("Unrecognized play combination".to_owned()),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Unrecognized outcome".to_owned()),
        }
    }
}

impl TryFrom<String> for Round {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<char> = value
            .trim()
            .split(' ')
            .map(|p| p.chars().next().expect("Unexpected format"))
            .collect();
        let opponent_play: Play = Play::try_from(parts[0])?;
        let player_play: Play = Play::try_from(parts[1])?;
        let outcome: Outcome = Outcome::try_from(&(opponent_play, player_play.clone()))?;
        Ok(Round {
            player_play,
            outcome,
        })
    }
}

struct Round2(Round);
impl TryFrom<String> for Round2 {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<char> = value
            .trim()
            .split(' ')
            .map(|p| p.chars().next().expect("Unexpected format"))
            .collect();
        let opponent_play: Play = Play::try_from(parts[0])?;
        let outcome: Outcome = Outcome::try_from(parts[1])?;
        let player_play: Play = Play::try_from((outcome.clone(), opponent_play))?;
        Ok(Round2(Round {
            player_play,
            outcome,
        }))
    }
}

struct Day2 {}
impl AoCProblem for Day2 {
    fn name(&self) -> String {
        "day-2".to_owned()
    }
}
impl Solution for Day2 {
    fn solution(&self, path: &str) {
        let file = path;
        let scores: Vec<i32> = read_lines(file)
            .expect("Should be able to read input file")
            .map(|line| {
                Round::try_from(line.expect("Should be able to read line"))
                    .expect("Should be able to parse Round")
            })
            .map(|round| round.score())
            .collect();
        // println!("{:#?}", scores);
        println!("Score {}", scores.iter().sum::<i32>());

        let scores2: Vec<i32> = read_lines(file)
            .expect("Should be able to read input file")
            .map(|line| {
                Round2::try_from(line.expect("Should be able to read line"))
                    .expect("Should be able to parse Round")
            })
            .map(|round| match round {
                Round2(r) => r.score(),
            })
            .collect();
        // println!("{:#?}", scores2);
        println!("Score2 {}", scores2.iter().sum::<i32>());
    }
}

fn main() {
    Day2 {}.test_and_run();
}

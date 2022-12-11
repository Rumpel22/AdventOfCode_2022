use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Invalid character to parse"),
        }
    }
}

impl Move {
    fn inherent_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn outcome(&self, theirs: &Move) -> Outcome {
        if self.beats(theirs) {
            Outcome::Win
        } else if theirs.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    fn beats(&self, theirs: &Move) -> bool {
        matches!(
            (self, theirs),
            (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper)
        )
    }
}

struct Round {
    theirs: Move,
    ours: Move,
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut chars = input.chars();
        let (Some(theirs), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Invalid input for parsing round.");
        };
        Ok(Self {
            theirs: theirs.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Round {
    fn outcome(&self) -> Outcome {
        self.ours.outcome(&self.theirs)
    }
    fn our_score(&self) -> u32 {
        self.ours.inherent_points() + self.outcome().inherent_points()
    }

    fn with_outcome(theirs: Move, outcome: &Outcome) -> Round {
        let ours = match (theirs, outcome) {
            (_, Outcome::Draw) => theirs,
            (Move::Rock, Outcome::Loss) => Move::Scissors,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Loss) => Move::Rock,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Loss) => Move::Paper,
            (Move::Scissors, Outcome::Win) => Move::Rock,
        };
        Round { theirs, ours }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn inherent_points(self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err("Invalid character for parsing to an Outcome."),
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let solution1: u32 = input
        .lines()
        .filter_map(|line| line.parse::<Round>().ok())
        .map(|round| round.our_score())
        .sum();

    let re = Regex::new(r"^([A|B|C]) ([X|Y|Z])$").unwrap();
    let solution2: u32 = input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                std::convert::TryInto::<Move>::try_into(
                    cap.get(1).unwrap().as_str().chars().next().unwrap(),
                )
                .unwrap(),
                std::convert::TryInto::<Outcome>::try_into(
                    cap.get(2).unwrap().as_str().chars().next().unwrap(),
                )
                .unwrap(),
            )
        })
        .map(|(theirs, outcome)| Round::with_outcome(theirs, &outcome))
        .map(|round| round.our_score())
        .sum();

    println!("Solution 1: {solution1}, solution 2: {solution2}");
}

use std::{fs, path::Path};

#[derive(Clone)]
enum Opponent {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum MyShape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn should_play(op: &Opponent, out: &Outcome, first_solution: bool) -> MyShape {
    if first_solution {
        match out {
            Outcome::Lose => MyShape::Rock,
            Outcome::Draw => MyShape::Paper,
            Outcome::Win => MyShape::Scissors,
        }
    } else {
        match (op, out) {
            (Opponent::Rock, Outcome::Lose) => MyShape::Scissors,
            (Opponent::Rock, Outcome::Draw) => MyShape::Rock,
            (Opponent::Rock, Outcome::Win) => MyShape::Paper,
            (Opponent::Paper, Outcome::Lose) => MyShape::Rock,
            (Opponent::Paper, Outcome::Draw) => MyShape::Paper,
            (Opponent::Paper, Outcome::Win) => MyShape::Scissors,
            (Opponent::Scissors, Outcome::Lose) => MyShape::Paper,
            (Opponent::Scissors, Outcome::Draw) => MyShape::Scissors,
            (Opponent::Scissors, Outcome::Win) => MyShape::Rock,
        }
    }
}

fn result_points(op: &Opponent, my: &MyShape) -> i32 {
    match (op, my) {
        (Opponent::Rock, MyShape::Paper)
        | (Opponent::Paper, MyShape::Scissors)
        | (Opponent::Scissors, MyShape::Rock) => 6,
        (Opponent::Rock, MyShape::Rock)
        | (Opponent::Paper, MyShape::Paper)
        | (Opponent::Scissors, MyShape::Scissors) => 3,
        _ => 0,
    }
}

fn shape_points(my: &MyShape) -> i32 {
    match my {
        MyShape::Rock => 1,
        MyShape::Paper => 2,
        MyShape::Scissors => 3,
    }
}

fn main() {
    let path = Path::new("src/bin/day_02/input.txt");
    let input = fs::read_to_string(path).unwrap();
    let points: i32 = input
        .lines()
        .map(|line| {
            (
                match line.chars().next().unwrap() {
                    'A' => Opponent::Rock,
                    'B' => Opponent::Paper,
                    'C' => Opponent::Scissors,
                    _ => panic!("Invalid input"),
                },
                match line.chars().nth(2).unwrap() {
                    'X' => Outcome::Lose,
                    'Y' => Outcome::Draw,
                    'Z' => Outcome::Win,
                    _ => panic!("Invalid input"),
                },
            )
        })
        .map(|(op, out)| (op.clone(), should_play(&op, &out, true)))
        .map(|(op, my)| result_points(&op, &my) + shape_points(&my))
        .sum();
    println!("Solution 2 {points}");
}

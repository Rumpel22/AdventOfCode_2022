use regex::Regex;
use std::{collections::HashSet, iter, str::FromStr};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Command {
    direction: Direction,
    count: u8,
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new("^(U|D|L|R) (\\d+)$").unwrap();
        let command = match rx.captures(line) {
            Some(captures) => {
                let count = captures[2].parse::<u8>().unwrap();
                let direction = match captures[1].chars().next().unwrap() {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => return Err("Something went really wrong."),
                };

                Command { direction, count }
            }
            None => return Err("Invalid line"),
        };

        Ok(command)
    }
}

fn get_new_tail(head: &Position, tail: &Position) -> Position {
    let horizontal_diff = head.x - tail.x;
    let vertical_diff = head.y - tail.y;
    if horizontal_diff == 2 {
        Position {
            x: head.x - 1,
            y: head.y,
        }
    } else if horizontal_diff == -2 {
        Position {
            x: head.x + 1,
            y: head.y,
        }
    } else if vertical_diff == 2 {
        Position {
            x: head.x,
            y: head.y - 1,
        }
    } else if vertical_diff == -2 {
        Position {
            x: head.x,
            y: head.y + 1,
        }
    } else {
        *tail
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let commands = input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect::<Vec<_>>();

    let tails = commands
        .iter()
        .flat_map(|command| iter::repeat(command.direction).take(command.count as usize))
        .scan(
            (Position::default(), Position::default()),
            |(head, tail), direction| {
                match direction {
                    Direction::Up => head.y += 1,
                    Direction::Down => head.y -= 1,
                    Direction::Left => head.x -= 1,
                    Direction::Right => head.x += 1,
                };
                *tail = get_new_tail(head, tail);
                Some(*tail)
            },
        )
        .collect::<HashSet<_>>();

    println!("The tail is at {} different positions", tails.len());
}

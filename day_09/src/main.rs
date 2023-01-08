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

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

type Rope = Vec<Position>;

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split_ascii_whitespace();
        let direction = match parts.next().unwrap().chars().next() {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => return Err("Something went really wrong."),
        };
        let count = parts.next().unwrap().parse::<u8>().unwrap();

        Ok(Command { direction, count })
    }
}

fn get_new_knot(head: &Position, tail: &Position) -> Position {
    let horizontal_diff = head.x - tail.x;
    let vertical_diff = head.y - tail.y;
    assert!(horizontal_diff <= 2);
    assert!(vertical_diff <= 2);

    if horizontal_diff.abs() == 2 && vertical_diff.abs() == 2 {
        Position {
            x: tail.x + horizontal_diff.signum(),
            y: tail.y + vertical_diff.signum(),
        }
    } else if horizontal_diff.abs() == 2 {
        Position {
            x: tail.x + horizontal_diff.signum(),
            y: head.y,
        }
    } else if vertical_diff.abs() == 2 {
        Position {
            x: head.x,
            y: tail.y + vertical_diff.signum(),
        }
    } else {
        *tail
    }
}

fn update_rope(rope: &mut Rope) {
    let head = rope[0];
    rope.iter_mut()
        .skip(1)
        .scan(head, |prev, knot| {
            *knot = get_new_knot(prev, knot);
            *prev = *knot;
            Some(*knot)
        })
        .for_each(|_| {});
}

fn main() {
    let input = include_str!("../data/input.txt");

    let commands = input
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .collect::<Vec<_>>();

    let mut rope = Rope::new();
    rope.resize(10, Position::default());

    let tails = commands
        .iter()
        .flat_map(|command| iter::repeat(command.direction).take(command.count as usize))
        .scan(rope, |rope, direction| {
            let head = rope.first_mut().unwrap();
            match direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            };
            update_rope(rope);
            Some(*(rope.last()).unwrap())
        })
        .collect::<HashSet<_>>();

    println!("The tail is at {} different positions", tails.len());
}

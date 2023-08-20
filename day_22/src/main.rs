use regex::Regex;

#[derive(Clone, Copy, PartialEq)]
enum Turn {
    Left,
    Right,
}

enum Command {
    Move(u8),
    Turn(Turn),
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match self {
            Direction::Left if turn == Turn::Left => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Right if turn == Turn::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up if turn == Turn::Left => Direction::Left,
            Direction::Up => Direction::Right,
            Direction::Down if turn == Turn::Left => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

struct Map(Vec<Vec<Option<Tile>>>);

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

struct PositionIterator<'a> {
    direction: Direction,
    position: Position,
    map: &'a Map,
}

impl Iterator for PositionIterator<'_> {
    type Item = (Position, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.position = match self.direction {
                Direction::Left => Position {
                    x: if self.position.x > 1 {
                        self.position.x - 1
                    } else {
                        self.map.width(self.position.y)
                    },
                    y: self.position.y,
                },
                Direction::Right => Position {
                    x: if self.position.x < self.map.width(self.position.y) {
                        self.position.x + 1
                    } else {
                        1
                    },
                    y: self.position.y,
                },
                Direction::Up => Position {
                    x: self.position.x,
                    y: if self.position.y > 1 {
                        self.position.y - 1
                    } else {
                        self.map.height()
                    },
                },
                Direction::Down => Position {
                    x: self.position.x,
                    y: if self.position.y < self.map.height() {
                        self.position.y + 1
                    } else {
                        1
                    },
                },
            };
            if self.map.get(&self.position).is_some() {
                break;
            }
        }

        Some((self.position, self.direction))
    }
}

impl Map {
    fn walk(&self, steps: u8, position: Position, direction: Direction) -> (Position, Direction) {
        self.iter(position, direction)
            .take(steps.into())
            .take_while(|(position, _)| self.get(position).unwrap() == Tile::Open)
            .last()
            .unwrap_or((position, direction))
    }

    fn start_position(&self) -> Position {
        let x = self.0[0].iter().position(|tile| tile.is_some()).unwrap();
        Position { x: x + 1, y: 1 }
    }

    fn iter(&self, position: Position, direction: Direction) -> PositionIterator {
        PositionIterator {
            map: self,
            direction,
            position,
        }
    }

    fn get(&self, position: &Position) -> Option<Tile> {
        *self
            .0
            .get((position.y - 1) as usize)
            .and_then(|row| row.get((position.x - 1) as usize))
            .unwrap_or(&None)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self, row: usize) -> usize {
        self.0[row - 1].len()
    }
}

fn parse_commands(line: &str) -> Vec<Command> {
    let regex = Regex::new("(L|R|[0-9]+)").unwrap();
    regex
        .find_iter(line)
        .map(|chars| {
            let chars = chars.as_str();
            match chars.chars().next().unwrap() {
                'L' => Command::Turn(Turn::Left),
                'R' => Command::Turn(Turn::Right),
                c if c.is_ascii_digit() => Command::Move(chars.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse_line(line: &str) -> Vec<Option<Tile>> {
    line.chars()
        .map(|c| match c {
            '.' => Some(Tile::Open),
            '#' => Some(Tile::Wall),
            ' ' => None,
            _ => unreachable!(),
        })
        .collect()
}

fn parse(input: &str) -> (Map, Vec<Command>) {
    let map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let command_line = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .next()
        .unwrap();
    let commands = parse_commands(command_line);
    (Map(map), commands)
}

fn main() {
    let input = include_str!("../data/input.txt");

    let (map, commands) = parse(input);

    let position = map.start_position();
    let (position, direction) = commands.iter().fold(
        (position, Direction::Right),
        |(position, direction), command| match command {
            Command::Move(steps) => map.walk(*steps, position, direction),
            Command::Turn(orientation) => (position, direction.turn(*orientation)),
        },
    );

    let direction_value = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
        Direction::Down => 1,
    };
    let password = 1000 * position.y + 4 * position.x + direction_value;

    println!("The password is {password}");
}

use std::marker::PhantomData;

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

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

trait Wrapper {
    fn wrap(map: &Map, position: &Position, direction: &Direction) -> (Position, Direction);
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

struct PositionIterator<'a, W>
where
    W: Wrapper,
{
    direction: Direction,
    position: Position,
    map: &'a Map,
    phantom: std::marker::PhantomData<W>,
}

impl<'a, W> Iterator for PositionIterator<'a, W>
where
    W: Wrapper,
{
    type Item = (Position, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        self.position = match self.direction {
            Direction::Left => Position {
                x: self.position.x - 1,
                y: self.position.y,
            },
            Direction::Right => Position {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Direction::Up => Position {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Direction::Down => Position {
                x: self.position.x,
                y: self.position.y + 1,
            },
        };
        if self.map.get(&self.position).is_none() {
            (self.position, self.direction) = W::wrap(self.map, &self.position, &self.direction);
            assert!(self.map.get(&self.position).is_some())
        }

        Some((self.position, self.direction))
    }
}

impl Map {
    fn row_min(&self, row_number: usize) -> Option<usize> {
        self.0.get(row_number - 1).and_then(|row| {
            row.iter()
                .position(|tile| tile.is_some())
                .map(|index| index + 1)
        })
    }
    fn row_max(&self, row_number: usize) -> Option<usize> {
        self.0.get(row_number - 1).and_then(|row| {
            row.iter()
                .rposition(|tile| tile.is_some())
                .map(|index| index + 1)
        })
    }
    fn col_min(&self, col: usize) -> Option<usize> {
        self.0
            .iter()
            .position(|row| row.get(col - 1).unwrap_or(&None).is_some())
            .map(|index| index + 1)
    }
    fn col_max(&self, col: usize) -> Option<usize> {
        self.0
            .iter()
            .rposition(|row| row.get(col - 1).unwrap_or(&None).is_some())
            .map(|index| index + 1)
    }

    fn width(&self) -> usize {
        self.0.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn get(&self, position: &Position) -> Option<Tile> {
        if position.x < 1 || position.y < 1 {
            return None;
        }

        *self
            .0
            .get(position.y - 1)
            .and_then(|row| row.get(position.x - 1))
            .unwrap_or(&None)
    }
}

struct FlatWrapper;

impl Wrapper for FlatWrapper {
    fn wrap(map: &'_ Map, position: &Position, direction: &Direction) -> (Position, Direction) {
        let position = if direction == &Direction::Left
            && Some(position.x).lt(&map.row_min(position.y))
        {
            Position {
                x: map.row_max(position.y).unwrap(),
                y: position.y,
            }
        } else if direction == &Direction::Right && Some(position.x).gt(&map.row_max(position.y)) {
            Position {
                x: map.row_min(position.y).unwrap(),
                y: position.y,
            }
        } else if direction == &Direction::Up && Some(position.y).lt(&map.col_min(position.x)) {
            Position {
                x: position.x,
                y: map.col_max(position.x).unwrap(),
            }
        } else if direction == &Direction::Down && Some(position.y).gt(&map.col_max(position.x)) {
            Position {
                x: position.x,
                y: map.col_min(position.x).unwrap(),
            }
        } else {
            unreachable!();
        };
        (position, *direction)
    }
}

struct CubeWrapper;

impl CubeWrapper {
    fn square_size(map: &Map) -> usize {
        ((map.0.iter().flatten().filter(|tile| tile.is_some()).count() / 6) as f64).sqrt() as usize
    }
}

impl Wrapper for CubeWrapper {
    fn wrap(map: &Map, position: &Position, direction: &Direction) -> (Position, Direction) {
        let square_size = Self::square_size(map);

        let offset_x = position.x % square_size;
        let left_x = Some(position.x - offset_x);
        let right_x = Some(position.x - offset_x + square_size + 1);

        let offset_y = position.y % square_size;
        let up_y = Some(position.y - offset_y);
        let down_y = Some(position.y - offset_y + square_size + 1);

        for turn in [Turn::Left, Turn::Right] {
            let new_direction = direction.turn(turn);
            let (new_x, new_y) = match (direction, new_direction) {
                (Direction::Left, Direction::Up) => (position.x.checked_sub(offset_y), up_y),
                (Direction::Left, Direction::Down) => (position.x.checked_sub(offset_y), down_y),
                (Direction::Right, Direction::Up) => (position.x.checked_add(offset_y), up_y),
                (Direction::Right, Direction::Down) => (position.x.checked_add(offset_y), down_y),
                (Direction::Up, Direction::Left) => (left_x, position.y.checked_sub(offset_x)),
                (Direction::Up, Direction::Right) => (right_x, position.y.checked_sub(offset_x)),
                (Direction::Down, Direction::Left) => (left_x, position.y.checked_add(offset_x)),
                (Direction::Down, Direction::Right) => (right_x, position.y.checked_add(offset_x)),
                _ => unreachable!(),
            };

            let new_position = new_x.and_then(|x| new_y.map(|y| Position { x, y }));
            if new_position.is_none() {
                continue;
            }
            let new_position = new_position.unwrap();

            if map.get(&new_position).is_some() {
                return (new_position, new_direction);
            }
        }
        (*position, *direction)
    }
}

fn parse_commands(line: &str) -> Vec<Command> {
    let regex = Regex::new("(L|R|[0-9]+)").unwrap();
    regex
        .find_iter(line)
        .map(|chars| match chars.as_str() {
            "L" => Command::Turn(Turn::Left),
            "R" => Command::Turn(Turn::Right),
            s => Command::Move(s.parse().unwrap()),
        })
        .collect()
}

fn parse_map(input: &str) -> Map {
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

    let map = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Vec<_>>();
    Map(map)
}

fn parse(input: &str) -> (Map, Vec<Command>) {
    let map = parse_map(input);

    let command_line = input.lines().last().unwrap();
    let commands = parse_commands(command_line);

    (map, commands)
}

fn start_position(map: &Map) -> Position {
    let x = map.0[0].iter().position(|tile| tile.is_some()).unwrap();
    Position { x: x + 1, y: 1 }
}

fn execute_commands<W>(commands: &[Command], map: &Map) -> (Position, Direction)
where
    W: Wrapper,
{
    commands.iter().fold(
        (start_position(map), Direction::Right),
        |(position, direction), command| match command {
            Command::Move(steps) => walk::<W>(map, *steps, position, direction),
            Command::Turn(orientation) => (position, direction.turn(*orientation)),
        },
    )
}

fn get_password(position: Position, direction: Direction) -> usize {
    let direction_value = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
        Direction::Down => 1,
    };
    1000 * position.y + 4 * position.x + direction_value
}

fn walk<W>(map: &Map, steps: u8, position: Position, direction: Direction) -> (Position, Direction)
where
    W: Wrapper,
{
    let iterator = PositionIterator::<'_, W> {
        map,
        position,
        direction,
        phantom: PhantomData,
    };

    iterator
        .take(steps.into())
        .take_while(|(position, _)| map.get(position).unwrap() == Tile::Open)
        .last()
        .unwrap_or((position, direction))
}

fn main() {
    let input = include_str!("../data/demo_input.txt");

    let (map, commands) = parse(input);

    let (position, direction) = execute_commands::<FlatWrapper>(&commands, &map);
    let password = get_password(position, direction);

    println!("The password for the flat map is {password}");

    let (position, direction) = execute_commands::<CubeWrapper>(&commands, &map);
    let password = get_password(position, direction);

    println!("The password for the cube map is {password}");
}

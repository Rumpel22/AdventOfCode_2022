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
    fn wrap(&self, position: &Position, direction: &Direction) -> (Position, Direction);
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
            (self.position, self.direction) = self.map.wrap(&self.position, &self.direction);
            assert!(self.map.get(&self.position).is_some())
        }

        Some((self.position, self.direction))
    }
}

impl Map {
    fn iter(&self, position: Position, direction: Direction) -> PositionIterator {
        PositionIterator {
            map: self,
            direction,
            position,
        }
    }

    fn row_min(&self, row_number: usize) -> Option<usize> {
        self.0.get(row_number - 1).and_then(|row| {
            row.iter()
                .enumerate()
                .find(|(_, tile)| tile.is_some())
                .map(|(index, _)| index + 1)
        })
    }
    fn row_max(&self, row_number: usize) -> Option<usize> {
        self.0.get(row_number - 1).and_then(|row| {
            row.iter()
                .enumerate()
                .rfind(|(_, tile)| tile.is_some())
                .map(|(index, _)| index + 1)
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

impl Wrapper for Map {
    fn wrap(&self, position: &Position, direction: &Direction) -> (Position, Direction) {
        let position = if direction == &Direction::Left
            && Some(position.x).lt(&self.row_min(position.y))
        {
            Position {
                x: self.row_max(position.y).unwrap(),
                y: position.y,
            }
        } else if direction == &Direction::Right && Some(position.x).gt(&self.row_max(position.y)) {
            Position {
                x: self.row_min(position.y).unwrap(),
                y: position.y,
            }
        } else if direction == &Direction::Up && Some(position.y).lt(&self.col_min(position.x)) {
            Position {
                x: position.x,
                y: self.col_max(position.x).unwrap(),
            }
        } else if direction == &Direction::Down && Some(position.y).gt(&self.col_max(position.x)) {
            Position {
                x: position.x,
                y: self.col_min(position.x).unwrap(),
            }
        } else {
            unreachable!();
        };
        (position, *direction)
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

struct Executor<W> {
    walker: W,
}

impl<'a, W> Executor<W>
where
    W: Walker<'a>,
{
    fn execute_commands(&self, commands: &Vec<Command>, map: &'a Map) -> (Position, Direction) {
        commands.iter().fold(
            (start_position(map), Direction::Right),
            |(position, direction), command| match command {
                Command::Move(steps) => self.walker.walk(map, *steps, position, direction),
                Command::Turn(orientation) => (position, direction.turn(*orientation)),
            },
        )
    }
}
fn get_password(position: Position, direction: Direction) -> usize {
    let direction_value = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Up => 3,
        Direction::Down => 1,
    };
    let password = 1000 * position.y + 4 * position.x + direction_value;
    password
}

struct FlatWalker {}

trait Walker<'a> {
    fn walk(
        &self,
        map: &'a Map,
        steps: u8,
        position: Position,
        direction: Direction,
    ) -> (Position, Direction) {
        let iterator = Self::iter(map, position, direction);

        iterator
            .take(steps.into())
            .take_while(|(position, _)| map.get(position).unwrap() == Tile::Open)
            .last()
            .unwrap_or((position, direction))
    }

    fn iter(
        map: &'a Map,
        position: Position,
        direction: Direction,
    ) -> Box<dyn 'a + Iterator<Item = (Position, Direction)>>;
}

impl<'a> Walker<'a> for FlatWalker {
    fn iter(
        map: &'a Map,
        position: Position,
        direction: Direction,
    ) -> Box<dyn 'a + Iterator<Item = (Position, Direction)>> {
        Box::new(PositionIterator::<'a> {
            map,
            direction,
            position,
        })
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let (map, commands) = parse(input);

    let walker = Executor {
        walker: FlatWalker {},
    };

    let (position, direction) = walker.execute_commands(&commands, &map);
    let password = get_password(position, direction);

    println!("The password for the flat map is {password}");

    // let cube = Cube::new(&map);

    // let (position, direction) = execute_commands(&commands, &cube);
    // let password = get_password(position, direction);

    // println!("The password for the cube map is {password}");
}

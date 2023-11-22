use std::{collections::HashMap, iter};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Direction::North => Some(Self::South),
            Direction::South => Some(Self::West),
            Direction::West => Some(Self::East),
            Direction::East => Some(Self::North),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum AreaPiece {
    Elf,
    Empty,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinates {
    x: i16,
    y: i16,
}

impl Coordinates {
    fn neighbor(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Coordinates {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Coordinates {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Coordinates {
                x: self.x - 1,
                y: self.y,
            },
            Direction::East => Coordinates {
                x: self.x + 1,
                y: self.y,
            },
            Direction::NorthWest => Coordinates {
                x: self.x - 1,
                y: self.y - 1,
            },
            Direction::NorthEast => Coordinates {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::SouthWest => Coordinates {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::SouthEast => Coordinates {
                x: self.x + 1,
                y: self.y + 1,
            },
        }
    }
}

struct Area {
    // x: RangeInclusive<i16>,
    // y: RangeInclusive<i16>,
    elfs: Vec<Coordinates>,
}

impl Area {
    fn get(&self, coordinates: Coordinates) -> AreaPiece {
        // if !self.y.contains(&coordinates.y) || self.x.contains(&coordinates.x) {
        //     return AreaPiece::Empty;
        // }
        if self.elfs.iter().any(|elf| elf == &coordinates) {
            AreaPiece::Elf
        } else {
            AreaPiece::Empty
        }
    }

    fn get_neighbors(&self, coordinates: &Coordinates) -> HashMap<Direction, AreaPiece> {
        let mut neighbors = HashMap::new();
        neighbors.insert(
            Direction::NorthWest,
            self.get(coordinates.neighbor(Direction::NorthWest)),
        );
        neighbors.insert(
            Direction::North,
            self.get(coordinates.neighbor(Direction::North)),
        );
        neighbors.insert(
            Direction::NorthEast,
            self.get(coordinates.neighbor(Direction::NorthEast)),
        );
        neighbors.insert(
            Direction::West,
            self.get(coordinates.neighbor(Direction::West)),
        );
        neighbors.insert(
            Direction::East,
            self.get(coordinates.neighbor(Direction::East)),
        );
        neighbors.insert(
            Direction::SouthWest,
            self.get(coordinates.neighbor(Direction::SouthWest)),
        );
        neighbors.insert(
            Direction::South,
            self.get(coordinates.neighbor(Direction::South)),
        );
        neighbors.insert(
            Direction::SouthEast,
            self.get(coordinates.neighbor(Direction::SouthEast)),
        );
        neighbors
    }

    fn min_y(&self) -> i16 {
        self.elfs.iter().map(|elf| elf.y).min().unwrap_or(0)
    }
    fn max_y(&self) -> i16 {
        self.elfs.iter().map(|elf| elf.y).max().unwrap_or(0)
    }
    fn min_x(&self) -> i16 {
        self.elfs.iter().map(|elf| elf.x).min().unwrap_or(0)
    }
    fn max_x(&self) -> i16 {
        self.elfs.iter().map(|elf| elf.x).max().unwrap_or(0)
    }

    fn height(&self) -> i16 {
        self.max_y() - self.min_y() + 1
    }
    fn width(&self) -> i16 {
        self.max_x() - self.min_x() + 1
    }
    fn area(&self) -> i16 {
        self.height() * self.width()
    }
}

fn parse(input: &str) -> Area {
    let elfs = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Coordinates {
                    x: x as i16,
                    y: y as i16,
                }),

                _ => None,
            })
        })
        .collect::<Vec<_>>();
    // let y_min = 0;
    // let y_max = input.lines().count() as i16;
    // let x_min = 0;
    // let x_max = input.len() as i16 / y_max;
    Area { elfs }
}

struct Proposition {
    origin: Coordinates,
    target: Coordinates,
}

fn can_walk(direction: Direction, neighbors: &HashMap<Direction, AreaPiece>) -> bool {
    let checks = match direction {
        Direction::North => [Direction::North, Direction::NorthEast, Direction::NorthWest],
        Direction::South => [Direction::South, Direction::SouthEast, Direction::SouthWest],
        Direction::West => [Direction::NorthWest, Direction::SouthWest, Direction::West],
        Direction::East => [Direction::East, Direction::NorthEast, Direction::SouthEast],
        _ => unreachable!(),
    };
    checks
        .iter()
        .all(|direction| neighbors[direction] == AreaPiece::Empty)
}

fn get_propositions(area: &Area, direction: Direction) -> Vec<Proposition> {
    area.elfs
        .iter()
        .map(|elf| {
            let neighbors = area.get_neighbors(elf);
            if neighbors
                .values()
                .all(|neighbor| neighbor == &AreaPiece::Empty)
            {
                return Proposition {
                    origin: *elf,
                    target: *elf,
                };
            }
            let walk_direction =
                iter::successors(Some(direction), |direction| direction.clone().next())
                    .take(4)
                    .filter(|actual_direction| can_walk(*actual_direction, &neighbors))
                    .next();
            Proposition {
                origin: *elf,
                target: match walk_direction {
                    Some(direction) => elf.neighbor(direction),
                    None => *elf,
                },
            }
        })
        .collect()
}

fn merge_propositions(propositions: &Vec<Proposition>) -> Area {
    let mut x = HashMap::new();
    for proposition in propositions {
        if let Some(old) = x.insert(proposition.target, proposition) {
            x.remove(&proposition.target);
            x.insert(proposition.origin, proposition);
            x.insert(old.origin, old);
        }
    }
    let elfs = x.keys().cloned().collect();
    Area { elfs }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let mut area = parse(input);
    let mut next_direction =
        iter::successors(Some(Direction::North), |direction| direction.clone().next());

    for turn in 1..=10 {
        let propositions = get_propositions(&area, next_direction.next().unwrap());
        area = merge_propositions(&propositions);
    }
    println!("{} x {} = {}", area.height(), area.width(), area.area());
    println!(
        "{} - {} = {}",
        area.area(),
        area.elfs.len(),
        area.area() - area.elfs.len() as i16
    )
}

use std::ops::RangeInclusive;

enum Direction {
    North,
    South,
    West,
    East,
}

impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Direction::North => Some(Self::South),
            Direction::South => Some(Self::West),
            Direction::West => Some(Self::East),
            Direction::East => Some(Self::North),
        }
    }
}

#[derive(Clone, Copy)]
enum AreaPiece {
    Elf,
    Empty,
}

#[derive(Hash, Eq, PartialEq)]
struct Coordinates {
    x: i16,
    y: i16,
}

struct Area {
    x: RangeInclusive<i16>,
    y: RangeInclusive<i16>,
    data: Vec<AreaPiece>,
}

impl Area {
    fn get(&self, x: i16, y: i16) -> AreaPiece {
        let index = (y - self.min_y()) * self.width() + (x - self.min_x());
        *self.data.get(index as usize).unwrap_or(&AreaPiece::Empty)
    }

    fn min_y(&self) -> i16 {
        *self.y.start()
    }
    fn max_y(&self) -> i16 {
        *self.y.end()
    }
    fn min_x(&self) -> i16 {
        *self.x.start()
    }
    fn max_x(&self) -> i16 {
        *self.x.end()
    }

    fn height(&self) -> i16 {
        self.max_y() - self.min_y()
    }
    fn width(&self) -> i16 {
        self.max_x() - self.min_x()
    }
    fn area(&self) -> i16 {
        self.height() * self.width()
    }
}

fn parse(input: &str) -> Area {
    let data = input
        .lines()
        .flat_map(|row| {
            row.chars().map(|c| match c {
                '#' => AreaPiece::Elf,
                '.' => AreaPiece::Empty,
                _ => unreachable!(),
            })
        })
        .collect::<Vec<_>>();
    let x_min = 0;
    let x_max = input.lines().next().map(|row| row.len()).unwrap_or(0) as i16;
    let y_min = 0;
    let y_max = data.len() as i16 / x_max;
    Area {
        x: x_min..=x_max,
        y: y_min..=y_max,
        data,
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let area = parse(input);

    for turn in 1..=10 {
        // let propositions = get_propositions(area, direction);
        // let area = merge_propositions(propositions);
    }
    println!("{} x {} = {}", area.height(), area.width(), area.area());
}

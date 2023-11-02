use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate(usize, usize);

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split(',').map(|s| s.parse::<usize>().unwrap());
        Ok(Self(x.next().unwrap(), x.next().unwrap()))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Unit {
    Rock,
    Air,
    Sand,
}

impl Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Air => write!(f, "."),
            Self::Sand => write!(f, "o"),
        }
    }
}

struct Map {
    fields: HashMap<Coordinate, Unit>,
    x_limits: (usize, usize),
    y_limits: (usize, usize),
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.y_limits.1 {
            for x in self.x_limits.0..=self.x_limits.1 {
                write!(
                    f,
                    "{:?}",
                    &self.fields.get(&Coordinate(x, y)).unwrap_or(&Unit::Air)
                )?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Index<Coordinate> for Map {
    type Output = Unit;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.fields.get(&index).unwrap_or(&Unit::Air)
    }
}

impl IndexMut<Coordinate> for Map {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.fields.entry(index).or_insert(Unit::Air)
    }
}

impl Map {
    fn new(wall_units: &[Coordinate]) -> Self {
        let x_limits = wall_units
            .iter()
            .map(|coordinate| coordinate.0)
            .minmax()
            .into_option()
            .unwrap();
        let y_limits = wall_units
            .iter()
            .map(|coordinate| coordinate.1)
            .minmax()
            .into_option()
            .unwrap();
        Self {
            fields: HashMap::from_iter(
                wall_units
                    .iter()
                    .map(|coordinate| (*coordinate, Unit::Rock)),
            ),
            x_limits,
            y_limits,
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let walls = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.parse::<Coordinate>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let wall_units = walls
        .iter()
        .flat_map(|wall| {
            wall.as_slice().windows(2).flat_map(|wall_part| {
                let start = wall_part[0];
                let end = wall_part[1];

                let x_range = match start.0 <= end.0 {
                    true => start.0..=end.0,
                    false => end.0..=start.0,
                };
                let y_range = match start.1 <= end.1 {
                    true => start.1..=end.1,
                    false => end.1..=start.1,
                };

                x_range
                    .cartesian_product(y_range)
                    .map(|(x, y)| Coordinate(x, y))
            })
        })
        .collect::<Vec<_>>();

    let mut map = Map::new(&wall_units);
    let mut sand_units = 0;

    loop {
        let mut current = Coordinate(500, 0);
        loop {
            let down = Coordinate(current.0, current.1 + 1);
            let left = Coordinate(current.0 - 1, current.1 + 1);
            let right = Coordinate(current.0 + 1, current.1 + 1);
            if map[down] == Unit::Air {
                current = down;
            } else if map[left] == Unit::Air {
                current = left;
            } else if map[right] == Unit::Air {
                current = right;
            } else {
                map[current] = Unit::Sand;
                break;
            }
            if current.1 == map.y_limits.1 + 1 {
                map[current] = Unit::Sand;
                break;
            }
        }
        if current.1 == 0 {
            sand_units += 1;
            break;
        }

        sand_units += 1;
    }

    println!("There are {sand_units} units of sand.");
}

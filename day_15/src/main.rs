use std::{collections::HashSet, str::FromStr};

use regex::Regex;

struct Coordinate {
    x: i64,
    y: i64,
}
impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[2..].split(", y=");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Ok(Self { x, y })
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Option<Self> {
        match start <= end {
            true => Some(Self { start, end }),
            false => None,
        }
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn merge(&self, range: &[Range]) -> Vec<Range> {
        let mut merged = false;
        let mut new_ranges = Vec::with_capacity(range.len());
        for r in range {
            if r.start > self.end || r.end < self.start {
                new_ranges.push(*r);
            } else {
                if merged {
                    new_ranges = r.merge(&new_ranges)
                } else {
                    merged = true;
                    new_ranges
                        .push(Self::new(self.start.min(r.start), self.end.max(r.end)).unwrap());
                }
            };
        }
        if !merged {
            new_ranges.push(*self);
        }
        new_ranges
    }

    fn contains(&self, number: i64) -> bool {
        number >= self.start && number <= self.end
    }
}

struct Sensor {
    position: Coordinate,
    distance: i64,
}

struct Beacon {
    position: Coordinate,
}

fn main() {
    let input = include_str!("../data/input.txt");
    let rx = Regex::new(r"(x=-?\d*, y=-?\d*)").unwrap();

    let (sensors, beacons): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut captures = rx.find_iter(line);
            let sensor_coordinate = captures
                .next()
                .unwrap()
                .as_str()
                .parse::<Coordinate>()
                .unwrap();
            let beacon_coordinate = captures
                .next()
                .unwrap()
                .as_str()
                .parse::<Coordinate>()
                .unwrap();
            (sensor_coordinate, beacon_coordinate)
        })
        .map(|(sensor_coordinate, beacon_coordinate)| {
            let distance = (sensor_coordinate.x - beacon_coordinate.x).abs()
                + (sensor_coordinate.y - beacon_coordinate.y).abs();
            (
                Sensor {
                    position: sensor_coordinate,
                    distance,
                },
                Beacon {
                    position: beacon_coordinate,
                },
            )
        })
        .unzip();
    let row = 2000000;
    let beacons_on_row = beacons
        .iter()
        .map(|beacon| beacon.position.y)
        .filter(|y_coordinate| y_coordinate == &row)
        .collect::<HashSet<_>>()
        .len();
    let ranges_on_row = sensors
        .iter()
        .filter_map(|sensor| {
            let remaining_x = sensor.distance - (sensor.position.y - row).abs();
            Range::new(
                sensor.position.x - remaining_x,
                sensor.position.x + remaining_x,
            )
        })
        .fold(Vec::<Range>::default(), |vec, range| range.merge(&vec));

    let uniques_on_row =
        ranges_on_row.iter().fold(0, |sum, range| sum + range.len()) - beacons_on_row;

    println!("There are {} covered fields on row {row}", uniques_on_row);
}

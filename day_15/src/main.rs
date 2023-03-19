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

    fn contains(&self, number: i64) -> bool {
        number >= self.start && number <= self.end
    }

    fn limit(&self, lower: i64, upper: i64) -> Self {
        Self {
            start: self.start.max(lower),
            end: self.end.min(upper),
        }
    }
}

#[derive(Default)]
struct Ranges(Vec<Range>);

impl Ranges {
    fn len(&self) -> usize {
        self.0.iter().map(|range| range.len()).sum()
    }

    fn merge(&self, range: &Range) -> Ranges {
        let mut merged = false;
        let mut new_ranges = Ranges::default();
        for r in &self.0 {
            if range.start > r.end || range.end < r.start {
                new_ranges.0.push(*r);
            } else {
                if merged {
                    new_ranges = new_ranges.merge(r);
                } else {
                    merged = true;
                    new_ranges
                        .0
                        .push(Range::new(range.start.min(r.start), range.end.max(r.end)).unwrap());
                }
            };
        }
        if !merged {
            new_ranges.0.push(*range);
        }
        new_ranges
    }

    fn contains(&self, number: i64) -> bool {
        self.0.iter().any(|range| range.contains(number))
    }
}

struct Sensor {
    position: Coordinate,
    distance: u64,
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
            let distance = sensor_coordinate.x.abs_diff(beacon_coordinate.x)
                + sensor_coordinate.y.abs_diff(beacon_coordinate.y);
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
            let remaining_x = sensor.distance as i64 - sensor.position.y.abs_diff(row) as i64;
            Range::new(
                sensor.position.x - remaining_x as i64,
                sensor.position.x + remaining_x as i64,
            )
        })
        .fold(Ranges::default(), |ranges, range| ranges.merge(&range));

    let uniques_on_row = ranges_on_row.len() - beacons_on_row;

    println!("There are {} covered fields on row {row}", uniques_on_row);

    for row in 0..=4000000 {
        let ranges_on_row = sensors
            .iter()
            .filter_map(|sensor| {
                let remaining_x = sensor.distance as i64 - sensor.position.y.abs_diff(row) as i64;
                Range::new(
                    sensor.position.x - remaining_x as i64,
                    sensor.position.x + remaining_x as i64,
                )
            })
            .fold(Ranges::default(), |ranges, range| {
                ranges.merge(&range.limit(0, 4000000))
            });

        if ranges_on_row.len() != 4000001 {
            let x = (0..=4000000)
                .filter(|number| !ranges_on_row.contains(*number))
                .next()
                .unwrap();
            println!("Found uncovered field at x={x}, y={row}.");
            println!(
                "The tuning frequency for this field is {}",
                x * 4000000 + row
            );
            break;
        }
    }
}

use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
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
    let uniques_on_rows = sensors
        .iter()
        .flat_map(|sensor| {
            let remaining_x = sensor.distance - (sensor.position.y - row).abs();
            (sensor.position.x - remaining_x)..=(sensor.position.x + remaining_x)
        })
        .unique()
        .count()
        - beacons_on_row;

    println!("There are {} covered fields on row {row}", uniques_on_rows);
}

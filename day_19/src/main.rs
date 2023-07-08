use std::str::FromStr;

use regex::Regex;

enum Unit {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Cost {
    amount: i32,
    unit: Unit,
}

struct Blueprint {
    id: i32,
    cost_per_ore_robot: Cost,
    cost_per_clay_robot: Cost,
    cost_per_obisidan_robot: [Cost; 2],
    cost_per_geode_robot: [Cost; 2],
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new("([0-9]+)").unwrap();
        let mut numbers = regex
            .captures_iter(s)
            .map(|x| x.extract())
            .map(|(_, x)| x.iter().map(|y| y.parse::<i32>().unwrap()))
            .flatten();

        let id = numbers.next().unwrap();
        let cost_per_ore_robot = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Ore,
        };
        let cost_per_clay_robot = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Ore,
        };
        let cost_per_obisdian_robot_1 = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Ore,
        };
        let cost_per_obisdian_robot_2 = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Clay,
        };
        let cost_per_geode_robot_1 = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Ore,
        };
        let cost_per_geode_robot_2 = Cost {
            amount: numbers.next().unwrap(),
            unit: Unit::Obsidian,
        };

        let blueprint = Blueprint {
            id,
            cost_per_ore_robot,
            cost_per_clay_robot,
            cost_per_obisidan_robot: [cost_per_obisdian_robot_1, cost_per_obisdian_robot_2],
            cost_per_geode_robot: [cost_per_geode_robot_1, cost_per_geode_robot_2],
        };

        Ok(blueprint)
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();
}

use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
    vec,
};

use regex::Regex;

enum Unit {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Cost {
    amount: u32,
    unit: Unit,
}

struct Blueprint {
    id: u32,
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
            .find_iter(s)
            .map(|x| x.as_str())
            .map(|s| s.parse::<u32>().unwrap());

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

#[derive(Default)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obisidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn initial() -> Self {
        Self {
            geode_robots: 1,
            ..Self::default()
        }
    }

    fn get_successors(&self, blueprint: &Blueprint) -> Vec<Self> {
        let time = self.time + 1;
        if time >= 25 {
            return vec![];
        }

        let mut ore = self.ore + self.ore_robots;
        let mut clay = self.clay + self.clay_robots;
        let mut obsidian = self.obisidian + self.obsidian_robots;
        let geode = self.geode + self.geode_robots;
        vec![]
    }
}

fn evaluate_blueprint(blueprint: &Blueprint) -> u32 {
    let mut max_geodes = 0_u32;
    let mut states = VecDeque::from([State::initial()]);
    while let Some(state) = states.pop_front() {
        max_geodes = max_geodes.max(state.geode);

        let next_states = state.get_successors(blueprint);

        states.extend(next_states);
    }

    max_geodes
}

fn main() {
    let input = include_str!("../data/input.txt");
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let geodes = blueprints
        .iter()
        .map(|blueprint| (blueprint.id, evaluate_blueprint(blueprint)))
        .collect::<HashMap<_, _>>();

    let quality_level = geodes
        .iter()
        .map(|(id, max_geodes)| id * max_geodes)
        .max()
        .unwrap();

    println!("Max geodes in 24 minutes: {}", quality_level);
}

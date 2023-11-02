use std::collections::{HashMap, VecDeque};

use std::str::FromStr;

use regex::Regex;

#[derive(Clone, Copy)]
enum Unit {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Blueprint {
    id: u32,
    cost_per_ore_robot: u32,
    cost_per_clay_robot: u32,
    cost_per_obisidan_robot: [u32; 2],
    cost_per_geode_robot: [u32; 2],
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
        let cost_per_ore_robot = numbers.next().unwrap();
        let cost_per_clay_robot = numbers.next().unwrap();
        let cost_per_obisdian_robot_1 = numbers.next().unwrap();
        let cost_per_obisdian_robot_2 = numbers.next().unwrap();
        let cost_per_geode_robot_1 = numbers.next().unwrap();
        let cost_per_geode_robot_2 = numbers.next().unwrap();

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

#[derive(Default, Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Hash)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn div_ceil(lhs: u32, rhs: u32) -> u32 {
    (lhs + rhs - 1) / rhs
}

impl State {
    fn initial() -> Self {
        Self {
            ore_robots: 1,
            ..Self::default()
        }
    }

    fn time_till_robot(&self, unit: Unit, blueprint: &Blueprint) -> u32 {
        match unit {
            Unit::Ore => {
                if self.ore >= blueprint.cost_per_ore_robot {
                    0
                } else {
                    div_ceil(
                        blueprint.cost_per_ore_robot.saturating_sub(self.ore),
                        self.ore_robots,
                    )
                }
            }
            Unit::Clay => {
                if self.ore >= blueprint.cost_per_clay_robot {
                    0
                } else {
                    div_ceil(
                        blueprint.cost_per_clay_robot.saturating_sub(self.ore),
                        self.ore_robots,
                    )
                }
            }
            Unit::Obsidian => {
                if self.ore >= blueprint.cost_per_obisidan_robot[0]
                    && self.clay >= blueprint.cost_per_obisidan_robot[1]
                {
                    0
                } else {
                    let ore_time = div_ceil(
                        blueprint.cost_per_obisidan_robot[0].saturating_sub(self.ore),
                        self.ore_robots,
                    );
                    let clay_time = div_ceil(
                        blueprint.cost_per_obisidan_robot[1].saturating_sub(self.clay),
                        self.clay_robots,
                    );
                    ore_time.max(clay_time)
                }
            }
            Unit::Geode => {
                if self.ore >= blueprint.cost_per_geode_robot[0]
                    && self.obsidian >= blueprint.cost_per_geode_robot[1]
                {
                    0
                } else {
                    let ore_time = div_ceil(
                        blueprint.cost_per_geode_robot[0].saturating_sub(self.ore),
                        self.ore_robots,
                    );
                    let obsidian_time = div_ceil(
                        blueprint.cost_per_geode_robot[1].saturating_sub(self.obsidian),
                        self.obsidian_robots,
                    );
                    ore_time.max(obsidian_time)
                }
            }
        }
    }

    fn can_buy_robot(&self, robot: Unit) -> bool {
        match robot {
            Unit::Ore => self.ore_robots > 0,
            Unit::Clay => self.ore_robots > 0,
            Unit::Obsidian => self.ore_robots > 0 && self.clay_robots > 0,
            Unit::Geode => self.ore_robots > 0 && self.obsidian_robots > 0,
        }
    }

    fn should_buy(&self, robot: Unit, blueprint: &Blueprint) -> bool {
        let factor = 2;
        match robot {
            Unit::Ore => self.ore < factor * blueprint.cost_per_ore_robot,
            Unit::Clay => self.ore < 2 * blueprint.cost_per_clay_robot,
            Unit::Obsidian => true,
            Unit::Geode => true,
        }
    }

    fn buy_robot(&self, robot: Unit, blueprint: &Blueprint) -> Option<State> {
        if !self.can_buy_robot(robot) {
            return None;
        }
        if !self.should_buy(robot, blueprint) {
            return None;
        }

        let needed_time = self.time_till_robot(robot, blueprint) + 1;
        let mut next_state = self.progress(needed_time);

        match robot {
            Unit::Ore => {
                next_state.ore -= blueprint.cost_per_ore_robot;
                next_state.ore_robots += 1;
            }
            Unit::Clay => {
                next_state.ore -= blueprint.cost_per_clay_robot;
                next_state.clay_robots += 1;
            }
            Unit::Obsidian => {
                next_state.ore -= blueprint.cost_per_obisidan_robot[0];
                next_state.clay -= blueprint.cost_per_obisidan_robot[1];
                next_state.obsidian_robots += 1;
            }
            Unit::Geode => {
                next_state.ore -= blueprint.cost_per_geode_robot[0];
                next_state.obsidian -= blueprint.cost_per_geode_robot[1];
                next_state.geode_robots += 1;
            }
        }

        Some(next_state)
    }

    fn progress(&self, time: u32) -> Self {
        Self {
            time: self.time + time,
            ore: self.ore + time * self.ore_robots,
            clay: self.clay + time * self.clay_robots,
            obsidian: self.obsidian + time * self.obsidian_robots,
            geode: self.geode + time * self.geode_robots,
            ..*self
        }
    }
}

#[allow(dead_code)]
pub(crate) fn evaluate(blueprint: &Blueprint, max_time: u32) -> u32 {
    let mut max_geodes = 0_u32;
    let mut states = VecDeque::from([State::initial()]);
    while let Some(state) = states.pop_front() {
        max_geodes = max_geodes.max(state.geode);

        if state.time >= max_time {
            continue;
        }

        for robot in [Unit::Ore, Unit::Clay, Unit::Obsidian, Unit::Geode] {
            if let Some(mut next_state) = state.buy_robot(robot, blueprint) {
                if next_state.time > max_time {
                    next_state = state.progress(max_time - state.time);
                }

                states.push_back(next_state);
            }
        }
    }

    max_geodes
}

fn main() {
    let input = include_str!("../data/input.txt");
    let max_time = 32;

    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .take(3)
        .collect::<Vec<_>>();

    let geodes = blueprints
        .iter()
        .map(|blueprint| (blueprint.id, evaluate(blueprint, max_time)))
        .collect::<HashMap<_, _>>();

    let quality_level = geodes
        .iter()
        .map(|(id, max_geodes)| id * max_geodes)
        .sum::<u32>();

    println!(
        "The quality level after {} minutes: {}",
        max_time, quality_level
    );

    let product = geodes.values().product::<u32>();

    println!(
        "The product of the geodes amount after {} minutes: {}",
        max_time, product
    );
}

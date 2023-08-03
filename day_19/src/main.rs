use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
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

impl Blueprint {
    fn buy_robots(&self, old_state: &State, new_state: &State) -> HashSet<State> {
        let mut states = HashSet::new();

        if !self.can_buy_robot(Unit::Ore, &old_state) && self.can_buy_robot(Unit::Ore, &new_state) {
            let state = State {
                ore: new_state.ore - self.cost_per_ore_robot.amount,
                ore_robots: new_state.ore_robots + 1,
                ..*new_state
            };
            states.insert(state);
            states.extend(self.buy_robots(&old_state, &state).iter());
        }
        if !self.can_buy_robot(Unit::Clay, &old_state) && self.can_buy_robot(Unit::Clay, &new_state)
        {
            let state = State {
                ore: new_state.ore - self.cost_per_clay_robot.amount,
                clay_robots: new_state.clay_robots + 1,
                ..*new_state
            };
            states.insert(state);
            states.extend(self.buy_robots(&old_state, &state).iter());
        }
        if !self.can_buy_robot(Unit::Obsidian, &old_state)
            && self.can_buy_robot(Unit::Obsidian, &new_state)
        {
            let state = State {
                ore: new_state.ore - self.cost_per_obisidan_robot[0].amount,
                clay: new_state.clay - self.cost_per_obisidan_robot[1].amount,
                obsidian_robots: new_state.obsidian_robots + 1,
                ..*new_state
            };
            states.insert(state);
            states.extend(self.buy_robots(&old_state, &state).iter());
        }
        if !self.can_buy_robot(Unit::Geode, &old_state)
            && self.can_buy_robot(Unit::Geode, &new_state)
        {
            let state = State {
                ore: new_state.ore - self.cost_per_geode_robot[0].amount,
                obsidian: new_state.obsidian - self.cost_per_geode_robot[1].amount,
                geode_robots: new_state.geode_robots + 1,
                ..*new_state
            };
            states.insert(state);
            states.extend(self.buy_robots(&old_state, &state).iter());
        }

        states
    }

    fn can_buy_robot(&self, unit: Unit, state: &State) -> bool {
        match unit {
            Unit::Ore => state.ore >= self.cost_per_ore_robot.amount,
            Unit::Clay => state.ore >= self.cost_per_clay_robot.amount,
            Unit::Obsidian => {
                state.ore >= self.cost_per_obisidan_robot[0].amount
                    && state.clay >= self.cost_per_obisidan_robot[1].amount
            }
            Unit::Geode => {
                state.ore >= self.cost_per_geode_robot[0].amount
                    && state.obsidian >= self.cost_per_geode_robot[1].amount
            }
        }
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

impl State {
    fn initial() -> Self {
        Self {
            ore_robots: 1,
            ..Self::default()
        }
    }

    fn get_successors(&self, blueprint: &Blueprint) -> HashSet<Self> {
        if self.time >= 24 {
            return HashSet::new();
        }

        let next_state = Self {
            time: self.time + 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geode: self.geode + self.geode_robots,
            ..*self
        };

        let mut successors = blueprint.buy_robots(&self, &next_state);
        successors.insert(next_state);
        successors
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
    let input = include_str!("../data/demo_input.txt");
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

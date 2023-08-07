use std::str::FromStr;

use regex::Regex;

#[derive(Clone, Copy)]
pub(crate) enum Unit {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

pub(crate) struct Cost {
    pub(crate) amount: u32,
    pub(crate) unit: Unit,
}

pub(crate) struct Blueprint {
    pub(crate) id: u32,
    pub(crate) cost_per_ore_robot: Cost,
    pub(crate) cost_per_clay_robot: Cost,
    pub(crate) cost_per_obisidan_robot: [Cost; 2],
    pub(crate) cost_per_geode_robot: [Cost; 2],
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

use std::collections::VecDeque;

use crate::blueprint::*;

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
                if self.ore >= blueprint.cost_per_ore_robot.amount {
                    0
                } else {
                    div_ceil(
                        blueprint.cost_per_ore_robot.amount.saturating_sub(self.ore),
                        self.ore_robots,
                    )
                }
            }
            Unit::Clay => {
                if self.ore >= blueprint.cost_per_clay_robot.amount {
                    0
                } else {
                    div_ceil(
                        blueprint
                            .cost_per_clay_robot
                            .amount
                            .saturating_sub(self.ore),
                        self.ore_robots,
                    )
                }
            }
            Unit::Obsidian => {
                if self.ore >= blueprint.cost_per_obisidan_robot[0].amount
                    && self.clay >= blueprint.cost_per_obisidan_robot[1].amount
                {
                    0
                } else {
                    let ore_time = div_ceil(
                        blueprint.cost_per_obisidan_robot[0]
                            .amount
                            .saturating_sub(self.ore),
                        self.ore_robots,
                    );
                    let clay_time = div_ceil(
                        blueprint.cost_per_obisidan_robot[1]
                            .amount
                            .saturating_sub(self.clay),
                        self.clay_robots,
                    );
                    ore_time.max(clay_time)
                }
            }
            Unit::Geode => {
                if self.ore >= blueprint.cost_per_geode_robot[0].amount
                    && self.obsidian >= blueprint.cost_per_geode_robot[1].amount
                {
                    0
                } else {
                    let ore_time = div_ceil(
                        blueprint.cost_per_geode_robot[0]
                            .amount
                            .saturating_sub(self.ore),
                        self.ore_robots,
                    );
                    let obsidian_time = div_ceil(
                        blueprint.cost_per_geode_robot[1]
                            .amount
                            .saturating_sub(self.obsidian),
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
            Unit::Ore => self.ore < factor * blueprint.cost_per_ore_robot.amount,
            Unit::Clay => self.ore < 2 * blueprint.cost_per_clay_robot.amount,
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
                next_state.ore -= blueprint.cost_per_ore_robot.amount;
                next_state.ore_robots += 1;
            }
            Unit::Clay => {
                next_state.ore -= blueprint.cost_per_clay_robot.amount;
                next_state.clay_robots += 1;
            }
            Unit::Obsidian => {
                next_state.ore -= blueprint.cost_per_obisidan_robot[0].amount;
                next_state.clay -= blueprint.cost_per_obisidan_robot[1].amount;
                next_state.obsidian_robots += 1;
            }
            Unit::Geode => {
                next_state.ore -= blueprint.cost_per_geode_robot[0].amount;
                next_state.obsidian -= blueprint.cost_per_geode_robot[1].amount;
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

use std::collections::{HashSet, VecDeque};

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

impl Blueprint {
    fn buy_robot(&self, unit: Unit, state: &State) -> State {
        let mut state = *state;
        match unit {
            Unit::Ore => {
                state.ore -= self.cost_per_ore_robot.amount;
                state.ore_robots += 1;
            }
            Unit::Clay => {
                state.ore -= self.cost_per_clay_robot.amount;
                state.clay_robots += 1;
            }
            Unit::Obsidian => {
                state.ore -= self.cost_per_obisidan_robot[0].amount;
                state.clay -= self.cost_per_obisidan_robot[1].amount;
                state.obsidian_robots += 1;
            }
            Unit::Geode => {
                state.ore -= self.cost_per_geode_robot[0].amount;
                state.obsidian -= self.cost_per_geode_robot[1].amount;
                state.geode_robots += 1;
            }
        }
        state
    }

    fn should_buy_robot(&self, unit: Unit, state: &State) -> bool {
        match unit {
            Unit::Ore => {
                state.ore
                    < self.cost_per_ore_robot.amount.max(
                        self.cost_per_clay_robot.amount.max(
                            self.cost_per_obisidan_robot[0]
                                .amount
                                .max(self.cost_per_geode_robot[0].amount),
                        ),
                    )
            }
            Unit::Clay => state.clay < self.cost_per_obisidan_robot[1].amount,
            Unit::Obsidian => state.obsidian < self.cost_per_geode_robot[1].amount,
            Unit::Geode => true,
        }
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

        let mut states = HashSet::new();

        if blueprint.should_buy_robot(Unit::Geode, &self)
            && blueprint.can_buy_robot(Unit::Geode, &self)
        {
            states.insert(blueprint.buy_robot(Unit::Geode, &self));
        }
        if blueprint.should_buy_robot(Unit::Obsidian, &self)
            && blueprint.can_buy_robot(Unit::Obsidian, &self)
        {
            states.insert(blueprint.buy_robot(Unit::Obsidian, &self));
        }
        if blueprint.should_buy_robot(Unit::Clay, &self)
            && blueprint.can_buy_robot(Unit::Clay, &self)
        {
            states.insert(blueprint.buy_robot(Unit::Clay, &self));
        }
        if blueprint.should_buy_robot(Unit::Ore, &self) && blueprint.can_buy_robot(Unit::Ore, &self)
        {
            states.insert(blueprint.buy_robot(Unit::Ore, &self));
        }

        states.insert(*self);

        let states = states
            .iter()
            .cloned()
            .map(|mut state| {
                state.time += 1;
                state.ore += self.ore_robots;
                state.clay += self.clay_robots;
                state.obsidian += self.obsidian_robots;
                state.geode += self.geode_robots;
                state
            })
            .collect();

        states
    }
}

#[allow(dead_code)]
pub(crate) fn evaluate(blueprint: &Blueprint) -> u32 {
    let mut max_geodes = 0_u32;
    let mut states = VecDeque::from([State::initial()]);
    while let Some(state) = states.pop_front() {
        max_geodes = max_geodes.max(state.geode);

        let next_states = state.get_successors(blueprint);

        states.extend(next_states);
    }

    max_geodes
}

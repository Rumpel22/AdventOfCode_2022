use std::collections::HashMap;

use regex::Regex;

struct Valve {
    flow_rate: u8,
    neighbors: Vec<&'static str>,
}

#[derive(Clone)]
struct State {
    remaining_time: u16,
    cumulated_pressure: u16,
    pressure_per_minute: u16,
    closed_valves: Vec<&'static str>,
    room: &'static str,
}

impl State {
    fn potential_pressure(&self, valves: &HashMap<&str, Valve>) -> u16 {
        self.closed_valves
            .iter()
            .map(|valve| valves[*valve].flow_rate as u16)
            .sum::<u16>()
            * self.remaining_time
            + self.pressure_per_minute * self.remaining_time
            + self.cumulated_pressure
    }
}

impl Valve {
    fn parse(line: &'static str) -> Option<(&'static str, Self)> {
        let rx = Regex::new(r"^Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .ok()?;
        let mut captures = rx.captures_iter(line);
        let captures = captures.next()?;

        let name = captures.get(1).and_then(|c| Some(c.as_str()))?;
        let flow_rate = captures
            .get(2)
            .and_then(|c| c.as_str().parse::<u8>().ok())?;
        let neighbors = captures
            .get(3)
            .and_then(|c| Some(c.as_str().split(", ").collect::<Vec<_>>()))?;

        Some((
            name,
            Self {
                flow_rate,
                neighbors,
            },
        ))
    }
}

fn main() {
    let input = include_str!("../data/demo_input.txt");
    let valves = input
        .lines()
        .filter_map(|line| Valve::parse(line))
        .collect::<HashMap<&str, Valve>>();
    let closed_valves = valves
        .iter()
        .filter_map(|(key, value)| match value.flow_rate {
            0 => None,
            _ => Some(*key),
        })
        .collect::<Vec<_>>();

    let initial_state = State {
        remaining_time: 30,
        cumulated_pressure: 0,
        pressure_per_minute: 0,
        closed_valves,
        room: "AA",
    };

    let mut states = vec![initial_state];
    let mut max = u16::MIN;

    while !states.is_empty() {
        let state = states.pop().unwrap();
        let room = state.room;

        max = max.max(state.cumulated_pressure);

        if state.closed_valves.contains(&room) && state.remaining_time > 0 {
            let mut remaining_valves = state.closed_valves.clone();
            remaining_valves.retain(|valve| valve != &room);
            states.push(State {
                remaining_time: state.remaining_time - 1,
                pressure_per_minute: state.pressure_per_minute
                    + valves[&state.room].flow_rate as u16,
                cumulated_pressure: state.cumulated_pressure + state.pressure_per_minute,
                closed_valves: remaining_valves,
                room,
            })
        }

        if state.remaining_time > 0 {
            for neighbor in &valves[room].neighbors {
                states.push(State {
                    room: neighbor,
                    remaining_time: state.remaining_time - 1,
                    closed_valves: state.closed_valves.clone(),
                    pressure_per_minute: state.pressure_per_minute,
                    cumulated_pressure: state.cumulated_pressure + state.pressure_per_minute,
                });
            }
        }

        states.retain(|state| state.potential_pressure(&valves) >= max);
        states.sort_unstable_by(|a, b| a.pressure_per_minute.cmp(&b.pressure_per_minute));
    }

    println!("The max. pressure released is {}", max);
}

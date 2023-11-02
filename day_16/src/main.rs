use std::collections::{BinaryHeap, HashMap};

use regex::Regex;

#[derive(Clone, Copy)]
enum Action<'a> {
    Move(&'a str),
    Open(u8),
}

#[derive(Clone)]
struct Valve<'a> {
    flow_rate: u8,
    neighbors: Vec<&'a str>,
}
impl<'a> Valve<'a> {
    fn parse(line: &'a str) -> Option<(&'a str, Self)> {
        let rx = Regex::new(r"^Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .ok()?;
        let mut captures = rx.captures_iter(line);
        let captures = captures.next()?;

        let name = captures.get(1).map(|c| c.as_str())?;
        let flow_rate = captures
            .get(2)
            .and_then(|c| c.as_str().parse::<u8>().ok())?;
        let neighbors = captures
            .get(3)
            .map(|c| c.as_str().split(", ").collect::<Vec<_>>())?;

        Some((
            name,
            Self {
                flow_rate,
                neighbors,
            },
        ))
    }
}

struct State<'a> {
    room: &'a str,
    action: Action<'a>,
    minute: u8,
    cumulated: u16,
    flow_per_minute: u16,
    // closed_valves: Vec<&'a str>,
    opened_valves: Vec<&'a str>,
}

fn get_move_actions<'a>(
    map: &HashMap<&'a str, Valve<'a>>,
    valve: &'a str,
    last_room: &'a str,
) -> Vec<Action<'a>> {
    map[valve]
        .neighbors
        .iter()
        .filter(|&&room| room != last_room)
        .map(|neighbor| Action::Move(neighbor))
        .collect()
}

impl Eq for State<'_> {}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
            && self.cumulated == other.cumulated
            && self.flow_per_minute == other.flow_per_minute
            && self.room == other.room
            && self.opened_valves == other.opened_valves
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.minute.partial_cmp(&other.minute) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cumulated.partial_cmp(&other.cumulated) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.flow_per_minute.partial_cmp(&other.flow_per_minute) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.room.partial_cmp(other.room) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.opened_valves.partial_cmp(&other.opened_valves)
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_path<'a>(valves: &HashMap<&'a str, Valve<'a>>, max_time: u8) -> (u16, Vec<&'a str>) {
    let mut states = get_move_actions(valves, "AA", "")
        .iter()
        .map(|action| State {
            room: "AA",
            cumulated: 0,
            minute: 0,
            flow_per_minute: 0,
            // closed_valves,
            opened_valves: vec![],
            action: *action,
        })
        .collect::<BinaryHeap<_>>();

    let mut max_released_pressure = 0;
    let mut path = vec![];

    while !states.is_empty() {
        let state = states.pop().unwrap();
        if state.cumulated > max_released_pressure {
            max_released_pressure = state.cumulated;
            path = state.opened_valves.clone();
        }

        let minutes_left = max_time - state.minute;
        let max_pressure_approx = state.cumulated + minutes_left as u16 * state.flow_per_minute * 2;

        // Some options are treated as "not good" and are skipped:
        // - The first 15 (or 13) minutes, every possible solution is taken into consideration
        // - Afterwards, states which have not released enough pressure yet are skipped. A state is not viable, if another
        // solution has already collected more pressure than this solution would, even if it has doubled is flow per minute immediately.
        if minutes_left > 0
            && (max_pressure_approx > max_released_pressure || state.minute <= max_time / 2)
        {
            let cumulated = state.cumulated + state.flow_per_minute;
            let minute = state.minute + 1;
            let (room, flow_per_minute, opened_valves) = match state.action {
                Action::Move(new_room) => (new_room, state.flow_per_minute, state.opened_valves),
                Action::Open(flow) => {
                    let mut opened_valves = state.opened_valves.clone();
                    opened_valves.push(state.room);
                    (
                        state.room,
                        state.flow_per_minute + flow as u16,
                        opened_valves,
                    )
                }
            };

            let mut actions = get_move_actions(valves, room, state.room);
            if valves[room].flow_rate > 0 && !opened_valves.contains(&room) {
                actions.push(Action::Open(valves[room].flow_rate));
            }
            let x = actions.iter().map(|action| State {
                room,
                cumulated,
                minute,
                flow_per_minute,
                opened_valves: opened_valves.clone(),
                action: *action,
            });
            states.extend(x);
        }
    }

    (max_released_pressure, path)
}

fn main() {
    let input = include_str!("../data/input.txt");
    let valves: HashMap<&str, Valve> = input.lines().filter_map(Valve::parse).collect();

    let (max_released_pressure, path) = find_path(&valves, 30);
    println!("============ PART I ============");
    println!("The max. pressure released is {}", max_released_pressure);
    println!("The path is {:?}", path);

    let (max_released_pressure_1, path_1) = find_path(&valves, 26);
    println!("============ PART II ============");
    println!("The max. pressure released is {}", max_released_pressure_1);
    println!("The path is {:?}", path_1);

    // This solution works for my real input, because the elephant can open valves on a complete different branch of the
    // tunnels. It does not work, if the same valves should be opened by two different openers.
    let valves_filtered: HashMap<&str, Valve> = valves
        .iter()
        .map(|(name, valve)| {
            (
                *name,
                Valve {
                    flow_rate: if path_1.contains(name) {
                        0
                    } else {
                        valve.flow_rate
                    },
                    ..valve.clone()
                },
            )
        })
        .collect();
    let (max_released_pressure_2, path_2) = find_path(&valves_filtered, 26);

    println!("The max. pressure released is {}", max_released_pressure_2);
    println!("The path is {:?}", path_2);

    println!(
        "The combined max. pressure released is {}",
        max_released_pressure_1 + max_released_pressure_2
    );
}

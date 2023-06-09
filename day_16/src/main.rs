use std::collections::{BinaryHeap, HashMap};

use regex::Regex;

struct Valve<'a> {
    name: &'a str,
    flow_rate: u8,
    neighbors: Vec<&'a str>,
}
impl<'a> Valve<'a> {
    fn parse(line: &'a str) -> Option<Self> {
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

        Some(Self {
            name,
            flow_rate,
            neighbors,
        })
    }
}

struct DistanceMap<'a>(HashMap<&'a str, HashMap<&'a str, u8>>);

impl<'a> DistanceMap<'a> {
    fn new(valves: &[Valve<'a>]) -> Self {
        let mut map = HashMap::<&str, HashMap<&str, u8>>::with_capacity(valves.len());

        for valve in valves {
            let mut inner_map = HashMap::<&str, u8>::with_capacity(valves.len());
            inner_map.insert(valve.name, 0);
            let mut next = valve.neighbors.clone();
            let mut distance = 1;

            while inner_map.len() < valves.len() {
                next = next
                    .iter()
                    .flat_map(|&other| {
                        if inner_map.contains_key(other) {
                            vec![]
                        } else {
                            inner_map.insert(other, distance);

                            valves
                                .iter()
                                .find(|&v| v.name == other)
                                .unwrap()
                                .neighbors
                                .clone()
                        }
                    })
                    .collect::<Vec<_>>();
                distance += 1;
            }

            map.insert(valve.name, inner_map);
        }

        Self(map)
    }

    fn distance(&self, from: &str, to: &str) -> u8 {
        self.0[from][to]
    }
}

struct State<'a> {
    room: &'a str,
    time: u8,
    cumulated: u16,
    closed_valves: Vec<&'a str>,
    opened_valves: Vec<&'a str>,
}

impl<'a> State<'a> {
    fn get_nexts(&self, flow: u8, distance_map: &DistanceMap<'a>) -> Vec<State<'a>> {
        self.closed_valves
            .iter()
            .map(|new_room| {
                let closed_valves = self
                    .closed_valves
                    .iter()
                    .filter(|&room| room != new_room)
                    .map(|room| *room)
                    .collect::<Vec<_>>();
                let distance = distance_map.distance(self.room, new_room);
                let mut time = self.time.saturating_sub(distance);
                if flow > 0 && time > 0 {
                    time = time.saturating_sub(1);
                }
                let cumulated = self.cumulated + flow as u16 * (self.time - 1) as u16;

                let mut opened_valves = self.opened_valves.clone();
                opened_valves.push(self.room);

                Self {
                    room: new_room,
                    time,
                    cumulated,
                    closed_valves,
                    opened_valves,
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Eq for State<'_> {}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.cumulated == other.cumulated
            && self.room == other.room
            && self.closed_valves == other.closed_valves
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cumulated.partial_cmp(&other.cumulated) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.room.partial_cmp(&other.room) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.closed_valves.partial_cmp(&other.closed_valves)
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Flows<'a>(HashMap<&'a str, u8>);

fn main() {
    let input = include_str!("../data/input.txt");
    let valves = input
        .lines()
        .filter_map(|line| Valve::parse(line))
        .collect::<Vec<_>>();
    let closed_valves = valves
        .iter()
        .filter_map(|valve| match valve.flow_rate {
            0 => None,
            _ => Some(valve.name),
        })
        .collect::<Vec<_>>();

    let distance_map = DistanceMap::new(&valves);
    let flows = Flows(
        valves
            .iter()
            .map(|valve| (valve.name, valve.flow_rate))
            .collect(),
    );

    let initial_option = State {
        cumulated: 0,
        time: 26,
        closed_valves,
        room: "AA",
        opened_valves: vec![],
    };

    let mut open_options = BinaryHeap::new();
    open_options.push(initial_option);

    let mut max_released_pressure = 0;
    let mut max_size = 0;
    let mut state_counter = 0;
    let mut path = vec![];

    while !open_options.is_empty() {
        state_counter += 1;
        let state = open_options.pop().unwrap();
        if state.cumulated > max_released_pressure {
            max_released_pressure = state.cumulated;
            path = state.opened_valves.clone();
        }

        if state.time > 0 {
            let new_states = state.get_nexts(flows.0[state.room], &distance_map);
            open_options.extend(new_states.into_iter());
        }
        max_size = open_options.len().max(max_size);
    }

    println!("The max. pressure released is {}", max_released_pressure);
    println!("The path is {:?}", path);
    println!("The max. queue length is {}", max_size);
    println!("It took {} steps", state_counter);
}

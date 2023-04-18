use std::collections::HashMap;

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

    // println!("The max. pressure released is {}", max);
}

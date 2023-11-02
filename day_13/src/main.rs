use std::{collections::VecDeque, str::FromStr};

#[derive(Clone, Debug)]
enum Entry {
    Number(i8),
    List(Vec<Entry>),
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = VecDeque::new();
        let mut active = None;
        let mut chars = s.chars();
        let mut current_number = None;
        loop {
            match chars.next() {
                Some('[') => {
                    if let Some(active) = active {
                        stack.push_back(active);
                    }
                    active = Some(vec![]);
                }
                Some(']') => {
                    if current_number.is_some() {
                        active
                            .as_mut()
                            .unwrap()
                            .push(Self::Number(current_number.take().unwrap()))
                    }
                    match stack.pop_back() {
                        Some(mut parent) => {
                            parent.push(Self::List(active.unwrap()));
                            active = Some(parent);
                        }
                        None => return Ok(Self::List(active.unwrap())),
                    }
                }
                Some(',') => {
                    if current_number.is_some() {
                        active
                            .as_mut()
                            .unwrap()
                            .push(Self::Number(current_number.take().unwrap()))
                    }
                }
                Some(c) if c.is_ascii_digit() => {
                    current_number =
                        Some(current_number.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as i8);
                }
                Some(_) => return Err("Not a valid input."),
                None => return Ok(Self::List(active.unwrap())),
            }
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Entry::Number(left), Entry::Number(right)) => left == right,
            (Entry::Number(_), Entry::List(_)) => Self::List(vec![self.clone()]) == *other,
            (Entry::List(_), Entry::Number(_)) => *self == Self::List(vec![other.clone()]),
            (Entry::List(left), Entry::List(right)) => {
                let mut l = left.iter();
                let mut r = right.iter();
                loop {
                    match (l.next(), r.next()) {
                        (None, None) => return true,
                        (None, Some(_)) => return false,
                        (Some(_), None) => return false,
                        (Some(x), Some(y)) if x == y => (),
                        (Some(_), Some(_)) => return false,
                    }
                }
            }
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // println!("self: {:?}, other: {:?}", self, other);
        match (self, other) {
            (Entry::Number(left), Entry::Number(right)) => left.cmp(right),
            (Entry::Number(_), Entry::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Entry::List(_), Entry::Number(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Entry::List(left), Entry::List(right)) => {
                let mut l = left.iter();
                let mut r = right.iter();
                loop {
                    match (l.next(), r.next()) {
                        (None, None) => return std::cmp::Ordering::Equal,
                        (None, Some(_)) => return std::cmp::Ordering::Less,
                        (Some(_), None) => return std::cmp::Ordering::Greater,
                        (Some(x), Some(y)) => {
                            let ordering = x.partial_cmp(y).unwrap();
                            if ordering != std::cmp::Ordering::Equal {
                                return ordering;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Eq for Entry {}

struct Pair {
    left: Entry,
    right: Entry,
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = s.lines().map(|line| line.parse::<Entry>().unwrap());
        Ok(Self {
            left: entries.next().unwrap(),
            right: entries.next().unwrap(),
        })
    }
}

impl Pair {
    fn is_valid(&self) -> bool {
        self.left < self.right
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let pairs = input
        .split("\n\n")
        .map(|input| input.parse::<Pair>().unwrap())
        .collect::<Vec<_>>();

    let valids = pairs.iter().enumerate().filter(|(_, pair)| pair.is_valid());

    let valid_pair_numbers = valids
        .map(|(index, _)| index + 1)
        // .inspect(|index| println!("{} is a valid pair", { index }))
        .sum::<usize>();

    println!("Sum of valid pair numbers is {}", valid_pair_numbers);

    let mut lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|input| input.parse::<Entry>().unwrap())
        .collect::<Vec<_>>();

    let divider_packet_1 = "[[2]]".parse::<Entry>().unwrap();
    let divider_packet_2 = "[[6]]".parse::<Entry>().unwrap();
    lines.push(divider_packet_1.clone());
    lines.push(divider_packet_2.clone());

    lines.sort();

    let decoder_key = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| **line == divider_packet_1 || **line == divider_packet_2)
        .map(|(line_number, _)| line_number + 1)
        .product::<usize>();

    println!("The decoder key is {}", decoder_key);
}

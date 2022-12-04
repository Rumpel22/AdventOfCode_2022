use std::ops::RangeInclusive;
use std::{fs, path::Path};

use regex::Regex;

fn parse(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let iter = re.captures(line).unwrap();

    (
        RangeInclusive::new(
            iter.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            iter.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        ),
        RangeInclusive::new(
            iter.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            iter.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        ),
    )
}

fn main() {
    let path = Path::new("src/bin/day_04/input.txt");
    let input = fs::read_to_string(path).unwrap();

    let mut partial_contained = 0;

    let fully_contained = input
        .lines()
        .map(parse)
        .filter(|(r1, r2)| r1.contains(r2.start()) || r2.contains(r1.start()))
        .inspect(|_| partial_contained += 1)
        .filter(|(r1, r2)| {
            (r1.start() >= r2.start() && r1.end() <= r2.end())
                || (r2.start() >= r1.start() && r2.end() <= r1.end())
        })
        .count();

    println!("Solution 1: {fully_contained}, solution 2: {partial_contained}");
}

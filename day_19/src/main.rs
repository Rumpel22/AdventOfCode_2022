mod blueprint;
mod v1;
mod v2;

use blueprint::Blueprint;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../data/input.txt");
    let max_time = 24;

    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();

    let geodes = blueprints
        .iter()
        .map(|blueprint| (blueprint.id, v2::evaluate(&blueprint, max_time)))
        .collect::<HashMap<_, _>>();

    let quality_level = geodes
        .iter()
        .map(|(id, max_geodes)| id * max_geodes)
        .sum::<u32>();

    println!(
        "The quality level after {} minutes: {}",
        max_time, quality_level
    );
}

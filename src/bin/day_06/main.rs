use std::{collections::HashSet, fs, path::Path};

fn parse1(str: &str) -> usize {
    str.chars()
        .filter(|c| c.is_alphabetic())
        .collect::<HashSet<char>>()
        .len()
}

fn parse2(str: &str) -> usize {
    str.split('\n')
        .map(|line| line.chars().collect::<HashSet<_>>())
        .fold({ 'a'..='z' }.collect::<HashSet<_>>(), |init, set| {
            init.intersection(&set).cloned().collect::<HashSet<_>>()
        })
        .len()
}

fn main() {
    let path = Path::new("src/bin/day_06/input.txt");
    match fs::read_to_string(path) {
        Ok(input) => {
            let c1: usize = input.split("\n\n").map(parse1).sum();
            let c2: usize = input.split("\n\n").map(parse2).sum();
            println!("Solution 1: {c1}, solution 2: {c2}");
        }
        Err(err) => {
            println!("{err}")
        }
    }
}

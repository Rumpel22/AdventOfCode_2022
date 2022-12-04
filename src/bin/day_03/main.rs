use std::{fs, path::Path};

use regex::Regex;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c.to_digit(36).unwrap() - 10 + 1,
        'A'..='Z' => c.to_digit(36).unwrap() - 10 + 1 + 26,
        _ => panic!("Invalid character found."),
    }
}

fn main() {
    let path = Path::new("src/bin/day_03/input.txt");
    let input = fs::read_to_string(path).unwrap();

    let solution1 = input
        .lines()
        .map(|line| {
            let s1 = &line[..line.len() / 2];
            let s2 = &line[line.len() / 2..];

            let c = s1.chars().find(|c| s2.contains(*c)).unwrap();
            priority(c)
        })
        .sum::<u32>();

    let re = Regex::new(r"\w+\n\w+\n\w+\n?").unwrap();
    let solution2 = re
        .find_iter(&input)
        .map(|group| {
            let mut lines = group.as_str().lines();
            let l1 = lines.next().unwrap();
            let l2 = lines.next().unwrap();
            let l3 = lines.next().unwrap();
            let c = l1
                .chars()
                .filter(|c| l2.contains(*c))
                .find(|c| l3.contains(*c))
                .unwrap();
            priority(c)
        })
        .sum::<u32>();

    println!("Solution 1: {solution1}, solution 2: {solution2}");
}

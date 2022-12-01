use std::{fs, path::Path};

fn main() {
    let path = Path::new("src/bin/day_01/input.txt");
    match fs::read_to_string(path) {
        Ok(input) => {
            let mut elves = input
                .split("\n\n")
                .map(|grp| {
                    grp.split('\n')
                        .map(|line| line.parse::<i32>().unwrap())
                        .sum::<i32>()
                })
                .collect::<Vec<_>>();
            elves.sort();
            elves.reverse();
            let max_value = elves.first().unwrap();

            let top_3_elves = elves.iter().take(3).sum::<i32>();
            println!("Solution 1: {max_value}, solution 2: {top_3_elves}");
        }
        Err(err) => {
            println!("{err}")
        }
    }
}

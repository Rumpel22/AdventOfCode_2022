use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("src/bin/day_03/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let commands = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let single_solutions =
        commands.map(|(horizontal, vertical)| tree_counter(&lines, horizontal, vertical));
    let total_solution: usize = single_solutions.iter().product();
    println!("{:?} = {}", single_solutions, total_solution);
}

fn tree_counter(lines: &[Vec<bool>], horizontal: usize, vertical: usize) -> usize {
    let mut h_position = 0;
    lines
        .iter()
        .step_by(vertical)
        .filter_map(|line| {
            let is_tree = if line[h_position] { Some(true) } else { None };
            h_position = (h_position + horizontal) % line.len();
            is_tree
        })
        .count()
}

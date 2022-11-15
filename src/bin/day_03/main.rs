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

    let tree_count_1 = tree_counter(&lines, 1, 1);
    let tree_count_3 = tree_counter(&lines, 3, 1);
    let tree_count_5 = tree_counter(&lines, 5, 1);
    let tree_count_7 = tree_counter(&lines, 7, 1);
    let tree_count_1_2 = tree_counter(&lines, 1, 2);
    println!("{tree_count_3}");
    println!(
        "{tree_count_1} x {tree_count_3} x {tree_count_5} x {tree_count_7} x {tree_count_1_2} = {}",
        tree_count_1 * tree_count_3 * tree_count_5 * tree_count_7 * tree_count_1_2
    );
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

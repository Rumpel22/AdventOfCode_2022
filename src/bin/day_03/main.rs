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

fn tree_counter(lines: &[Vec<bool>], horizontal: usize, vertical: usize) -> i64 {
    let mut tree_count = 0;
    let mut v_position = 0;
    let mut h_position = 0;
    while h_position < lines.len() {
        if lines[h_position][v_position] {
            tree_count += 1;
        }
        v_position = (v_position + horizontal) % lines[h_position].len();
        h_position += vertical;
    }
    tree_count
}

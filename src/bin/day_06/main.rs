use std::{fs, path::Path};

fn main() {
    let path = Path::new("src/bin/day_06/input.txt");
    let input = fs::read_to_string(path).unwrap();

    let solution_1 = input
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, window)| !(1..window.len()).any(|i| window[i..].contains(&window[i - 1])))
        .map(|(index, _)| index + 4)
        .unwrap();

    let solution_2 = input
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, window)| !(1..window.len()).any(|i| window[i..].contains(&window[i - 1])))
        .map(|(index, _)| index + 14)
        .unwrap();
    println!("Solution 1: {solution_1}, solution 2: {solution_2}");
}

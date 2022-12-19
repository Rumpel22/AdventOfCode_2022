use itertools::Itertools;

fn solution(length: usize, input: &str) -> usize {
    input
        .as_bytes()
        .windows(length)
        .position(|x| x.iter().unique().count() == x.len())
        .unwrap()
        + length
}

fn main() {
    let input = include_str!("../data/input.txt");

    let solution_1 = solution(4, input);
    let solution_2 = solution(14, input);
    println!("Solution 1: {solution_1}, solution 2: {solution_2}");
}

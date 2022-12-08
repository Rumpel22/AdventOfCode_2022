use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let (solution1, solution2) = input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted_by_key(|&v| std::cmp::Reverse(v))
        .take(3)
        .fold((u32::MIN, u32::default()), |(max, sum), value| {
            (max.max(value), sum + value)
        });

    println!("Solution 1: {solution1}, solution 2: {solution2}");
}

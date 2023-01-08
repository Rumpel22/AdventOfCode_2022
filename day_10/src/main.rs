enum Command {
    Addx(i16),
    Noop,
}

fn main() {
    let input = include_str!("../data/input.txt");

    let commands = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            if iter.next().unwrap() == "noop" {
                Command::Noop
            } else {
                let count = iter.next().unwrap().parse::<i16>().unwrap();
                Command::Addx(count)
            }
        })
        .collect::<Vec<_>>();

    let mut cycle_counter = 0;
    let mut reg_value = 1;

    let intermediate_values = commands
        .iter()
        .map(|command| {
            match command {
                Command::Addx(delta) => {
                    reg_value += *delta;
                    cycle_counter += 2
                }
                Command::Noop => cycle_counter += 1,
            };
            (cycle_counter, reg_value)
        })
        .collect::<Vec<_>>();

    let cycles = [20, 60, 100, 140, 180, 220];
    let signal_strengts = cycles.iter().map(|current_cycle| {
        let index = intermediate_values
            .iter()
            .enumerate()
            .find(|(_, (cyc, _))| cyc >= current_cycle)
            .unwrap()
            .0;
        let value = intermediate_values.get(index - 1).unwrap();
        value.1 * current_cycle
    });
    let signal_strengths_sum = signal_strengts.sum::<i16>();
    println!("The sum of the signal strengths is {signal_strengths_sum}");
}

enum Command {
    Addx(i16),
    Noop,
}

fn value_at_cycle(values: &[(usize, i16)], cycle: usize) -> i16 {
    let index = values
        .iter()
        .take_while(|(cyc, _)| cyc < &cycle)
        .last()
        .map_or(1, |(_, value)| *value);
    index
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

    // Part I
    let cycles = [20, 60, 100, 140, 180, 220];
    let signal_strengts = cycles.iter().map(|current_cycle| {
        let value = value_at_cycle(&intermediate_values, *current_cycle);
        value * (*current_cycle as i16)
    });
    let signal_strengths_sum = signal_strengts.sum::<i16>();
    println!("The sum of the signal strengths is {signal_strengths_sum}");

    // Part II
    let pixels = (1..241)
        .map(|cycle| {
            let value = value_at_cycle(&intermediate_values, cycle);
            let pixel = ((cycle - 1) as i16) % 40;
            (pixel - value).abs() <= 1
        })
        .collect::<Vec<_>>();

    pixels.chunks(40).for_each(|line| {
        line.iter()
            .for_each(|pixel| print!("{}", if *pixel { '█' } else { '.' }));
        println!();
    })
}

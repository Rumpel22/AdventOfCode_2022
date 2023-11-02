struct Number {
    value: i64,
    initial_position: i64,
    position: i64,
}

fn main() {
    // numbers are NOT unique!!!
    let input = include_str!("../data/input.txt");
    let mut numbers = input
        .lines()
        .enumerate()
        .map(|(position, line)| {
            let value = line.parse::<i64>().unwrap() * 811589153;
            let position = position as i64;
            Number {
                value,
                initial_position: position,
                position,
            }
        })
        .collect::<Vec<_>>();

    let num_numbers = numbers.len() as i64;

    for _ in 0..10 {
        for position in 0..num_numbers {
            let current_number = numbers
                .iter_mut()
                .find(|n| n.initial_position == position)
                .unwrap();

            let new_position = current_number.position + current_number.value;
            let new_position = new_position.rem_euclid(num_numbers - 1);
            let old_position = current_number.position;
            let init_position = current_number.initial_position;
            current_number.position = new_position;

            let (lower, upper, offset) = if new_position > old_position {
                (old_position, new_position, -1)
            } else {
                (new_position, old_position, 1)
            };

            numbers
                .iter_mut()
                .filter(|number| {
                    number.position >= lower
                        && number.position <= upper
                        && init_position != number.initial_position
                })
                .for_each(|number| {
                    number.position += offset;
                });
        }
    }

    let zero_position = numbers.iter().find(|number| number.value == 0).unwrap();

    let sum = [1000, 2000, 3000]
        .iter()
        .map(|nth_number| ((zero_position.position + (nth_number % num_numbers)) % num_numbers))
        .map(|position| {
            numbers
                .iter()
                .find(|number| number.position == position)
                .unwrap()
                .value
        })
        .sum::<i64>();

    println!("Sum of grove coordinates: {}", sum);
}

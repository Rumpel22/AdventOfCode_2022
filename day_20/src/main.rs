struct Number {
    value: i32,
    initial_position: i32,
    position: i32,
}

fn wrap(mut value: i32, max: i32) -> i32 {
    while value <= 0 {
        value = value + max - 1;
    }
    while value > max {
        value = value - max + 1;
    }
    value
}

fn main() {
    // numbers are NOT unique!!!
    let input = include_str!("../data/input.txt");
    let mut numbers = input
        .lines()
        .enumerate()
        .map(|(position, line)| {
            let value = line.parse::<i32>().unwrap();
            let position = position as i32;
            Number {
                value,
                initial_position: position,
                position,
            }
        })
        .collect::<Vec<_>>();

    let num_numbers = numbers.len() as i32;

    for position in 0..num_numbers {
        let current_number = numbers
            .iter_mut()
            .find(|n| n.initial_position == position)
            .unwrap();

        let new_position = current_number.position + current_number.value;
        let new_position = wrap(new_position, num_numbers);
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
                number.position = number.position + offset;
            });
    }

    let zero_position = numbers.iter().find(|number| number.value == 0).unwrap();

    let sum = [1000, 2000, 3000]
        .iter()
        .map(|nth_number| ((zero_position.position + (nth_number % num_numbers)) % num_numbers))
        .map(|position| {
            numbers
                .iter()
                .filter(|number| number.position == position)
                .next()
                .unwrap()
                .value
        })
        .sum::<i32>();

    println!("Sum of grove coordinates: {}", sum);
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn row(data: &str) -> u16 {
    let mut value = 0;
    for c in data.chars() {
        match c {
            'F' => value <<= 1,
            'B' => value = (value << 1) + 1,
            _ => panic!("Invalid input data for row"),
        }
    }

    value
}

fn column(data: &str) -> u16 {
    let mut value = 0;
    for c in data.chars() {
        match c {
            'L' => value <<= 1,
            'R' => value = (value << 1) + 1,
            _ => panic!("Invalid input data for row"),
        }
    }

    value
}
fn seat_id(seat_ids: &[u16]) -> Option<u16> {
    seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .find(|(&left, &right)| left + 1 != right)
        .map(|(&_, &seat_id)| seat_id - 1)
}

fn main() {
    let path = Path::new("src/bin/day_05/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap());

    let mut seat_ids = lines
        .map(|line| row(&line.as_str()[..7]) * 8 + column(&line.as_str()[7..]))
        .collect::<Vec<_>>();
    seat_ids.sort();
    let max_value = *seat_ids.last().unwrap();

    let seat_id = seat_id(&seat_ids).unwrap();

    println!("The maxiumum seat ID is {max_value}, your seat ID is {seat_id}");
}

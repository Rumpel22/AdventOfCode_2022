use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn row(data: &str) -> u16 {
    data.chars()
        .fold(0, |value, c| (value << 1) + (if c == 'B' { 1 } else { 0 }))
}

fn column(data: &str) -> u16 {
    data.chars().fold(0, |value, c| match c {
        'L' => value << 1,
        'R' => (value << 1) + 1,
        _ => panic!("Invalid input data for column"),
    })
}
fn seat_id(seat_ids: &[u16]) -> Option<u16> {
    seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .find(|(&prev, &next)| prev + 1 != next)
        .map(|(&prev, _)| prev + 1)
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

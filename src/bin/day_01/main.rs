use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("src/bin/day_01/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let reader = BufReader::new(file);
    let numbers = reader
        .lines()
        .map(|number| number.unwrap())
        .map(|number| number.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for a in numbers.iter() {
        for b in numbers.iter() {
            for c in numbers.iter() {
                if a + b + c == 2020 {
                    println!("{}", a * b * c);
                    return;
                }
            }
        }
    }
}

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

struct Entry {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl Entry {
    fn is_valid_old(&self) -> bool {
        let char_count = self.password.chars().filter(|c| *c == self.char).count();
        char_count >= self.min && char_count <= self.max
    }

    fn is_valid(&self) -> bool {
        let c1 = self.password.chars().nth(self.min - 1).unwrap();
        let c2 = self.password.chars().nth(self.max - 1).unwrap();
        (c1 == self.char) ^ (c2 == self.char)
    }
}

fn parse(line: &str) -> Result<Entry, std::fmt::Error> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
    if let Some(captures) = re.captures(line) {
        assert!(captures.len() == 5);
        Ok(Entry {
            min: captures[1].parse().unwrap(),
            max: captures[2].parse().unwrap(),
            char: captures[3].parse().unwrap(),
            password: captures[4].to_string(),
        })
    } else {
        Err(std::fmt::Error)
    }
}

fn main() {
    let path = Path::new("src/bin/day_02/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap());

    let line_count = lines
        .map(|line| parse(&line))
        .filter(|entry| {
            let x = entry.as_ref().unwrap();
            x.is_valid()
        })
        .count();
    println!("{line_count}");
}

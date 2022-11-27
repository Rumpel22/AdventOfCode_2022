use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

#[derive(Default)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    _cid: Option<u32>,
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        self.byr
            .and(self.iyr)
            .and(self.eyr)
            .and(self.hgt.as_ref())
            .and(self.hcl.as_ref())
            .and(self.ecl.as_ref())
            .and(self.pid.as_ref())
            .is_some()
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && (self.iyr.unwrap() >= 2010 && self.iyr.unwrap() <= 2020)
            && (self.byr.unwrap() >= 1920 && self.byr.unwrap() <= 2002)
            && (self.eyr.unwrap() >= 2020 && self.eyr.unwrap() <= 2030)
            && {
                let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .contains(&self.ecl.as_ref().unwrap().to_lowercase().as_str());
                valid
            }
            && {
                let re = Regex::new("^#[[:xdigit:]]{6}$").unwrap();
                let valid = re.is_match(self.hcl.as_ref().unwrap().to_lowercase().as_str());
                valid
            }
            && {
                let re = Regex::new(r"^\d{9}$").unwrap();
                let valid = re.is_match(self.pid.as_ref().unwrap().as_str());
                valid
            }
            && {
                let string = self.hgt.as_ref().unwrap().as_str();
                let number = &string[..string.len() - 2].parse::<u8>();
                let valid = if number.is_err() {
                    false
                } else if string.ends_with("in") {
                    let number = number.as_ref().unwrap();
                    (59..=76).contains(number)
                } else if string.ends_with("cm") {
                    let number = number.as_ref().unwrap();
                    (150..=193).contains(number)
                } else {
                    false
                };
                valid
            }
    }
}

fn parse(line: &str) -> Result<Passport, std::fmt::Error> {
    let mut passport = Passport::default();
    let re = Regex::new(r"(\w{3}:\S+\b)").unwrap();

    for capture in re.captures_iter(line) {
        let c = &capture[0];
        match &c[0..3] {
            "byr" => {
                passport = Passport {
                    byr: Some(c[4..].parse::<u16>().unwrap()),
                    ..passport
                }
            }
            "iyr" => {
                passport = Passport {
                    iyr: Some(c[4..].parse::<u16>().unwrap()),
                    ..passport
                }
            }
            "eyr" => {
                passport = Passport {
                    eyr: Some(c[4..].parse::<u16>().unwrap()),
                    ..passport
                }
            }
            "hgt" => {
                passport = Passport {
                    hgt: Some(c[4..].to_string()),
                    ..passport
                }
            }
            "hcl" => {
                passport = Passport {
                    hcl: Some(c[4..].to_string()),
                    ..passport
                }
            }
            "ecl" => {
                passport = Passport {
                    ecl: Some(c[4..].to_string()),
                    ..passport
                }
            }
            "pid" => {
                passport = Passport {
                    pid: Some(c[4..].to_string()),
                    ..passport
                }
            }
            "cid" => {
                passport = Passport {
                    _cid: Some(c[4..].parse::<u32>().unwrap()),
                    ..passport
                }
            }
            _ => return Err(std::fmt::Error),
        }
    }
    Ok(passport)
}

fn main() {
    let path = Path::new("src/bin/day_04/input.txt");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap());

    let mut passport_lines = Vec::<String>::new();
    let mut passport_line = String::new();

    for line in lines {
        if line.is_empty() {
            passport_lines.push(passport_line);
            passport_line = String::new();
        } else {
            passport_line += &line;
            passport_line += " ";
        }
    }
    passport_lines.push(passport_line);

    let passports = passport_lines
        .iter()
        .filter_map(|line| parse(line).ok())
        .collect::<Vec<_>>();

    let have_required_fields = passports
        .iter()
        .filter(|passport| passport.has_required_fields())
        .count();
    let are_valid = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    println!("{have_required_fields} passports have all required fields, {are_valid} are valid.");
}

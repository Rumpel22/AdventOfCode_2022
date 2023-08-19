use std::collections::HashMap;

enum Operation {
    Addition,
    Substraction,
    Multiplication,
    Division,
}

enum Monkey<'a> {
    Number(i64),
    Operation(&'a str, &'a str, Operation),
}

fn parse_line(line: &str) -> (&str, Monkey) {
    let name = &line[..4];
    let monkey = if line.chars().nth(6).unwrap().is_ascii_digit() {
        let number = line[6..].parse::<i64>().unwrap();
        Monkey::Number(number)
    } else {
        let name1 = &line[6..10];
        let name2 = &line[13..17];
        let operation = match line.chars().nth(11).unwrap() {
            '+' => Operation::Addition,
            '-' => Operation::Substraction,
            '*' => Operation::Multiplication,
            '/' => Operation::Division,
            _ => panic!("Invalid character"),
        };
        Monkey::Operation(name1, name2, operation)
    };
    (name, monkey)
}

fn evaluate(name: &str, map: &HashMap<&str, Monkey>) -> i64 {
    let monkey = &map[name];
    match monkey {
        Monkey::Number(number) => *number,
        Monkey::Operation(name1, name2, operation) => {
            let number1 = evaluate(&name1, &map);
            let number2 = evaluate(&name2, &map);
            match operation {
                Operation::Addition => number1 + number2,
                Operation::Substraction => number1 - number2,
                Operation::Multiplication => number1 * number2,
                Operation::Division => number1 / number2,
            }
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let monkeys = input.lines().map(parse_line).collect::<HashMap<_, _>>();

    let result = evaluate("root", &monkeys);
    println!("root yells {result}");
}

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
    Human,
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

fn evaluate(name: &str, map: &HashMap<&str, Monkey>) -> Option<i64> {
    let monkey = &map[name];
    match monkey {
        Monkey::Number(number) => Some(*number),
        Monkey::Operation(name1, name2, operation) => {
            let number1 = evaluate(&name1, &map);
            match operation {
                Operation::Addition => number1
                    .and_then(|num1| evaluate(&name2, &map).and_then(|num2| Some(num1 + num2))),
                Operation::Substraction => number1
                    .and_then(|num1| evaluate(&name2, &map).and_then(|num2| Some(num1 - num2))),
                Operation::Multiplication => number1
                    .and_then(|num1| evaluate(&name2, &map).and_then(|num2| Some(num1 * num2))),
                Operation::Division => number1
                    .and_then(|num1| evaluate(&name2, &map).and_then(|num2| Some(num1 / num2))),
            }
        }
        Monkey::Human => None,
    }
}

fn search_unknown(name: &str, map: &HashMap<&str, Monkey>, expected_result: i64) -> i64 {
    let monkey = &map[name];
    match monkey {
        Monkey::Number(_) => unreachable!(),
        Monkey::Operation(name1, name2, operation) => {
            let result1 = evaluate(name1, &map);
            let known_result = result1.or(evaluate(name2, &map)).unwrap();

            let unknown_name = if result1.is_none() { name1 } else { name2 };
            let expected_result = match operation {
                Operation::Addition => expected_result - known_result,
                Operation::Substraction => {
                    if result1.is_some() {
                        known_result - expected_result
                    } else {
                        expected_result + known_result
                    }
                }
                Operation::Multiplication => expected_result / known_result,
                Operation::Division => {
                    if result1.is_some() {
                        known_result / expected_result
                    } else {
                        known_result * expected_result
                    }
                }
            };
            search_unknown(&unknown_name, &map, expected_result)
        }
        Monkey::Human => expected_result,
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let monkeys = input.lines().map(parse_line).collect::<HashMap<_, _>>();

    let result = evaluate("root", &monkeys);
    println!("root yells {}", result.unwrap());

    // ======================

    let mut monkeys = monkeys;
    monkeys.insert("humn", Monkey::Human);

    let (name1, name2) = match monkeys["root"] {
        Monkey::Operation(name1, name2, _) => (name1, name2),
        _ => unreachable!(),
    };
    let result1 = evaluate(name1, &monkeys);

    let human_number = if result1.is_some() {
        search_unknown(name2, &monkeys, result1.unwrap())
    } else {
        let result2 = evaluate(name2, &monkeys);
        search_unknown(name1, &monkeys, result2.unwrap())
    };

    println!("I should call {human_number}");
}

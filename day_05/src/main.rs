use regex::Regex;

#[derive(Debug)]
struct Command {
    count: u32,
    from: usize,
    to: usize,
}

type Commands = Vec<Command>;
type Stack = Vec<char>;
type Stacks = Vec<Stack>;

fn parse_stacks(input: &str) -> Stacks {
    let line_length = input.find('\n').unwrap() + 1;
    let nb_stacks = (line_length + 1) / 4;
    let stack_height = input.chars().filter(|c| c == &'\n').count();
    let mut stacks = Stacks::new();

    for stack_index in 0..nb_stacks {
        let x_pos = stack_index * 4;

        let mut stack = (0..stack_height)
            .filter(|y| {
                let position = y * line_length + x_pos;
                input.chars().nth(position).unwrap_or_default() == '['
            })
            .map(|y| {
                let position = y * line_length + x_pos + 1;
                input.chars().nth(position).unwrap()
            })
            .collect::<Stack>();
        stack.reverse();
        stacks.push(stack);
    }
    stacks
}

fn parse_commands(input: &str) -> Commands {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Command {
                count: captures.get(1).unwrap().as_str().parse().unwrap(),
                from: captures.get(2).unwrap().as_str().parse().unwrap(),
                to: captures.get(3).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

fn parse(input: &str) -> (Stacks, Commands) {
    let mut parts = input.split("\n\n");

    (
        parse_stacks(parts.next().unwrap()),
        parse_commands(parts.next().unwrap()),
    )
}

fn main() {
    let input = include_str!("../data/input.txt");

    let (mut stacks, commands) = parse(input);
    // for command in commands {
    //     for i in 0..command.count {
    //         let c = stacks[command.from - 1].pop().unwrap();
    //         stacks[command.to - 1].push(c);
    //     }
    // }

    for command in commands {
        let old_stack_size = stacks[command.from - 1].len();
        let (x, y) = stacks[command.from - 1].split_at_mut(old_stack_size - command.count as usize);
        let remainder = x.to_vec();
        let to_move = y.to_vec();
        stacks[command.from - 1] = remainder;
        stacks[command.to - 1].append(&mut to_move.to_vec());
    }

    let result = stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();
    println!("{result}");
}

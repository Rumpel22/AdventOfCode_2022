struct Monkey {
    items: Vec<u64>,
    inspect: Box<dyn Fn(u64) -> u64>,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn main() {
    // let input = include_str!("../data/input.txt");
    let monkeys = vec![
        Monkey {
            items: vec![84, 66, 62, 69, 88, 91, 91],
            inspect: Box::new(|old| old * 11),
            test: 2,
            if_true: 4,
            if_false: 7,
        },
        Monkey {
            items: vec![98, 50, 76, 99],
            inspect: Box::new(|old| old * old),
            test: 7,
            if_true: 3,
            if_false: 6,
        },
        Monkey {
            items: vec![72, 56, 94],
            inspect: Box::new(|old| old + 1),
            test: 13,
            if_true: 4,
            if_false: 0,
        },
        Monkey {
            items: vec![55, 88, 90, 77, 60, 67],
            inspect: Box::new(|old| old + 2),
            test: 3,
            if_true: 6,
            if_false: 5,
        },
        Monkey {
            items: vec![69, 72, 63, 60, 72, 52, 63, 78],
            inspect: Box::new(|old| old * 13),
            test: 19,
            if_true: 1,
            if_false: 7,
        },
        Monkey {
            items: vec![89, 73],
            inspect: Box::new(|old| old + 5),
            test: 17,
            if_true: 2,
            if_false: 0,
        },
        Monkey {
            items: vec![78, 68, 98, 88, 66],
            inspect: Box::new(|old| old + 6),
            test: 11,
            if_true: 2,
            if_false: 5,
        },
        Monkey {
            items: vec![70],
            inspect: Box::new(|old| old + 7),
            test: 5,
            if_true: 1,
            if_false: 3,
        },
    ];

    let mut items = monkeys
        .iter()
        .enumerate()
        .flat_map(|(id, monkey)| {
            monkey
                .items
                .iter()
                .map(|item| (id, *item))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let round_count = 10000;
    let mut item_count: [usize; 8] = [0; 8];

    let lcm = monkeys.iter().map(|monkey| monkey.test).product::<u64>();

    for _x in 0..round_count {
        for (monkey_id, monkey) in monkeys.iter().enumerate() {
            let new_items = items
                .iter()
                .map(|(id, item)| {
                    if id != &monkey_id {
                        return (*id, *item);
                    }
                    item_count[monkey_id] += 1;
                    let new_item = ((monkey.inspect)(*item)) % lcm;
                    if new_item % monkey.test == 0 {
                        (monkey.if_true, new_item)
                    } else {
                        (monkey.if_false, new_item)
                    }
                })
                .collect::<Vec<_>>();
            items = new_items;
        }
    }

    item_count.sort();
    item_count.reverse();

    println!("{:?}", item_count);
    println!("{:?}", item_count[0] * item_count[1]);
}

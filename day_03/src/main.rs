use item::Item;
use itertools::Itertools;

mod item {
    pub(crate) struct Item(char);

    impl Item {
        pub(crate) fn priority(self) -> u32 {
            match self.0 {
                'a'..='z' => self.0.to_digit(36).unwrap() - 10 + 1,
                'A'..='Z' => self.0.to_digit(36).unwrap() - 10 + 1 + 26,
                _ => unreachable!("Should never happen"),
            }
        }
    }

    impl From<char> for Item {
        fn from(c: char) -> Self {
            Self(c)
        }
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let solution1 = input
        .lines()
        .map(|line| {
            let s1 = &line[..line.len() / 2];
            let s2 = &line[line.len() / 2..];

            let item = Item::from(s1.chars().find(|c| s2.contains(*c)).unwrap());
            item.priority()
        })
        .sum::<u32>();

    let solution2 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let l1 = group.next().unwrap();
            let l2 = group.next().unwrap();
            let l3 = group.next().unwrap();
            let c = l1
                .chars()
                .find(|c| l2.contains(*c) && l3.contains(*c))
                .unwrap();
            Item::from(c).priority()
        })
        .sum::<u32>();

    println!("Solution 1: {solution1}, solution 2: {solution2}");
}

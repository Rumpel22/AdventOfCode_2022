use std::{str::FromStr, usize};

#[derive(Default, PartialEq, PartialOrd)]
struct Tree(u8);

#[derive(Default)]
struct Trees {
    trees: Vec<Tree>,
    height: usize,
    width: usize,
}

impl Tree {
    fn is_higher_than_all<'a>(&self, mut others: impl Iterator<Item = &'a Tree>) -> bool {
        others.all(|other| self > other)
    }

    fn count_visible_trees<'a>(&self, others: impl Iterator<Item = &'a Tree>) -> u64 {
        others
            .scan(false, |found, tree| {
                if *found {
                    None
                } else {
                    *found = tree >= self;
                    Some(tree)
                }
            })
            .count() as u64
    }
}

impl FromStr for Trees {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trees = input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|digit| Tree(digit as u8)))
            .collect::<Vec<_>>();

        let height = input.lines().count();
        let width = trees.len() / height;

        Ok(Trees {
            trees,
            height,
            width,
        })
    }
}

impl Trees {
    fn left_trees(&self, index: usize) -> impl Iterator<Item = &Tree> {
        let y = index / self.height;
        let (p1, _) = self.trees.split_at(index);
        let left = &p1[(self.width * y)..];

        left.iter().rev()
    }

    fn right_trees(&self, index: usize) -> impl Iterator<Item = &Tree> {
        let x = index % self.width;
        let (_, p2) = self.trees.split_at(index);
        let right = &p2[1..(self.width - x)];

        right.iter()
    }

    fn top_trees(&self, index: usize) -> impl Iterator<Item = &Tree> {
        let (p1, _) = self.trees.split_at(index);
        let top = if index >= self.width {
            &p1[..=(index - self.width)]
        } else {
            &p1[..0]
        };

        top.iter().rev().step_by(self.width)
    }

    fn bottom_trees(&self, index: usize) -> impl Iterator<Item = &Tree> {
        let (_, p2) = self.trees.split_at(index);
        let bottom = if p2.len() >= self.width {
            &p2[(self.width)..]
        } else {
            &p2[..0]
        };

        bottom.iter().step_by(self.width)
    }

    fn outside_visible_trees(&self) -> u64 {
        self.trees
            .iter()
            .enumerate()
            .filter(|(index, tree)| {
                tree.is_higher_than_all(self.left_trees(*index))
                    || tree.is_higher_than_all(self.right_trees(*index))
                    || tree.is_higher_than_all(self.top_trees(*index))
                    || tree.is_higher_than_all(self.bottom_trees(*index))
            })
            .count() as u64
    }

    fn max_scenic_score(&self) -> u64 {
        self.trees
            .iter()
            .enumerate()
            .map(|(index, tree)| {
                tree.count_visible_trees(self.left_trees(index))
                    * tree.count_visible_trees(self.right_trees(index))
                    * tree.count_visible_trees(self.top_trees(index))
                    * tree.count_visible_trees(self.bottom_trees(index))
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let trees = input.parse::<Trees>().unwrap();

    let visible_trees = trees.outside_visible_trees();
    println!("{visible_trees} trees are visible from the outside.");

    let max_scenic_score = trees.max_scenic_score();
    println!("The max. scenic score is {max_scenic_score}.");
}

use std::str::FromStr;

#[derive(Default)]
struct Trees {
    trees: Vec<u8>,
    height: usize,
    width: usize,
}

impl FromStr for Trees {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let trees = input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|digit| digit as u8))
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
    fn count_visibles(&self) -> u64 {
        self.trees
            .iter()
            .enumerate()
            .filter(|(index, &height)| {
                let x = index % self.width;
                let y = index / self.height;

                let (p1, p2) = self.trees.split_at(*index);
                let left = &p1[(self.width * y)..];
                let top = &p1[x..];
                let right = &p2[1..(self.width - x)];
                let bottom = if p2.len() >= self.width {
                    &p2[(self.width)..]
                } else {
                    &p2[0..0]
                };
                (!left.iter().any(|tree| tree >= &height))
                    || (!right.iter().any(|tree| tree >= &height))
                    || (!top.iter().step_by(self.width).any(|tree| tree >= &height))
                    || (!bottom
                        .iter()
                        .step_by(self.width)
                        .any(|tree| tree >= &height))
            })
            .count() as u64
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let trees = input.parse::<Trees>().unwrap();

    let visible_trees = trees.count_visibles();

    println!("{visible_trees} trees are visible from the outside.");
}

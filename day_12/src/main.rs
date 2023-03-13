use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(PartialEq, Clone, Copy)]
struct Field {
    x: usize,
    y: usize,
}

struct Map<T> {
    values: Vec<T>,
    height: usize,
    width: usize,
}

impl FromStr for Map<u8> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let heights = input
            .as_bytes()
            .iter()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        let width = input
            .as_bytes()
            .iter()
            .position(|c| !c.is_ascii_alphabetic())
            .unwrap();
        let height = heights.len() / width;

        Ok(Self {
            values: heights,
            height,
            width,
        })
    }
}

impl<T> Index<Field> for Map<T> {
    type Output = T;

    fn index(&self, index: Field) -> &Self::Output {
        &self.values[index.x + index.y * self.width]
    }
}

impl<T> IndexMut<Field> for Map<T> {
    fn index_mut(&mut self, index: Field) -> &mut Self::Output {
        &mut self.values[index.x + index.y * self.width]
    }
}

impl<T: std::default::Default + std::clone::Clone> Map<T> {
    fn position(&self, index: usize) -> Option<Field> {
        if index >= self.values.len() {
            return None;
        }
        let x = index % self.width;
        let y = index / self.width;
        Some(Field { x, y })
    }

    fn neighbors(&self, field: &Field) -> Vec<Field> {
        let mut neighbors = vec![];

        if field.x > 0 {
            neighbors.push(Field {
                x: field.x - 1,
                y: field.y,
            });
        }
        if field.x < self.width - 1 {
            neighbors.push(Field {
                x: field.x + 1,
                y: field.y,
            });
        }
        if field.y > 0 {
            neighbors.push(Field {
                x: field.x,
                y: field.y - 1,
            });
        }
        if field.y < self.height - 1 {
            neighbors.push(Field {
                x: field.x,
                y: field.y + 1,
            })
        }

        neighbors
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            values: vec![T::default(); width * height],
        }
    }
}

impl Map<u8> {
    #[allow(dead_code)]
    fn start(&self) -> Option<Field> {
        let index = self.values.iter().position(|c| *c == b'S');
        index.map(|index| self.position(index))?
    }

    fn end(&self) -> Option<Field> {
        let index = self.values.iter().position(|c| *c == b'E');
        index.map(|index| self.position(index))?
    }

    fn find_lowest_fields(&self) -> Vec<Field> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, field)| **field == b'a' || **field == b'S')
            .map(|(index, _)| self.position(index).unwrap())
            .collect::<Vec<_>>()
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let map = input.parse::<Map<u8>>().unwrap();

    // let start = vec![map.start().unwrap()];
    let start = map.find_lowest_fields();
    let end = map.end().unwrap();

    let mut steps = Map::<Option<usize>>::new(map.width, map.height);
    let mut options = VecDeque::<Field>::new();
    for s in start {
        steps[s] = Some(0);
        options.push_back(s);
    }

    while let Some(current_field) = options.pop_front() {
        if current_field == end {
            println!(
                "Goal reached after {} steps.",
                steps[current_field].unwrap()
            );
            return;
        }
        let current_height = match map[current_field] {
            b'S' => b'a',
            b'E' => b'z',
            height => height,
        };
        let current_step_count = steps[current_field].unwrap();

        let neighbors = map.neighbors(&current_field);
        let neighbors = neighbors
            .iter()
            .filter(|field| map[**field] <= current_height + 1);

        for neighbor in neighbors {
            if steps[*neighbor].is_none() {
                steps[*neighbor] = Some(current_step_count + 1);
                options.push_back(*neighbor);
            }
        }
    }
    println!("Found no path...");
}

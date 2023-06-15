use std::iter::successors;

use enum_iterator::{next_cycle, Sequence};

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Sequence)]
enum Shape {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square,
}

impl Shape {
    fn width(&self) -> u8 {
        match self {
            Shape::Horizontal => 4,
            Shape::Plus => 3,
            Shape::L => 3,
            Shape::Vertical => 1,
            Shape::Square => 2,
        }
    }

    fn coordinates(&self) -> Vec<(u8, u8)> {
        match self {
            Shape::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Shape::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Shape::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Shape::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Shape::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }
}

#[derive(Clone, Copy)]
struct Rock {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Rock {
    fn new(shape: Shape, y: usize) -> Self {
        Self {
            shape,
            x: 3,
            y: y + 4,
        }
    }

    fn push(&self, direction: Direction) -> Self {
        let x = match direction {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1,
        };

        Self { x, ..*self }
    }

    fn down(&self) -> Self {
        let y = self.y - 1;
        Self { y, ..*self }
    }

    fn is_valid(&self, chamber: &Chamber) -> bool {
        if self.y == 0 || self.x == 0 || self.x + self.shape.width() as usize > 8 {
            return false;
        }
        self.shape
            .coordinates()
            .iter()
            .map(|(piece_x, piece_y)| (*piece_x as usize + self.x, *piece_y as usize + self.y))
            .all(|(x, y)| !chamber.is_occupied(x, y))
    }
}

#[derive(Debug)]
struct Chamber(Vec<[bool; 7]>);

impl Chamber {
    fn is_occupied(&self, x: usize, y: usize) -> bool {
        if y > self.0.len() {
            return false;
        }
        self.0[(y - 1) as usize][(x - 1) as usize]
    }

    fn add(&mut self, rock: &Rock) {
        rock.shape
            .coordinates()
            .iter()
            .map(|(x, y)| (*x as usize + rock.x, *y as usize + rock.y))
            .for_each(|(x, y)| {
                if y as usize > self.0.len() {
                    self.0.resize((y) as usize, [false; 7])
                }
                self.0[(y - 1) as usize][(x - 1) as usize] = true;
            });
    }
}

fn main() {
    let input = include_str!("data/input.txt");
    let mut stream = input
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid input"),
        })
        .cycle();

    let shapes = successors(Some(Shape::Horizontal), |prev| next_cycle(prev));

    let mut chamber = Chamber { 0: vec![] };

    for shape in shapes.take(2022) {
        let highest_block = chamber.0.len();
        let mut rock = Rock::new(shape, highest_block);

        loop {
            let next_rock = rock.push(stream.next().unwrap());
            if next_rock.is_valid(&chamber) {
                rock = next_rock;
            }
            let next_rock = rock.down();
            if next_rock.is_valid(&chamber) {
                rock = next_rock;
            } else {
                chamber.add(&rock);
                break;
            }
        }
    }

    // println!("{:?}", chamber);

    println!("The tower of rocks is {} units tall", chamber.0.len());
}

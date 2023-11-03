use std::{fmt::Display, iter::successors};

use enum_iterator::{cardinality, next_cycle, Sequence};

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

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Shape::Horizontal => 'H',
            Shape::Plus => '+',
            Shape::L => 'L',
            Shape::Vertical => 'v',
            Shape::Square => 'x',
        };
        write!(f, "{}", c)
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
struct Chamber(Vec<[Option<Shape>; 7]>);

impl Chamber {
    fn is_occupied(&self, x: usize, y: usize) -> bool {
        if y > self.height() {
            return false;
        }
        self.0[y - 1][x - 1].is_some()
    }

    fn add(&mut self, rock: &Rock) {
        rock.shape
            .coordinates()
            .iter()
            .map(|(x, y)| (*x as usize + rock.x, *y as usize + rock.y))
            .for_each(|(x, y)| {
                if y > self.height() {
                    self.0.resize(y, [None; 7])
                }
                self.0[y - 1][x - 1] = Some(rock.shape);
            });
    }
    fn height(&self) -> usize {
        self.0.len()
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            write!(f, "|")?;
            for field in row {
                match field {
                    Some(s) => write!(f, "{}", s)?,
                    None => write!(f, " ")?,
                };
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

fn fallen_rocks(rock_count: usize) -> Chamber {
    const INPUT: &str = include_str!("../data/input.txt");
    let mut stream = INPUT
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid input"),
        })
        .cycle();

    let shapes = successors(Some(Shape::Horizontal), next_cycle);

    let mut chamber = Chamber(vec![]);

    for shape in shapes.take(rock_count) {
        let highest_block = chamber.height();
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
    chamber
}

fn main() {
    let chamber = fallen_rocks(2022);
    println!("The tower of rocks is {} units tall.\n", chamber.height());

    println!("PART II\n========");

    // The first ~50 lines do not repeat, reason unknown.
    // At line 75, there is a horizontal piece and nothing else (found experimentally)
    let offset = 75;
    let ten_lines = &chamber.0[offset..offset + 100];
    let distance = chamber.0[offset + 1..]
        .windows(ten_lines.len())
        .position(|window| window == ten_lines)
        .map(|x| x + 1);
    match distance {
        Some(pos) => println!("Repetition after {} lines", pos),
        None => {
            println!("No repetition");
            return;
        }
    }

    let repeating_rows = distance.unwrap();

    let lines = [
        offset,
        offset + repeating_rows - 1,
        offset + repeating_rows,
        offset + repeating_rows + 1,
    ];
    for line in lines {
        println!("Line {}: {:?}", line, chamber.0[line]);
    }

    fn pieces_in_first_n_rows(n: usize, chamber: &Chamber) -> usize {
        chamber
            .0
            .iter()
            .enumerate()
            .take_while(|(line, _)| line < &n)
            .filter(|(_, row)| row.iter().any(|shape| shape == &Some(Shape::Horizontal)))
            .count()
            * cardinality::<Shape>()
    }

    // There are X pieces fallen down before the found repetition starts and the tower is already 'offset' tall
    let first_n_pieces = pieces_in_first_n_rows(offset, &chamber);
    println!(
        "To fill the first {} rows, it takes {} pieces.",
        offset, first_n_pieces
    );

    let pieces_until_repetition = pieces_in_first_n_rows(repeating_rows + offset, &chamber);
    // println!("{}", pieces_until_repetition);

    let pieces_within_repetition = pieces_until_repetition - first_n_pieces;
    // println!("{}", pieces_within_repetition);

    let total_pieces = 1000000000000;
    let remaining_pieces = total_pieces - first_n_pieces;

    let n_repetitions = remaining_pieces / pieces_within_repetition;
    let height_repetitions = n_repetitions * repeating_rows;
    println!(
        "There are {} repetitions, each with {} pieces. This results in a tower that is {} pieces tall.",
        n_repetitions, pieces_within_repetition, height_repetitions
    );

    let repeated_pieces = n_repetitions * pieces_within_repetition;
    let final_pieces = remaining_pieces - repeated_pieces;
    println!(
        "At the end, there are {} pieces left, that are not part of a complete repetition.",
        final_pieces
    );

    let remaining_chamber = fallen_rocks(first_n_pieces + final_pieces);
    let total_height = remaining_chamber.height() + height_repetitions;
    println!("The total height is {}.", total_height);
}

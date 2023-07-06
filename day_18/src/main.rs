use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();
        Ok(Cube { x, y, z })
    }
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn connected_to(&self, cube: &Cube) -> bool {
        (self.x - cube.x).abs() + (self.y - cube.y).abs() + (self.z - cube.z).abs() == 1
    }

    fn above(&self) -> Self {
        Self {
            z: self.z - 1,
            ..*self
        }
    }
    fn beneath(&self) -> Self {
        Self {
            z: self.z + 1,
            ..*self
        }
    }
    fn left(&self) -> Self {
        Self {
            y: self.y - 1,
            ..*self
        }
    }
    fn right(&self) -> Self {
        Self {
            y: self.y + 1,
            ..*self
        }
    }
    fn front(&self) -> Self {
        Self {
            x: self.x + 1,
            ..*self
        }
    }
    fn back(&self) -> Self {
        Self {
            x: self.x - 1,
            ..*self
        }
    }

    fn neighbors(&self) -> CubeNeighbors {
        CubeNeighbors {
            cube: *self,
            index: 0,
        }
    }
}

struct CubeNeighbors {
    cube: Cube,
    index: usize,
}

impl Iterator for CubeNeighbors {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        let cube = match self.index {
            0 => self.cube.above(),
            1 => self.cube.beneath(),
            2 => self.cube.left(),
            3 => self.cube.right(),
            4 => self.cube.front(),
            5 => self.cube.back(),
            _ => return None,
        };
        self.index += 1;
        Some(cube)
    }
}

#[derive(PartialEq)]
enum Content {
    Lava,
    Water,
    Unknown,
}
struct Vessel {
    map: HashMap<Cube, Content>,
    first: Cube,
}

impl Vessel {
    fn new_for_lava(cubes: &[Cube]) -> Self {
        // +/-1, to allow some water to flow around the lava
        let x_min = cubes.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x - 1;
        let y_min = cubes.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y - 1;
        let z_min = cubes.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap().z - 1;
        let x_max = cubes.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x + 1;
        let y_max = cubes.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 1;
        let z_max = cubes.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap().z + 1;

        // This one is guaranteed to be empty and will be filled with water
        let first = Cube::new(x_min, y_min, z_min);

        // Create empty map with all cubes set to unknown
        let mut map = [x_min..=x_max, y_min..=y_max, z_min..=z_max]
            .into_iter()
            .multi_cartesian_product()
            .map(|v| {
                let mut v = v.iter();
                let x = v.next().unwrap();
                let y = v.next().unwrap();
                let z = v.next().unwrap();
                (Cube::new(*x, *y, *z), Content::Unknown)
            })
            .collect::<HashMap<_, _>>();

        // Insert lava
        map.extend(cubes.iter().map(|cube| (*cube, Content::Lava)));

        Self { map, first }
    }

    fn fill_water(&mut self) {
        let mut candidates = vec![self.first];

        while let Some(candidate) = candidates.pop() {
            self.map
                .entry(candidate)
                .and_modify(|x| *x = Content::Water);

            for neighbor in candidate.neighbors() {
                if let Some((_, content)) = self.map.get_key_value(&neighbor) {
                    if content == &Content::Unknown {
                        candidates.push(neighbor);
                    }
                }
            }
        }
    }

    fn air(&self) -> Vec<Cube> {
        self.map
            .iter()
            .filter_map(|(cube, content)| {
                if content == &Content::Unknown {
                    Some(*cube)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn get_total_surface(cubes: &[Cube]) -> usize {
    let sites = cubes
        .iter()
        .map(|cube| {
            cubes
                .iter()
                .filter(|inner_cube| inner_cube.connected_to(cube))
                .count()
        })
        .sum::<usize>();
    let total_sites = cubes.len() * 6;
    total_sites - sites
}

fn main() {
    let input = include_str!("../data/input.txt");

    let lava_cubes = input
        .lines()
        .map(|line| line.parse::<Cube>().unwrap())
        .collect::<Vec<_>>();

    let total_surface = get_total_surface(&lava_cubes);
    println!("Total surface of the droplet is {}.", total_surface);

    let mut vessel = Vessel::new_for_lava(&lava_cubes);
    vessel.fill_water();

    let air_cubes = vessel.air();
    let air_surface = get_total_surface(&air_cubes);

    let outer_surface = total_surface - air_surface;
    println!("The total outer surface is {}.", outer_surface);
}

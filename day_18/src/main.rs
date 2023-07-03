use std::str::FromStr;

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
    fn connected_to(&self, cube: &Cube) -> bool {
        (self.x - cube.x).abs() + (self.y - cube.y).abs() + (self.z - cube.z).abs() == 1
    }
}

fn main() {
    let input = include_str!("../data/input.txt");

    let cubes = input
        .lines()
        .map(|line| line.parse::<Cube>().unwrap())
        .collect::<Vec<_>>();

    let covered_sites = cubes
        .iter()
        .map(|cube| {
            cubes
                .iter()
                .filter(|inner_cube| inner_cube.connected_to(cube))
                .count()
        })
        .sum::<usize>();
    let total_sites = cubes.len() * 6;
    println!("{}", total_sites - covered_sites);
}

use std::str::FromStr;

#[derive(Clone)]
struct File(usize);

#[derive(Default, Clone)]
struct Folder {
    files: Vec<File>,
    dirs: Vec<Folder>,
}

trait Size {
    fn size(&self) -> usize;
}

impl Size for File {
    fn size(&self) -> usize {
        self.0
    }
}

impl Size for Folder {
    fn size(&self) -> usize {
        self.files.iter().map(|file| file.0).sum::<usize>()
            + self.dirs.iter().fold(0, |sum, dir| sum + dir.size())
    }
}

fn parse_folder<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Folder {
    let mut folder = Folder::default();
    while let Some(line) = lines.next() {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            if dir == ".." {
                return folder;
            } else {
                folder.dirs.push(parse_folder(lines));
            }
        } else if let Some(x) = line.split_ascii_whitespace().next() {
            if let Ok(number) = x.parse::<usize>() {
                folder.files.push(File(number));
            }
        }
    }

    folder
}

impl FromStr for Folder {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        Ok(parse_folder(&mut lines))
    }
}

impl Folder {
    fn get_folders(&self) -> Vec<Folder> {
        if self.dirs.is_empty() {
            return vec![];
        }
        let mut dirs = self.dirs.clone();
        for dir in &self.dirs {
            dirs.append(&mut dir.get_folders());
        }
        dirs
    }
}

fn main() {
    let input = include_str!("../data/input.txt");
    let data = input.parse::<Folder>().unwrap();

    let folders = data.get_folders();
    let sum = folders
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size <= &100000)
        .sum::<usize>();

    println!("Solution part 1: {sum}");

    let used_disk_space = data.size();
    let free_disk_space = 70000000 - used_disk_space;
    let required_disk_space = 30000000 - free_disk_space;

    let folder_size = folders
        .iter()
        .map(|dir| dir.size())
        .filter(|size| size >= &required_disk_space)
        .min()
        .unwrap();
    println!("Solution part 2: {folder_size}");
}

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(file_name: &str) -> Vec<String> {
    let path = format!("resources/{}", file_name);
    let reader = BufReader::new(File::open(path).unwrap());
    reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>()
}

pub fn read_scratch_file() -> String {
    fs::read_to_string("resources/scratch.txt").unwrap()
}

const DIRS4: &[(isize, isize)] = &[(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn neighbours4(
    c: (usize, usize),
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    DIRS4
        .into_iter()
        .map(move |(dx, dy)| (c.0 as isize + dx, c.1 as isize + dy))
        .filter(move |&(nx, ny)| {
            nx >= min_x as isize
                && nx < max_x as isize
                && ny >= min_y as isize
                && ny < max_y as isize
        })
        .map(|(nx, ny)| (nx as usize, ny as usize))
}

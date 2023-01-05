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

pub fn debug_2d_map(map: &[(isize, isize, char)], default: char) {
    let (min_x, max_x, min_y, max_y) = map.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), (x, y, _)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    debug_2d_map_fixed(min_x, max_x, min_y, max_y, map, default);
}

pub fn debug_2d_map_fixed(
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    map: &[(isize, isize, char)],
    default: char,
) {
    let mut v = vec![vec![default; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for (x, y, c) in map {
        v[(y - min_y) as usize][(x - min_x) as usize] = *c;
    }
    for row in v {
        println!("{}", row.iter().collect::<String>());
    }
}

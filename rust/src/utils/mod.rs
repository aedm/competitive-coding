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

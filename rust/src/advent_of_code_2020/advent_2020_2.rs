use crate::utils::read_lines;
use regex::Regex;
use std::ops::BitXor;

pub fn solve_1() -> usize {
    let lines = read_lines("advent_2020/2.txt");
    lines.iter().filter(|x| valid_1(x)).count()
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2020/2.txt");
    lines.iter().filter(|x| valid_2(x)).count()
}

fn valid_1(pass: &str) -> bool {
    let pattern: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    let caps = pattern.captures(pass).unwrap();
    let from = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let to = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let c = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let s = caps.get(4).unwrap().as_str();
    let count = s.chars().filter(|x| *x == c).count();
    count >= from && count <= to
}

fn valid_2(pass: &str) -> bool {
    let pattern: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
    let caps = pattern.captures(pass).unwrap();
    let from = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let to = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let c = caps.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let s = caps.get(4).unwrap().as_str();
    (s.chars().nth(from - 1).unwrap_or(' ') == c).bitxor(s.chars().nth(to - 1).unwrap_or(' ') == c)
}

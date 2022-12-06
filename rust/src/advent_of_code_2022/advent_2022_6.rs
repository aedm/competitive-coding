use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub fn solve_1() -> usize {
    read_lines("advent_2022/6.txt")[0]
        .as_bytes()
        .windows(4)
        .find_position(|&w| w.iter().cloned().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        .0
        + 4
}

pub fn solve_2() -> usize {
    read_lines("advent_2022/6.txt")[0]
        .as_bytes()
        .windows(14)
        .find_position(|&w| w.iter().cloned().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        .0
        + 14
}

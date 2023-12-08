use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn read_input() -> (Vec<usize>, HashMap<String, [String; 2]>) {
    let pattern: Regex = Regex::new(r"^(...) = .(...), (...).$").unwrap();
    let lines = read_lines("advent_2023/8.txt");
    let dirs = lines[0].chars().map(|c| if c == 'L' { 0 } else { 1 }).collect_vec();
    let junc = lines[2..]
        .iter()
        .map(|l| {
            let s = pattern.captures(l).unwrap();
            (s[1].to_string(), [s[2].to_string(), s[3].to_string()])
        })
        .collect::<HashMap<_, _>>();
    (dirs, junc)
}

pub fn solve_1() -> usize {
    let (dirs, junc) = read_input();
    let (mut s, mut i) = ("AAA", 0);
    while s != "ZZZ" {
        s = &junc[s][dirs[i % dirs.len()]];
        i += 1;
    }
    i
}

pub fn solve_2() -> u128 {
    let (dirs, junc) = read_input();
    junc.keys()
        .filter(|k| k.ends_with('A'))
        .map(|mut s| {
            let mut i = 0;
            while !s.ends_with('Z') {
                s = &junc[s][dirs[i % dirs.len()]];
                i += 1;
            }
            i as u128
        })
        .fold(1, num::integer::lcm)
}

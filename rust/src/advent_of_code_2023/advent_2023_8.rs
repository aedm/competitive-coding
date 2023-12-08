use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Input = (Vec<usize>, HashMap<String, [String; 2]>);

fn read_input() -> Input {
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

pub fn find_cycle<'a>(input: &'a Input, mut s: &'a str, end: &str) -> u128 {
    let mut i = 0;
    while !s.ends_with('Z') {
        s = &input.1[s][input.0[i % input.0.len()]];
        i += 1;
    }
    i as u128
}

pub fn solve_1() -> usize {
    find_cycle(&read_input(), "AAA", "ZZZ") as usize
}

pub fn solve_2() -> u128 {
    let input = read_input();
    input.1.keys()
        .filter(|k| k.ends_with('A'))
        .map(|mut s| find_cycle(&input, &mut s, "Z"))
        .fold(1, num::integer::lcm)  // ottrohadjonmeg
}

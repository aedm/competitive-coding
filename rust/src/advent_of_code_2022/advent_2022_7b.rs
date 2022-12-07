use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

fn read_input() -> Vec<i64> {
    let mut dirs = HashMap::new();
    let mut path = vec![];
    for line in read_lines("advent_2022/7.txt") {
        let p = line.split(' ').collect_vec();
        if line.starts_with("$ cd") {
            if p[2] == ".." {
                path.pop();
            } else {
                path.push(p[2].to_string());
            }
        } else if line.starts_with("$ ls") {
            dirs.insert(path.join("/"), 0);
        } else if let Ok(v) = p[0].parse::<i64>() {
            *dirs.get_mut(&path.join("/")).unwrap() += v;
        }
    }
    dirs.keys()
        .map(|k| dirs.iter().filter(|(p, _)| p.starts_with(k)).map(|(_, v)| v).sum::<i64>())
        .sorted()
        .collect()
}

pub fn solve_1() -> i64 {
    read_input().iter().filter(|&&s| s <= 100000).sum::<i64>()
}

pub fn solve_2() -> i64 {
    let sizes = read_input();
    let needed = sizes.last().unwrap() - (70000000 - 30000000);
    *sizes.iter().filter(|v| **v >= needed).min().unwrap()
}

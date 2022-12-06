use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;

pub fn solve_1() -> String {
    let pattern: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let lines = read_lines("advent_2022/5.txt");
    let counter = lines.iter().find_position(|l| l.starts_with(" 1 ")).unwrap();
    let n = (counter.1.len() + 1) / 3;
    let height = counter.0;
    let mut stacks = (0..n)
        .map(|i| {
            let mut v = vec![];
            for row in 0..height {
                if let Some(b) = lines[height - row - 1].chars().nth(i * 4 + 1) {
                    if b != ' ' {
                        v.push(b);
                    }
                }
            }
            v
        })
        .collect_vec();
    for line in lines[height + 2..].iter() {
        let caps = pattern.captures(line).unwrap();
        let r = (1usize..=3)
            .map(|i| caps.get(i).unwrap().as_str().parse::<usize>().unwrap())
            .collect_vec();
        let source = r[1] - 1;
        let target = r[2] - 1;
        let first = stacks[source].len() - r[0];
        let mut q = stacks[source][first..].iter().cloned().rev().collect_vec();
        stacks[target].append(&mut q);
        stacks[source].truncate(first);
    }
    stacks.iter().map(|s| s.last()).flatten().collect()
}

pub fn solve_2() -> String {
    let pattern: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let lines = read_lines("advent_2022/5.txt");
    let counter = lines.iter().find_position(|l| l.starts_with(" 1 ")).unwrap();
    let n = (counter.1.len() + 1) / 3;
    let height = counter.0;
    let mut stacks = (0..n)
        .map(|i| {
            let mut v = vec![];
            for row in 0..height {
                if let Some(b) = lines[height - row - 1].chars().nth(i * 4 + 1) {
                    if b != ' ' {
                        v.push(b);
                    }
                }
            }
            v
        })
        .collect_vec();
    for line in lines[height + 2..].iter() {
        let caps = pattern.captures(line).unwrap();
        let r = (1usize..=3)
            .map(|i| caps.get(i).unwrap().as_str().parse::<usize>().unwrap())
            .collect_vec();
        let source = r[1] - 1;
        let target = r[2] - 1;
        let first = stacks[source].len() - r[0];
        let mut q = stacks[source][first..].iter().cloned().collect_vec();
        stacks[target].append(&mut q);
        stacks[source].truncate(first);
    }
    stacks.iter().map(|s| s.last()).flatten().collect()
}

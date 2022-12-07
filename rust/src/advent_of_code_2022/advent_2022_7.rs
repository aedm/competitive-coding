use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

fn collect(path: &str, dirs: &HashMap<String, (i64, Vec<String>)>, sums: &mut Vec<i64>) -> i64 {
    let d = dirs.get(path).unwrap();
    let size = d.0 + d.1.iter().map(|s| collect(&format!("{path}/{s}"), dirs, sums)).sum::<i64>();
    sums.push(size);
    size
}

fn read_input() -> HashMap<String, (i64, Vec<String>)> {
    let mut dirs = HashMap::from([("/".to_string(), (0, vec![]))]);
    let mut current = vec![];
    let mut path: Option<String> = None;
    for line in read_lines("advent_2022/7.txt") {
        let p = line.split(' ').collect_vec();
        if p[0] == "$" {
            if p[1] == "cd" {
                if p[2] == ".." {
                    current.pop();
                } else {
                    current.push(p[2].to_string());
                    path = Some(current.join("/"));
                }
            }
        } else if p[0] == "dir" {
            let key = path.as_ref().unwrap();
            dirs.get_mut(key).unwrap().1.push(p[1].to_string());
            dirs.insert(format!("{key}/{}", p[1]), (0, Vec::new()));
        } else {
            dirs.get_mut(path.as_ref().unwrap()).unwrap().0 += p[0].parse::<i64>().unwrap();
        }
    }
    dirs
}

pub fn solve_1() -> i64 {
    let mut sums = vec![];
    let _ = collect("/", &read_input(), &mut sums);
    sums.iter().filter(|s| **s <= 100000).sum::<i64>()
}

pub fn solve_2() -> i64 {
    let mut sums = vec![];
    let total = collect("/", &read_input(), &mut sums);
    let needed = total - (70000000 - 30000000);
    *sums.iter().filter(|v| **v >= needed).min().unwrap()
}

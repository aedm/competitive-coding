use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

fn sum_dir_sizes(
    path: &str,
    dirs: &HashMap<String, (i64, Vec<String>)>,
    sums: &mut HashMap<String, i64>,
) -> i64 {
    let dir = dirs.get(path).unwrap();
    let mut size = dir.0;
    for subdir in dir.1.iter() {
        size += sum_dir_sizes(&format!("{}/{}", path, subdir), dirs, sums);
    }
    sums.insert(path.to_string(), size);
    size
}

fn read() -> HashMap<String, (i64, Vec<String>)> {
    let mut dirs: HashMap<String, (i64, Vec<String>)> =
        HashMap::from([("/".to_string(), (0, vec![]))]);
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
                }
            } else if p[1] == "ls" {
                path = Some(current.join("/"));
            }
        } else if p[0] == "dir" {
            let key = path.as_ref().unwrap();
            dirs.get_mut(key).unwrap().1.push(p[1].to_string());
            dirs.insert(format!("{key}/{}", p[1]), (0, Vec::new()));
        } else {
            let key = path.as_ref().unwrap();
            dirs.get_mut(key).unwrap().0 += p[0].parse::<i64>().unwrap();
        }
    }
    dirs
}

pub fn solve_1() -> i64 {
    let mut sums = HashMap::new();
    let _ = sum_dir_sizes("/", &read(), &mut sums);
    sums.values().filter(|s| **s <= 100000).sum::<i64>()
}

pub fn solve_2() -> i64 {
    let mut sums = HashMap::new();
    let total = sum_dir_sizes("/", &read(), &mut sums);
    let needed = total - (70000000 - 30000000);
    *sums.values().filter(|v| **v >= needed).min().unwrap()
}

use crate::utils::{read_lines, read_lines_split};
use itertools::Itertools;
use regex::Regex;
use std::array::from_fn;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

type Rules = HashMap<String, Vec<(Option<usize>, bool, i64, String)>>;

fn read_input() -> (Rules, Vec<Vec<i64>>) {
    let lines = read_lines("advent_2023/19.txt");
    let ls = lines.split(String::is_empty).collect_vec();

    let rule_pattern = Regex::new(r"(\w)([<>])(\d+):(.*)").unwrap();
    let comps = HashMap::from([("x", 0), ("m", 1), ("a", 2), ("s", 3)]);
    let rules = ls[0]
        .iter()
        .map(|l| {
            let it = l.split_terminator(&['{', ',', '}']).collect_vec();
            let rule = it[1..]
                .iter()
                .map(|r| {
                    if let Some(c) = rule_pattern.captures(r) {
                        let index = comps[&c[1]];
                        let is_greater = &c[2] == ">";
                        let amount = c[3].parse::<i64>().unwrap();
                        let target = c[4].to_string();
                        (Some(index), is_greater, amount, target)
                    } else {
                        (None, false, 0, r.to_string())
                    }
                })
                .collect_vec();
            (it[0].to_string(), rule)
        })
        .collect::<HashMap<_, _>>();

    let num_pattern = Regex::new(r"(\d+)").unwrap();
    let items = ls[1]
        .iter()
        .map(|l| num_pattern.find_iter(l).map(|m| m.as_str().parse::<i64>().unwrap()).collect_vec())
        .collect_vec();

    (rules, items)
}

pub fn solve_core(rules: &Rules, v: [(i64, i64); 4]) -> i64 {
    let mut acc = 0;
    let mut q = VecDeque::from([(v, "in")]);
    while let Some((mut v, mut label)) = q.pop_front() {
        if label == "R" || v.iter().any(|(a, b)| a > b) {
            continue;
        }
        if label == "A" {
            acc += v.iter().map(|(a, b)| b - a + 1).product::<i64>();
            continue;
        }
        for (index, cmp, num, target) in rules.get(label).unwrap() {
            if let Some(i) = index.clone() {
                let mut vgo = v.clone();
                if *cmp {
                    vgo[i].0 = v[i].0.max(*num + 1);
                    v[i].1 = v[i].1.min(*num);
                } else {
                    vgo[i].1 = v[i].1.min(*num - 1);
                    v[i].0 = v[i].0.max(*num);
                }
                q.push_back((vgo, target));
            } else {
                q.push_back((v, target));
                break;
            }
        }
    }
    acc
}

pub fn solve_1() -> i64 {
    let (rules, items) = read_input();
    items
        .iter()
        .map(|c| solve_core(&rules, from_fn(|i| (c[i], c[i]))) * (c[0] + c[1] + c[2] + c[3]))
        .sum::<i64>()
}

pub fn solve_2() -> i64 {
    let (rules, _) = read_input();
    solve_core(&rules, [(1, 4000); 4])
}

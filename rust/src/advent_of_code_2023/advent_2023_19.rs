use crate::utils::{read_lines, read_lines_split};
use itertools::Itertools;
use regex::Regex;
use std::array::from_fn;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

type Rules = HashMap<String, Vec<(Option<usize>, bool, i64, String)>>;

fn read_input() -> (Rules, Vec<Vec<i64>>) {
    let lines = read_lines("advent_2023/19.txt");
    let ls = lines.split(|l| l.is_empty()).collect_vec();

    let comps = HashMap::from([('x', 0usize), ('m', 1), ('a', 2), ('s', 3)]);
    let rules = ls[0]
        .iter()
        .map(|l| {
            let it = l.split_terminator(&['{', ',', '}']).collect_vec();
            let id = it[0].to_string();
            let rule = it[1..]
                .iter()
                .map(|r| {
                    let ps = r.split_inclusive(&[':', '<', '>']).collect_vec();
                    if ps.len() == 1 {
                        (None, false, 0, ps[0].to_string())
                    } else {
                        let target = ps[2].to_string();
                        let amount = ps[1][..ps[1].len() - 1].parse::<i64>().unwrap();
                        let cond = ps[0].chars().last() == Some('>');
                        let el = comps[&ps[0].chars().nth(0).unwrap()];
                        (Some(el), cond, amount, target)
                    }
                })
                .collect_vec();
            (id, rule)
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
        for (el, cmp, num, target) in rules.get(label).unwrap() {
            if let Some(el) = el.clone() {
                let mut vgo = v.clone();
                if *cmp {
                    vgo[el].0 = v[el].0.max(*num + 1);
                    v[el].1 = v[el].1.min(*num);
                } else {
                    vgo[el].1 = v[el].1.min(*num - 1);
                    v[el].0 = v[el].0.max(*num);
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
        .map(|item| {
            let v = from_fn(|i| (item[i], item[i]));
            solve_core(&rules, v) * (item[0] + item[1] + item[2] + item[3])
        })
        .sum::<i64>()
}

pub fn solve_2() -> i64 {
    let (rules, _) = read_input();
    solve_core(&rules, [(1, 4000); 4])
}

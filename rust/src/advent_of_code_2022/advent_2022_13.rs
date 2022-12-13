use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use serde_json::Value;
use std::cmp::{min, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};

fn order(a: &Value, b: &Value) -> Ordering {
    match (a.is_array(), b.is_array()) {
        (true, true) => {
            let alen = a.as_array().unwrap().len();
            let blen = b.as_array().unwrap().len();
            for i in 0..min(alen, blen) {
                let o = order(&a[i], &b[i]);
                if o != Ordering::Equal {
                    return o;
                }
            }
            alen.cmp(&blen)
        }
        (true, false) => order(&a, &Value::from(vec![b.clone()])),
        (false, true) => order(&Value::from(vec![a.clone()]), &b),
        _ => a.as_i64().cmp(&b.as_i64()),
    }
}

pub fn solve_1() -> i64 {
    read_lines("advent_2022/13.txt")
        .chunks(3)
        .enumerate()
        .map(|(i, c)| {
            let n = c[0..2].iter().map(|s| serde_json::from_str::<Value>(s).unwrap()).collect_vec();
            return if order(&n[0], &n[1]) != Ordering::Greater { i + 1 } else { 0 };
        })
        .sum::<usize>() as i64
}

pub fn solve_2() -> i64 {
    let mut l = read_lines("advent_2022/13.txt")
        .into_iter()
        .filter(|s| s.len() > 0)
        .map(|s| serde_json::from_str::<Value>(&s).unwrap())
        .collect_vec();
    let v1 = Value::from(vec![Value::from(vec![2i64])]);
    let v2 = Value::from(vec![Value::from(vec![6i64])]);
    let vs = vec![v1, v2];
    l.append(&mut vs.clone());
    l.sort_by(|a, b| order(a, b));
    vs.iter().map(|a| l.iter().find_position(|&v| v == a).unwrap().0 + 1).product::<usize>() as i64
}

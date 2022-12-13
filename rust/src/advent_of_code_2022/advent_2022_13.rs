use crate::utils::read_lines;
use itertools::Itertools;
use serde_json::{json, Value};
use std::cmp::{min, Ordering};

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
    let mut ls = read_lines("advent_2022/13.txt")
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str::<Value>(&s).unwrap())
        .collect_vec();
    let vs = vec![json!([[2]]), json!([[6]])];
    ls.append(&mut vs.clone());
    ls.sort_by(|a, b| order(a, b));
    vs.iter().map(|v| ls.iter().find_position(|&l| l == v).unwrap().0 + 1).product::<usize>() as i64
}

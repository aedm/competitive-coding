use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(alias = "barrelEthanolPairs")]
    barrels: Vec<Vec<i64>>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let count = input.barrels.len();

    let mut order: Vec<_> = (0..count).collect();
    order.sort_by(|&a, &b| {
        input.barrels[a][0]
            .cmp(&input.barrels[b][0])
            .then(input.barrels[a][1].cmp(&input.barrels[b][1]))
    });
    let barrels: Vec<_> = order
        .iter()
        .map(|&i| (input.barrels[i][0], input.barrels[i][1]))
        .collect();

    let mut min: Vec<(i64, i64)> = Vec::with_capacity(count);
    let mut prev = vec![-1i64; count];

    for (i, c) in barrels[0..count].iter().enumerate() {
        let f = min.binary_search_by(|&m| c.1.cmp(&m.0));
        if let Err(n) = f {
            if n >= min.len() {
                min.push((c.1, i as i64));
            } else {
                min[n] = (c.1, i as i64);
            }
            if n > 0 {
                prev[i] = min[n - 1].1;
            }
        }
    }
    let mut s = min[min.len() - 1].1;
    let mut solution = vec![];
    while s >= 0 {
        solution.push(order[s as usize] + 1);
        s = prev[s as usize];
    }
    solution.reverse();

    json!({ "barrelSequence": Value::from(solution) })
}

pub fn solve2(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let count = input.barrels.len();

    let mut order: Vec<_> = (0..count).collect();
    order.sort_by(|a, b| input.barrels[*a][0].cmp(&input.barrels[*b][0]));

    let barrels: Vec<_> = (0..count)
        .map(|i| (input.barrels[order[i]][0], input.barrels[order[i]][1]))
        .collect();

    let mut back = vec![(0, -1i64); count];
    let mut low = 0;
    for (i, c) in barrels[0..count].iter().enumerate() {
        while barrels[low].0 < c.0 {
            low += 1;
        }
        let mut mx = (0, -1i64);
        for (o, b) in barrels[0..low].iter().enumerate() {
            if b.1 > c.1 && mx.0 < back[o].0 {
                mx = (back[o].0, o as i64);
            }
        }
        back[i] = (mx.0 + 1i64, mx.1);
    }
    let mut s: i64 = (0..count)
        .max_by(|a, b| back[*a].0.cmp(&back[*b].0))
        .unwrap() as i64;
    let mut solution = Vec::new();
    loop {
        solution.push(order[s as usize] + 1);
        s = back[s as usize].1;
        if s < 0 {
            break;
        }
    }
    solution.reverse();

    json!({ "barrelSequence": Value::from(solution) })
}

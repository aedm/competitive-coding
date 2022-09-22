use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(alias = "barrelEthanolPairs")]
    barrels: Vec<Vec<i64>>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let now = Instant::now();

    let barrels: Vec<_> = input.barrels.iter().map(|b| (b[0], b[1])).collect();
    let mut order: Vec<_> = (0..barrels.len()).collect();
    order.sort_by(|&a, &b| {
        barrels[a].0.cmp(&barrels[b].0).reverse().then(barrels[a].1.cmp(&barrels[b].1).reverse())
    });

    let mut min: Vec<(i64, isize)> = Vec::with_capacity(barrels.len());
    let mut prev = vec![-1; barrels.len()];

    for i in 0..barrels.len() {
        let c = barrels[order[i]].1;
        let f = min.binary_search_by(|m| m.0.cmp(&c));
        if let Err(n) = f {
            if n >= min.len() {
                min.push((c, i as isize));
            } else {
                min[n] = (c, i as isize);
            }
            if n > 0 {
                prev[i] = min[n - 1].1;
            }
        }
    }
    let mut solution = vec![];
    let mut s = min[min.len() - 1].1;
    while s >= 0 {
        solution.push(order[s as usize] + 1);
        s = prev[s as usize];
    }

    let elapsed = now.elapsed().as_micros();
    println!("Core runtime: {} sec", elapsed as f64 / 1000000.0);

    json!({ "barrelSequence": Value::from(solution) })
}

use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;
use std::iter::successors;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(alias = "barrelEthanolPairs")]
    barrels: Vec<Vec<i64>>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let now = Instant::now();

    let bar: Vec<_> = input.barrels.iter().map(|b| (b[0], b[1])).collect();
    let mut order: Vec<_> = (0..bar.len()).collect();
    order.sort_by(|&a, &b| bar[b].0.cmp(&bar[a].0).then(bar[b].1.cmp(&bar[a].1)));

    let mut opt: Vec<(i64, usize)> = Vec::with_capacity(bar.len());
    let mut next_barrel = vec![None; bar.len()];

    for i in 0..bar.len() {
        let alcohol = bar[order[i]].1;
        let f = opt.binary_search_by_key(&alcohol, |m| m.0);
        if let Err(n) = f {
            if n >= opt.len() {
                opt.push((alcohol, i));
            } else {
                opt[n] = (alcohol, i);
            }
            if n > 0 {
                next_barrel[i] = Some(opt[n - 1].1);
            }
        }
    }
    let solution: Vec<_> = successors(Some(opt.last().unwrap().1), |&s| next_barrel[s])
        .map(|n| order[n] + 1)
        .collect();

    let elapsed = now.elapsed().as_micros();
    println!("Core runtime: {} sec", elapsed as f64 / 1000000.0);

    json!({ "barrelSequence": Value::from(solution) })
}

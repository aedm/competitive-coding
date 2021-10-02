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

    let mut barrels: Vec<_> = (0..input.barrels.len()).collect();
    barrels.sort_by(|a, b| input.barrels[*a][0].cmp(&input.barrels[*b][0]));

    let mut back = vec![(0, -1i64); barrels.len()];
    for i in 0..barrels.len() {
        let c = &input.barrels[barrels[i]];
        let mut mx = (0, -1i64);
        for o in 0..i {
            let b = &input.barrels[barrels[o]];
            if b[0] < c[0] && b[1] > c[1] && mx.0 < back[o].0 {
                mx = (back[o].0, o as i64);
            }
        }
        back[i] = (mx.0 + 1i64, mx.1);
    }
    // barrels
    //     .iter()
    //     .for_each(|x| println!(":{:?}", input.barrels[*x]));
    // println!("{:?}", barrels);
    // println!("{:?}", back);
    let mut s: i64 = (0..barrels.len())
        .max_by(|a, b| back[*a].0.cmp(&back[*b].0))
        .unwrap() as i64;
    // println!("{:?}", s);
    let mut solution = Vec::new();
    loop {
        solution.push(barrels[s as usize] + 1);
        s = back[s as usize].1;
        if s < 0 {
            break;
        }
    }
    solution.reverse();

    json!({ "barrelSequence": Value::from(solution) })
}

use crate::utils::neighbours4;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Deserialize, Debug)]
struct Input {
    square: Vec<Vec<i32>>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let input = input.square.into_iter().flatten().collect_vec();

    let m = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    let magic = (1..=9)
        .permutations(9)
        .map(|p| {
            for n in &m {
                if p[n[0]] + p[n[1]] + p[n[2]] != 15 {
                    return 10000;
                }
            }
            (0..9).map(|i| i32::abs(p[i] - input[i])).sum::<i32>()
        })
        .min()
        .unwrap();

    json!(magic)
}

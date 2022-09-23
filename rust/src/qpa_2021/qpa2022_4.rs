use crate::utils::neighbours4;
use itertools::Itertools;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Deserialize, PartialEq, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Deserialize, Debug)]
struct Input {
    #[serde(alias = "boardSize")]
    board_size: usize,
    obstacles: Vec<Coord>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let s: i64 = input.board_size as i64;
    let set: HashSet<_> = input.obstacles.iter().map(|c| (c.x, c.y)).collect();

    let (mut minx, mut miny, mut minv) = (0, 0, -1);

    let ds = (-1..=1).cartesian_product(-1..=1).filter(|c| *c != (0, 0)).collect_vec();
    println!("ds: {:?}", ds);
    for y in 0..s {
        for x in 0..s {
            if set.contains(&(x, y)) {
                continue;
            }
            let mut v = 0;
            for &d in ds.iter() {
                for i in 1.. {
                    let x1 = x + d.0 * i;
                    let y1 = y + d.1 * i;
                    if x1 < 0 || y1 < 0 || x1 >= s || y1 >= s || set.contains(&(x1, y1)) {
                        break;
                    }
                    v += 1;
                }
            }
            if v > minv {
                minv = v;
                minx = x;
                miny = y;
            }
        }
    }

    json!({"x": minx, "y": miny})
}

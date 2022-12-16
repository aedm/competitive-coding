use crate::utils::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn read_valves() -> (Vec<(i64, Vec<usize>)>, usize) {
    let pattern: Regex =
        Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.*)$")
            .unwrap();
    let input = read_lines("advent_2022/16.txt")
        .into_iter()
        .map(|s| {
            let caps = pattern.captures(&s).unwrap();
            (
                caps[1].to_string(),
                caps[2].parse::<i64>().unwrap(),
                caps[3].split(", ").map(|s| s.to_string()).collect_vec(),
            )
        })
        .sorted_by_key(|&(_, flow, _)| -flow)
        .collect_vec();
    let valve_ids: HashMap<_, _> =
        input.iter().enumerate().map(|(i, (name, _, _))| (name.clone(), i)).collect();
    let valves = input
        .iter()
        .map(|valve| {
            (
                valve.1,
                valve.2.iter().map(|s| valve_ids.get(s).unwrap()).copied().collect_vec(),
            )
        })
        .collect();
    let start = valve_ids["AA"];

    (valves, start)
}

pub fn solve_1() -> i64 {
    let (valves, start) = read_valves();
    let mut state = HashMap::from([((start, 0), 0)]);
    let non_empty_count = valves.iter().filter(|v| v.0 > 0).count();

    for time in 1..=30 {
        let mut new_state = HashMap::new();
        for ((pos, opened), total) in state {
            if pos < non_empty_count && (opened & (1 << pos)) == 0 {
                let new_key = (pos, opened | (1 << pos));
                let new_total = total + valves[pos].0 * (30 - time);
                if !new_state.contains_key(&new_key) || new_state[&new_key] < new_total {
                    new_state.insert(new_key, new_total);
                }
            }
            for &next_valve in &valves[pos].1 {
                let new_key = (next_valve, opened);
                if !new_state.contains_key(&new_key) || new_state[&new_key] < total {
                    new_state.insert(new_key, total);
                }
            }
        }
        state = new_state;
    }

    *state.values().max().unwrap()
}

pub fn solve_2() -> i64 {
    let (valves, start) = read_valves();
    let mut state = HashMap::from([((start, start, 0), 0)]);
    let mut maxs = HashMap::from([((start, start, 0), 0)]);
    let non_empty_count = valves.iter().filter(|v| v.0 > 0).count();
    let mut m = 0;
    for time in 1..=26 {
        for _ in 0..2 {
            let mut new_state = HashMap::new();
            for (&(pos, other, opened), &total) in &state {
                if pos < non_empty_count && (opened & (1 << pos)) == 0 {
                    let new_key = (other, pos, opened | (1 << pos));
                    let new_total = total + valves[pos].0 * (26 - time);
                    if maxs.get(&new_key).cloned().unwrap_or_default() < new_total {
                        new_state.insert(new_key, new_total);
                        maxs.insert(new_key, new_total);
                        if new_total > m {
                            m = new_total;
                        }
                    }
                }
                for &next_valve in &valves[pos].1 {
                    let new_key = (other, next_valve, opened);
                    if let Some(v) = maxs.get(&new_key) {
                        if *v >= total {
                            continue;
                        }
                    }
                    new_state.insert(new_key, total);
                    maxs.insert(new_key, total);
                }
            }
            state = new_state;
        }
    }
    m
}

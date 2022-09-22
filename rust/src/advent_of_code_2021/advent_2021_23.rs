use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::hash_map::Entry;
use std::collections::hash_set::Union;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

type State = [u8; 15];
type State2 = [u8; 23];

const INPUT_TEST: &State = &[0, 0, 0, 0, 0, 0, 0, 2, 1, 3, 4, 2, 3, 4, 1];
const INPUT: &State = &[0, 0, 0, 0, 0, 0, 0, 4, 2, 1, 3, 4, 2, 3, 1];

const INPUT_TEST_2: &State2 = &[
    0, 0, 0, 0, 0, 0, 0, 2, 4, 4, 1, 3, 3, 2, 4, 2, 2, 1, 3, 4, 1, 3, 1,
];
const INPUT_2: &State2 = &[
    0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 2, 1, 3, 2, 3, 4, 2, 1, 2, 3, 1, 3, 1,
];

const TARGET: &State = &[0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4];
const TARGET_2: &State2 = &[
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4,
];
const COST: &[i64] = &[1, 10, 100, 1000];

fn enqueue(
    s: State,
    states: &mut HashMap<State, i64>,
    queue: &mut VecDeque<State>,
    cost: i64,
    o: &State,
    prev: &mut HashMap<State, State>,
) -> bool {
    if *states.get(&s).unwrap_or(&(cost + 1)) <= cost {
        return false;
    }
    // println!("-- new {:?} {}", s, cost);
    states.insert(s, cost);
    queue.push_back(s);
    prev.insert(s, *o);
    true
}

fn enqueue2(
    s: State2,
    states: &mut HashMap<State2, i64>,
    queue: &mut VecDeque<State2>,
    cost: i64,
    o: &State2,
    prev: &mut HashMap<State2, State2>,
) -> bool {
    if *states.get(&s).unwrap_or(&(cost + 1)) <= cost {
        return false;
    }
    // println!("-- new {:?} {}", s, cost);
    states.insert(s, cost);
    queue.push_back(s);
    prev.insert(s, *o);
    true
}

pub fn solve_1() -> i64 {
    let mut states = HashMap::<State, i64>::new();
    let mut prev = HashMap::<State, State>::new();
    let mut q = VecDeque::<State>::new();
    q.push_back(INPUT.clone());
    states.insert(INPUT.clone(), 0);
    while let Some(s) = q.pop_front() {
        let cost = *states.get(&s).unwrap();
        // println!("> {:?} {}", s, cost);
        for i in (0..15).filter(|&x| s[x] != 0) {
            let p = s[i] as usize;
            // println!(">> {} {}", i, p);
            if i < 7 {
                let slot = 5 + p * 2;
                if s[slot] != 0 || (s[slot + 1] != 0 && s[slot + 1] != p as u8) {
                    continue;
                }
                let (st, en) = if i > p { (p + 1, i - 1) } else { (i + 1, p) };
                if s[st..=en].iter().any(|&c| c != 0) {
                    continue;
                }
                let dest = slot + (s[slot + 1] == 0) as usize;
                let mut n = s.clone();
                n[i] = 0;
                n[dest] = p as u8;

                let edge = (i == 0 || i == 6) as i64;
                // println!("---- {} {} {} {} {}", en, st, dest, slot, edge);
                let nc = cost
                    + ((en as i64 - st as i64 + 2) * 2 + dest as i64 - slot as i64 - edge)
                        * COST[p as usize - 1];
                if enqueue(n, &mut states, &mut q, nc, &s, &mut prev) {
                    // println!("-- new {:?} {}", n, nc);
                }
            } else {
                let right_slot = 5 + p * 2;
                if i % 2 == 0 && ((right_slot == i - 1) || s[i - 1] != 0) {
                    continue;
                }
                if i % 2 == 1 && right_slot == i && s[i + 1] == p as u8 {
                    continue;
                }
                for ii in 0..7 {
                    let slot = (i - 5) / 2;
                    let (st, en) = if ii > slot { (slot + 1, ii) } else { (ii, slot) };
                    if s[st..=en].iter().any(|&c| c != 0) {
                        continue;
                    }
                    let mut n = s.clone();
                    n[i] = 0;
                    n[ii] = p as u8;

                    let edge = (ii == 0 || ii == 6) as usize;
                    let nc = cost
                        + ((en - st + 1) * 2 + 1 - (i % 2) - edge) as i64 * COST[p as usize - 1];
                    if enqueue(n, &mut states, &mut q, nc, &s, &mut prev) {
                        // println!("-- new {:?} {}", n, nc);
                    }
                }
            };
        }
    }

    let mut s = *TARGET;
    loop {
        let cost = *states.get(&s).unwrap();
        println!(". {:?} {}", s, cost);
        if let Some(x) = prev.get(&s) {
            s = *x;
        } else {
            break;
        }
    }

    *states.get(TARGET).unwrap()
}

pub fn solve() -> i64 {
    let mut states = HashMap::<State2, i64>::new();
    let mut prev = HashMap::<State2, State2>::new();
    let mut q = VecDeque::<State2>::new();
    q.push_back(INPUT_2.clone());
    states.insert(INPUT_2.clone(), 0);
    while let Some(s) = q.pop_front() {
        let cost = *states.get(&s).unwrap();
        for i in (0..23).filter(|&x| s[x] != 0) {
            let p = s[i] as usize;
            if i < 7 {
                let slot = 3 + p * 4;
                if s[slot] != 0 || s[slot + 1..slot + 4].iter().any(|&v| v != 0 && v != p as u8) {
                    continue;
                }
                let (st, en) = if i > p { (p + 1, i - 1) } else { (i + 1, p) };
                if s[st..=en].iter().any(|&c| c != 0) {
                    continue;
                }
                let dest = slot + 3 - (0..4).find(|v| s[slot + 3 - v] == 0).unwrap();
                let mut n = s.clone();
                n[i] = 0;
                n[dest] = p as u8;

                let edge = (i == 0 || i == 6) as i64;
                let nc = cost
                    + ((en as i64 - st as i64 + 2) * 2 + dest as i64 - slot as i64 - edge)
                        * COST[p as usize - 1];
                if enqueue2(n, &mut states, &mut q, nc, &s, &mut prev) {
                    // println!("!!{:?} {}", n, nc);
                }
            } else {
                let slot = i - (i + 1) % 4;
                let right_slot = 3 + p * 4;
                if right_slot == slot && (0..4).all(|v| s[slot + v] == 0 || s[slot + v] == p as u8)
                {
                    continue;
                }
                if (slot..i).any(|v| s[v] != 0) {
                    continue;
                }
                for ii in 0..7 {
                    let slot = (i - 3) / 4;
                    let (st, en) = if ii > slot { (slot + 1, ii) } else { (ii, slot) };
                    if s[st..=en].iter().any(|&c| c != 0) {
                        continue;
                    }
                    let mut n = s.clone();
                    n[i] = 0;
                    n[ii] = p as u8;

                    let edge = (ii == 0 || ii == 6) as usize;
                    let nc = cost
                        + ((en - st + 1) * 2 + ((i + 1) % 4) - edge) as i64 * COST[p as usize - 1];
                    if enqueue2(n, &mut states, &mut q, nc, &s, &mut prev) {
                        // println!("!!{:?} {}", n, nc);
                    }
                }
            };
        }
    }

    *states.get(TARGET_2).unwrap()
}

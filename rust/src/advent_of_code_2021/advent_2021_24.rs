use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::hash_set::Union;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve_1() -> i64 {
    let s = [
        (1, 11, 16),
        (1, 12, 11),
        (1, 13, 12),
        (26, -5, 12),
        (26, -3, 12),
        (1, 14, 2),
        (1, 15, 11),
        (26, -16, 4),
        (1, 14, 12),
        (1, 15, 9),
        (26, -7, 10),
        (26, -11, 11),
        (26, -6, 6),
        (26, -11, 15),
    ];
    let mut zs = HashSet::new();
    zs.insert((0, 0, 0));
    let mut v = vec![];

    for &(a, b, c) in s.iter().rev() {
        let mut hs = HashSet::new();
        for &(oz, last_imp, last_z) in &zs {
            for inp in 1..=9 {
                // eq
                for z in oz * a..(oz + 1) * a {
                    if z % 26 + b == inp {
                        hs.insert((z, inp, oz));
                    }
                }
                let dz = oz - c - inp;
                if dz % 26 == 0 {
                    let fz = dz / 26;
                    for z in fz * a..(fz + 1) * a {
                        if z % 26 + b != inp {
                            hs.insert((z, inp, oz));
                        }
                    }
                }
            }
        }
        v.push(hs.clone());
        zs = hs;
    }

    let mut res = 0;
    let mut az: Option<HashSet<i64>> = None;
    for hs in v.iter().rev() {
        let p = if let Some(x) = az {
            hs.iter().filter(|&c| x.contains(&c.0)).cloned().collect()
        } else {
            hs.clone()
        };
        let max = p.iter().map(|&c| c.1).max().unwrap();
        res = res * 10 + max;
        az = Some(p.iter().map(|&c| c.2).collect());
    }

    res
}

pub fn solve_2() -> i64 {
    let s = [
        (1, 11, 16),
        (1, 12, 11),
        (1, 13, 12),
        (26, -5, 12),
        (26, -3, 12),
        (1, 14, 2),
        (1, 15, 11),
        (26, -16, 4),
        (1, 14, 12),
        (1, 15, 9),
        (26, -7, 10),
        (26, -11, 11),
        (26, -6, 6),
        (26, -11, 15),
    ];
    let mut zs = HashSet::new();
    zs.insert((0, 0, 0));
    let mut v = vec![];

    for &(a, b, c) in s.iter().rev() {
        let mut hs = HashSet::new();
        for &(oz, last_imp, last_z) in &zs {
            for inp in 1..=9 {
                // eq
                for z in oz * a..(oz + 1) * a {
                    if z % 26 + b == inp {
                        hs.insert((z, inp, oz));
                    }
                }
                let dz = oz - c - inp;
                if dz % 26 == 0 {
                    let fz = dz / 26;
                    for z in fz * a..(fz + 1) * a {
                        if z % 26 + b != inp {
                            hs.insert((z, inp, oz));
                        }
                    }
                }
            }
        }
        v.push(hs.clone());
        zs = hs;
    }

    let mut res = 0;
    let mut az: Option<HashSet<i64>> = None;
    for hs in v.iter().rev() {
        let p = if let Some(x) = az {
            hs.iter().filter(|&c| x.contains(&c.0)).cloned().collect()
        } else {
            hs.clone()
        };
        let max = p.iter().map(|&c| c.1).min().unwrap();
        res = res * 10 + max;
        az = Some(p.iter().map(|&c| c.2).collect());
    }

    res
}

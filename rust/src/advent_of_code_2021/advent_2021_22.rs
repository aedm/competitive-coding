use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::hash_set::Union;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

type Cube = (i64, i64, i64, i64, i64, i64);

fn sub(a: &Cube, b: &Cube) -> HashSet<Cube> {
    let mut a = *a;
    let b = *b;
    let mut m = HashSet::new();
    if a.0 < b.0 {
        let n = min(a.1, b.0 - 1);
        m.insert((a.0, n, a.2, a.3, a.4, a.5));
        a.0 = n + 1;
        if a.0 > a.1 {
            return m;
        }
    }
    if a.1 > b.1 {
        let n = max(a.0, b.1 + 1);
        m.insert((n, a.1, a.2, a.3, a.4, a.5));
        a.1 = n - 1;
        if a.0 > a.1 {
            return m;
        }
    }
    if a.2 < b.2 {
        let n = min(a.3, b.2 - 1);
        m.insert((a.0, a.1, a.2, n, a.4, a.5));
        a.2 = n + 1;
        if a.2 > a.3 {
            return m;
        }
    }
    if a.3 > b.3 {
        let n = max(a.2, b.3 + 1);
        m.insert((a.0, a.1, n, a.3, a.4, a.5));
        a.3 = n - 1;
        if a.2 > a.3 {
            return m;
        }
    }
    if a.4 < b.4 {
        let n = min(a.5, b.4 - 1);
        m.insert((a.0, a.1, a.2, a.3, a.4, n));
        a.4 = n + 1;
        if a.4 > a.5 {
            return m;
        }
    }
    if a.5 > b.5 {
        let n = max(a.4, b.5 + 1);
        m.insert((a.0, a.1, a.2, a.3, n, a.5));
        a.5 = n - 1;
        if a.4 > a.5 {
            return m;
        }
    }
    m
}

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2021/22.txt");
    let pattern: Regex =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();
    let l = lines
        .iter()
        .map(|l| {
            let caps = pattern.captures(l).unwrap();
            let b = caps.get(1).unwrap().as_str() == "on";
            let r = (2..=7)
                .map(|i| caps.get(i).unwrap().as_str().parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64, i64, i64, i64, i64)>();
            (b, r.unwrap())
        })
        .collect_vec();

    let mut cs = HashSet::new();
    for (i, &(on, cube)) in l.iter().enumerate() {
        let mut cs2 = HashSet::new();
        for c2 in &cs {
            sub(c2, &cube).iter().for_each(|c| {
                cs2.insert(*c);
            });
        }
        if on {
            cs2.insert(cube);
        }
        cs = cs2;
    }

    cs = cs
        .iter()
        .map(|c| {
            (
                max(c.0, -50),
                min(c.1, 50),
                max(c.2, -50),
                min(c.3, 50),
                max(c.4, -50),
                min(c.5, 50),
            )
        })
        .collect();

    cs.iter().map(|c| max(c.1 - c.0 + 1, 0) * max(c.3 - c.2 + 1, 0) * max(c.5 - c.4 + 1, 0)).sum()
}

// Task 2, Vava's idea

fn area(cubes: &[(bool, &[i64])]) -> i64 {
    if cubes.len() == 0 {
        return 0;
    }
    if cubes[0].1.len() == 0 {
        return cubes[cubes.len() - 1].0 as i64;
    }
    cubes
        .iter()
        .flat_map(|&c| c.1[0..2].iter())
        .cloned()
        .sorted()
        .tuple_windows()
        .map(|(start, end)| {
            let slices = cubes
                .iter()
                .filter(|&c| c.1[0] <= start && c.1[1] >= end)
                .map(|&(on, c)| (on, &c[2..]))
                .collect_vec();
            (end - start) * area(&slices)
        })
        .sum()
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2021/22.txt");
    let pattern =
        Regex::new(r"^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$")
            .unwrap();
    let l = lines
        .iter()
        .map(|l| {
            let caps = pattern.captures(l).unwrap();
            let on = caps.get(1).unwrap().as_str() == "on";
            let cube = (2..=7)
                .map(|i| caps.get(i).unwrap().as_str().parse::<i64>().unwrap() + i as i64 % 2)
                .collect_vec();
            (on, cube)
        })
        .collect_vec();

    let cube_slices = l.iter().map(|(on, c)| (*on, &c[..])).collect_vec();
    area(&cube_slices)
}

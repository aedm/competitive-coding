use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

type Coord = (i64, i64, i64);
type Matrix = (Coord, Coord, Coord);

fn cross(a: &Coord, b: &Coord) -> Coord {
    (
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

fn sub(a: &Coord, b: &Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add(a: &Coord, b: &Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn mult_matrix(c: &Coord, m: &Matrix) -> Coord {
    (
        m.0 .0 * c.0 + m.1 .0 * c.1 + m.2 .0 * c.2,
        m.0 .1 * c.0 + m.1 .1 * c.1 + m.2 .1 * c.2,
        m.0 .2 * c.0 + m.1 .2 * c.1 + m.2 .2 * c.2,
    )
}

fn read() -> Vec<HashSet<Coord>> {
    let pattern: Regex = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    let mut lines = read_lines("advent_2021/19.txt");
    let mut v = vec![];
    let mut k = HashSet::new();
    for l in lines {
        if l.len() == 0 {
            continue;
        }
        if pattern.is_match(&l) {
            if k.len() > 0 {
                v.push(k);
            }
            k = HashSet::new();
        } else {
            k.insert(
                l.split(',').map(|s| s.parse::<i64>().unwrap()).collect_tuple::<Coord>().unwrap(),
            );
        }
    }
    v.push(k);
    v
}

fn build_matrices() -> Vec<Matrix> {
    let v = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut m = vec![];
    for &x in &v {
        for &y in &v {
            let z = cross(&x, &y);
            if z.0 != 0 || z.1 != 0 || z.2 != 0 {
                m.push((x, y, z));
            }
        }
    }
    m
}

fn project(c: &Coord, source_origin: &Coord, target_origin: &Coord, rotate: &Matrix) -> Coord {
    add(&mult_matrix(&sub(c, source_origin), rotate), target_origin)
}

fn match_scanners(
    origin_a: &Coord,
    a: &HashSet<Coord>,
    b: &HashSet<Coord>,
    ms: &[Matrix],
) -> Option<(Coord, Coord, usize)> {
    for (im, m) in ms.iter().enumerate() {
        for match_a in a {
            'inner: for match_b in b {
                let mut count = 0;
                for c in b {
                    let r = project(c, match_b, match_a, m);
                    if (origin_a.0 - r.0).abs() <= 1000
                        && (origin_a.1 - r.1).abs() <= 1000
                        && (origin_a.2 - r.2).abs() <= 1000
                    {
                        if a.contains(&r) {
                            count += 1;
                        } else {
                            continue 'inner;
                        }
                    }
                }
                if count >= 12 {
                    return Some((*match_a, *match_b, im));
                }
            }
        }
    }
    None
}

pub fn solve() -> (i64, i64) {
    let mut s = read();
    let ms = build_matrices();

    let mut origins = vec![(0, 0, 0); s.len()];
    let mut left = (1..s.len()).collect_vec();
    let mut done = vec![0];
    let mut beacons = s[0].clone();
    while left.len() > 0 {
        let mut ss = left;
        left = vec![];
        for i in ss {
            if let Some((target, source, rotate)) =
                done.iter().find_map(|&t| match_scanners(&origins[t], &s[t], &s[i], &ms))
            {
                done.push(i);
                s[i] = s[i].iter().map(|c| project(c, &source, &target, &ms[rotate])).collect();
                origins[i] = project(&(0, 0, 0), &source, &target, &ms[rotate]);
                beacons = beacons.union(&s[i]).cloned().collect();
            } else {
                left.push(i);
            }
        }
    }

    let task_1 = beacons.len() as i64;
    let task_2 = origins
        .iter()
        .map(|o1| {
            origins
                .iter()
                .map(|o2| (o1.0 - o2.0).abs() + (o1.1 - o2.1).abs() + (o1.2 - o2.2).abs())
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    (task_1, task_2)
}

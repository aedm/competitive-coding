use crate::utils::array2d::{IVec2D, Map2D, DIRS4};
use crate::utils::{read_lines, read_lines_split};
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

pub fn solve_1() -> impl Debug {
    let mut m = HashSet::new();
    let mut p = IVec2D::new(0, 0);
    m.insert(p);
    let ds = HashMap::from([('R', 3), ('L', 1), ('U', 0), ('D', 2)]);
    for l in read_lines_split("advent_2023/18.txt", &[' ']) {
        let d = DIRS4[ds[&l[0].chars().nth(0).unwrap()]];
        let len = l[1].parse::<i64>().unwrap();
        for i in 0..len {
            p = p + d;
            m.insert(p);
        }
    }
    let MinMax(ymin, ymax) = m.iter().map(|p| p.y).minmax() else {
        panic!()
    };
    let MinMax(xmin, xmax) = m.iter().map(|p| p.x).minmax() else {
        panic!()
    };
    let mut m2 = Map2D {
        w: xmax - xmin + 3,
        h: ymax - ymin + 3,
        items: vec![false; ((xmax - xmin + 3) * (ymax - ymin + 3)) as usize],
    };
    for c in &m {
        m2[*c + IVec2D::new(-xmin + 1, -ymin + 1)] = true;
    }
    let mut queue = VecDeque::from([IVec2D::new(0, 0)]);
    while let Some(c) = queue.pop_front() {
        for d in DIRS4 {
            if let Some((nc, false)) = m2.add_coord(c, *d) {
                queue.push_back(nc);
                m2[nc] = true;
            }
        }
    }
    for c in &m {
        m2[*c + IVec2D::new(-xmin + 1, -ymin + 1)] = false;
    }
    m2.filter(|_, v| !*v).count()
}

pub fn solve_2() -> i64 {
    let mut rows = HashMap::<i64, Vec<i64>>::new();
    let mut cols = HashMap::<i64, Vec<(i64, i64)>>::new();
    let mut p = IVec2D::new(0, 0);
    let ds = HashMap::from([('0', 3), ('2', 1), ('3', 0), ('1', 2)]);
    let l = read_lines_split("advent_2023/18.txt", &[' '])
        .into_iter()
        .map(|l| {
            let d = ds[&l[2].chars().nth(7).unwrap()];
            let len = i64::from_str_radix(&l[2][2..7], 16).unwrap();
            (len, d)
        })
        .collect::<Vec<_>>();

    for i in 0..l.len() {
        let (len, d) = l[i];
        let p2 = IVec2D::new(p.x + DIRS4[d].x * len, p.y + DIRS4[d].y * len);
        let (minx, maxx) = (p.x.min(p2.x), p.x.max(p2.x));
        let (miny, maxy) = (p.y.min(p2.y), p.y.max(p2.y));
        if d == 0 || d == 2 {
            for y in miny + 1..=maxy {
                rows.entry(y).or_insert_with(|| vec![]).push(p.x);
            }
        } else {
            cols.entry(p.y).or_insert_with(|| vec![]).push((minx, maxx));
        }
        p = p2;
    }

    let mut count = 0;
    for (y, mut row) in rows {
        let mut blocks = cols.get(&y).cloned().unwrap_or(vec![]);
        blocks.sort();
        row.sort();
        for (x1, x2) in row.into_iter().tuples() {
            count += x2 - x1 - 1;
            for &(b1, b2) in &blocks {
                let v1 = b1.max(x1 + 1);
                let v2 = b2.min(x2 - 1);
                if v1 <= v2 {
                    count -= v2 - v1 + 1;
                }
            }
        }
    }

    count + l.iter().map(|(len, _)| len).sum::<i64>()
}

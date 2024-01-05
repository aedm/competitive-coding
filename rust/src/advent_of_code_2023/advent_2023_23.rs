use crate::utils::array2d::{v, IVec2D, Map2D, DIRS4};
use crate::utils::{read_lines, read_lines_split};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

// Sorry, cleaning up didn't spark joy

fn dfs(
    m: &Map2D<char>,
    visited: &mut HashSet<IVec2D>,
    pos: IVec2D,
    len: i64,
    end: IVec2D,
) -> Option<i64> {
    let ds = HashMap::from([('^', 0), ('<', 1), ('v', 2), ('>', 3)]);
    let mut result = None;
    for d in 0..4 {
        if m[pos] != '.' && d != ds[&m[pos]] {
            continue;
        }
        if let Some((nc, t)) = m.add_coord(pos, DIRS4[d]) {
            if t != '#' && !visited.contains(&nc) {
                if nc == end {
                    result = Some(len + 1);
                    break;
                }

                visited.insert(nc);
                let l = dfs(m, visited, nc, len + 1, end);
                visited.remove(&nc);

                if l.is_some() {
                    if result.is_none() || l.unwrap() > result.unwrap() {
                        result = l;
                    }
                }
            }
        }
    }
    result
}

pub fn solve_1() -> impl Debug {
    let mut m = Map2D::from_text(&read_lines("advent_2023/23.txt"));
    let s = v((0..m.w).find(|&x| m[v(x, 0)] == '.').unwrap(), 0);
    let e = v(
        (0..m.w).find(|&x| m[v(x, m.h - 1)] == '.').unwrap(),
        m.h - 1,
    );
    dfs(&m, &mut HashSet::new(), s, 0, e).unwrap()
}

fn dfs2(
    paths: &HashMap<IVec2D, Vec<(IVec2D, i64)>>,
    visited: &mut HashSet<IVec2D>,
    pos: IVec2D,
    len: i64,
    end: IVec2D,
    depth: usize,
) -> Option<i64> {
    if pos == end {
        return Some(len);
    }
    visited.insert(pos);
    let mut result = None;
    for &(node, plen) in &paths[&pos] {
        if visited.contains(&node) {
            continue;
        }
        if let Some(l) = dfs2(paths, visited, node, len + plen, end, depth + 1) {
            if result.is_none() || l > result.unwrap() {
                result = Some(l);
            }
        }
    }
    visited.remove(&pos);
    result
}

pub fn solve_2() -> impl Debug {
    let mut m = Map2D::from_text(&read_lines("advent_2023/23.txt"));
    let mut nodes = m
        .filter(|c, v| {
            if *v != '#' {
                return DIRS4
                    .iter()
                    .filter(|d| {
                        if let Some((_, nv)) = m.add_coord(c, **d) {
                            return nv != '#';
                        }
                        false
                    })
                    .count()
                    > 2;
            }
            false
        })
        .map(|(c, _)| c)
        .collect::<HashSet<_>>();

    let s = v((0..m.w).find(|&x| m[v(x, 0)] == '.').unwrap(), 0);
    let e = v(
        (0..m.w).find(|&x| m[v(x, m.h - 1)] == '.').unwrap(),
        m.h - 1,
    );

    nodes.insert(s);
    nodes.insert(e);

    let mut paths = nodes.iter().map(|&c| (c, Vec::new())).collect::<HashMap<_, _>>();
    for &c in &nodes {
        for d in 0..4 {
            let mut pc = c;
            let mut pd = d;
            let mut l = 1;
            while let Some((nc, t)) = m.add_coord(pc, DIRS4[pd]) {
                if t == '#' {
                    break;
                }
                if nodes.contains(&nc) {
                    paths.get_mut(&c).unwrap().push((nc, l));
                    break;
                }
                pc = nc;
                l += 1;
                pd = (0..4)
                    .find(|&d| {
                        if let Some((_, t)) = m.add_coord(pc, DIRS4[d]) {
                            return t != '#' && d == (pd + 2) % 4;
                        }
                        false
                    })
                    .unwrap();
            }
        }
    }

    dfs2(&paths, &mut HashSet::new(), s, 0, e, 0).unwrap()
}

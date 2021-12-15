use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn solve(costs: &[Vec<i64>]) -> i64 {
    let (mx, my) = (costs[0].len(), costs.len());
    let dirs = [(1i64, 0), (-1, 0), (0, 1), (0, -1)];
    let mut reach = BTreeSet::<(i64, usize, usize)>::new();
    let mut path_cost = vec![vec![i64::MAX; mx]; my];
    path_cost[0][0] = 0;

    let mut now = (0, 0);
    let end = (mx - 1, my - 1);
    while now != end {
        let now_cost = path_cost[now.1][now.0];
        reach.remove(&(now_cost, now.0, now.1));

        for &(dx, dy) in &dirs {
            let (nx, ny) = (now.0 as i64 + dx, now.1 as i64 + dy);
            if nx == nx.clamp(0, mx as i64 - 1) && ny == ny.clamp(0, my as i64 - 1) {
                let (ux, uy) = (nx as usize, ny as usize);
                let pc = &mut path_cost[uy][ux];
                let c = costs[uy][ux];
                if *pc > now_cost + c {
                    reach.remove(&(*pc, ux, uy));
                    *pc = now_cost + c;
                    reach.insert((*pc, ux, uy));
                }
            }
        }
        let next = reach.iter().next().unwrap();
        now = (next.1, next.2);
    }
    path_cost[end.1][end.0]
}

pub fn solve_1() -> i64 {
    let mut lines = read_lines("advent_2021/15.txt");
    let costs =
        lines.iter().map(|l| l.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();
    solve(&costs)
}

pub fn solve_2() -> i64 {
    let mut lines = read_lines("advent_2021/15.txt");
    let costs_small =
        lines.iter().map(|l| l.bytes().map(|b| (b - b'0') as i64).collect_vec()).collect_vec();
    let (mx, my) = (costs_small[0].len(), costs_small.len());

    let mut costs = vec![vec![0; mx * 5]; my * 5];
    for y in 0..my * 5 {
        for x in 0..mx * 5 {
            if y >= my {
                costs[y][x] = if costs[y - my][x] > 8 { 1 } else { costs[y - my][x] + 1 };
            } else if x >= mx {
                costs[y][x] = if costs[y][x - mx] > 8 { 1 } else { costs[y][x - mx] + 1 };
            } else {
                costs[y][x] = costs_small[y][x];
            }
        }
    }
    solve(&costs)
}

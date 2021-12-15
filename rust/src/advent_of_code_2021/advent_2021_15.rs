use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn solve(costs: &[Vec<i64>]) -> i64 {
    let (mx, my) = (costs[0].len(), costs.len());
    let mut reach = BinaryHeap::<Reverse<(i64, usize, usize)>>::new();
    let mut path_cost = vec![vec![-1; mx]; my];
    path_cost[0][0] = 0;
    reach.push(Reverse((0, 0, 0)));

    while let Some(Reverse(n)) = reach.pop() {
        if n.1 != mx - 1 || n.2 != my - 1 {
            for (x, y) in neighbours4((n.1, n.2), 0, 0, mx, my) {
                if path_cost[y][x] < 0 {
                    let c = n.0 + costs[y][x];
                    path_cost[y][x] = c;
                    reach.push(Reverse((c, x, y)));
                }
            }
        }
    }
    path_cost[my - 1][mx - 1]
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
            costs[y][x] = if y >= my {
                costs[y - my][x] % 9 + 1
            } else if x >= mx {
                costs[y][x - mx] % 9 + 1
            } else {
                costs_small[y][x]
            }
        }
    }
    solve(&costs)
}

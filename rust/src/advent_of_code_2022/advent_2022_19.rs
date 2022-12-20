use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashSet;

fn get_qualities(rounds: usize, count: Option<usize>) -> Vec<i32> {
    let mut input = read_lines("advent_2022/19.txt")
        .into_iter()
        .map(|s| s.split(' ').map(|p| p.parse::<i32>()).flatten().collect_vec())
        .collect_vec();
    if let Some(count) = count {
        input = input[0..count].iter().cloned().collect_vec();
    }
    input
        .into_iter()
        .enumerate()
        .map(|(index, bp)| {
            let max_ore = max(max(max(bp[1], bp[2]), bp[4]), bp[0]);
            let max_clay = bp[3];
            let max_obsidian = bp[5];
            let deps = [
                [bp[0], 0, 0],
                [bp[1], 0, 0],
                [bp[2], bp[3], 0],
                [bp[4], 0, bp[5]],
            ];
            let mut states = HashSet::from([([1, 0, 0, 0], [0, 0, 0, 0])]);
            for i in 0..rounds {
                let mut candidates = Vec::new();
                for (b, r) in states {
                    for (di, dep) in deps.iter().enumerate() {
                        if r[0] >= dep[0] && r[1] >= dep[1] && r[2] >= dep[2] {
                            let mut nb = b.clone();
                            nb[di] += 1;
                            if nb[0] <= max_ore && nb[1] <= max_clay && nb[2] <= max_obsidian {
                                let new_res = [
                                    r[0] + b[0] - dep[0],
                                    r[1] + b[1] - dep[1],
                                    r[2] + b[2] - dep[2],
                                    r[3] + b[3],
                                ];
                                candidates.push((nb, new_res));
                            }
                        }
                    }
                    candidates.push((b, [r[0] + b[0], r[1] + b[1], r[2] + b[2], r[3] + b[3]]));
                }
                candidates.sort();
                let mut new_states = HashSet::new();
                for (i, (b, r)) in candidates.iter().enumerate() {
                    if !candidates.iter().skip(i).any(|(cb, cr)| {
                        (r[0] <= cr[0] && r[1] <= cr[1] && r[2] <= cr[2] && r[3] <= cr[3])
                            && (b[0] <= cb[0] && b[1] <= cb[1] && b[2] <= cb[2] && b[3] <= cb[3])
                            && ((r[0] < cr[0] || r[1] < cr[1] || r[2] < cr[2] || r[3] < cr[3])
                                || (b[0] < cb[0] || b[1] < cb[1] || b[2] < cb[2] || b[3] < cb[3]))
                    }) {
                        new_states.insert((*b, *r));
                    }
                }
                states = new_states;
            }
            states.into_iter().map(|(_, r)| r[3]).max().unwrap()
        })
        .collect_vec()
}

// Runtime: 0.401813 sec
pub fn solve_1() -> i32 {
    let r = get_qualities(24, None);
    r.iter().enumerate().map(|(i, v)| (i as i32 + 1) * *v).sum::<i32>()
}

// Runtime: 3.909407 sec
pub fn solve_2() -> i32 {
    let r = get_qualities(32, Some(3));
    r.iter().take(3).product()
}

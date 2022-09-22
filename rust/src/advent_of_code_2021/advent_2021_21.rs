use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn step(pos: &mut i64, dice_step: &mut i64, score: &mut i64) -> bool {
    for _ in 0..3 {
        *dice_step += 1;
        *pos += *dice_step;
    }
    *pos = (*pos - 1) % 10 + 1;
    *score += *pos;
    *score >= 1000
}

pub fn solve_1() -> i64 {
    let mut pos = (1, 10);
    let mut score = (0, 0);
    let mut dice_step = 0;
    loop {
        if step(&mut pos.0, &mut dice_step, &mut score.0) {
            return score.1 * dice_step;
        }
        if step(&mut pos.1, &mut dice_step, &mut score.1) {
            return score.0 * dice_step;
        }
    }
}

fn mm(
    m: &mut [i64],
    pos1: usize,
    pos2: usize,
    score1: usize,
    score2: usize,
    next: usize,
) -> &mut i64 {
    let index = (((score1 * 40 + score2) * 10 + pos1) * 10 + pos2) * 2 + next;
    &mut m[index]
}

pub fn solve_2() -> i64 {
    let mut m = vec![0i64; 40 * 40 * 10 * 10 * 2];
    *mm(&mut m, 0, 9, 0, 0, 0) = 1;

    let mut p = vec![0; 10];
    (1..=3)
        .cartesian_product(1..=3)
        .cartesian_product(1..=3)
        .for_each(|((a, b), c)| p[a + b + c] += 1);

    while (0..21)
        .cartesian_product(0..21)
        .cartesian_product(0..10)
        .cartesian_product(0..10)
        .cartesian_product(0..=1)
        .any(|((((score1, score2), pos1), pos2), next)| {
            *mm(&mut m, pos1, pos2, score1, score2, next) > 0
        })
    {
        for sc1 in (0..21).rev() {
            for sc2 in (0..21).rev() {
                for p1 in 0..10 {
                    for p2 in 0..10 {
                        let v1 = mm(&mut m, p1, p2, sc1, sc2, 0);
                        let s1 = *v1;
                        *v1 = 0;
                        let v2 = mm(&mut m, p1, p2, sc1, sc2, 1);
                        let s2 = *v2;
                        *v2 = 0;
                        for i in 3..p.len() {
                            let np1 = (p1 + i) % 10;
                            *mm(&mut m, np1, p2, sc1 + np1 + 1, sc2, 1) += s1 * p[i];
                            let np2 = (p2 + i) % 10;
                            *mm(&mut m, p1, np2, sc1, sc2 + np2 + 1, 0) += s2 * p[i];
                        }
                    }
                }
            }
        }
    }

    let mut a = [0, 0];
    for i in 0..m.len() {
        a[i % 2] += m[i];
    }
    max(a[0], a[1])
}

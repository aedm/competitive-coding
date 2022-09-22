use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve() -> () {
    let mut lines = read_lines("advent_2021/20.txt");
    let m = lines[0].chars().map(|c| (c == '#') as i64).collect_vec();
    let p = ['.', '#'];
    let mut s =
        lines[2..].iter().map(|l| l.chars().map(|c| (c == '#') as i64).collect_vec()).collect_vec();

    let mut def = 0;
    for i in 0..=50 {
        let lit = s.iter().flatten().cloned().sum::<i64>();
        println!("Step {} -> Lit {}", i, lit);

        let mut d = vec![vec![999_999; s[0].len() + 2]; s.len() + 2];
        let lx = d[0].len();
        let ly = d.len();
        for y in 0..ly {
            for x in 0..lx {
                let c = ((-1..=1).cartesian_product(-1..=1))
                    .map(|(dy, dx)| {
                        let mx = x as isize + dx - 1;
                        let my = y as isize + dy - 1;
                        if mx == mx.clamp(0, s[0].len() as isize - 1)
                            && my == my.clamp(0, s.len() as isize - 1)
                        {
                            s[my as usize][mx as usize]
                        } else {
                            def
                        }
                    })
                    .fold(0, |acc, a| acc * 2 + a);
                d[y][x] = m[c as usize];
            }
        }
        s = d;
        def = m[511 * def as usize];
    }
}

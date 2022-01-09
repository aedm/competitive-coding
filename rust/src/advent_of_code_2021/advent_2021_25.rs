use crate::utils::{neighbours4, read_lines};
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::cmp::{max, min, Reverse};
use std::collections::hash_set::Union;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

pub fn solve() -> i64 {
    let lines = read_lines("advent_2021/25.txt");
    let mp: HashMap<char, u8> = [('>', 1u8), ('v', 2), ('.', 0)].iter().cloned().collect();
    let mut t =
        lines.iter().map(|l| l.chars().map(|c| *mp.get(&c).unwrap()).collect_vec()).collect_vec();
    let (mx, my) = (t[0].len(), t.len());
    for i in 1.. {
        let mut change = false;
        let mut n = t.clone();
        for (y, x) in iproduct!(0..my, 0..mx) {
            if t[y][x] == 1 && t[y][(x + 1) % mx] == 0 {
                n[y][(x + 1) % mx] = 1;
                n[y][x] = 0;
                change = true;
            }
        }
        t = n.clone();
        for (y, x) in iproduct!(0..my, 0..mx) {
            if t[y][x] == 2 && t[(y + 1) % my][x] == 0 {
                n[(y + 1) % my][x] = 2;
                n[y][x] = 0;
                change = true;
            }
        }
        t = n;
        if !change {
            return i;
        }
    }
    panic!();
}

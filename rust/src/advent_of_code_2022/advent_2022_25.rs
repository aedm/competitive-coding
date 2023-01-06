use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() -> String {
    let snafu = HashMap::from([('2', 2), ('1', 1), ('0', 0), ('-', -1), ('=', -2)]);
    let mut sum = read_lines("advent_2022/25.txt")
        .into_iter()
        .map(|line| line.chars().fold(0, |acc, c| acc * 5 + snafu[&c]))
        .sum::<i64>();
    let mut n = 1;
    while n * 2 < sum {
        n *= 5;
    }
    let mut v = vec![];
    while n > 0 {
        let d = (sum + 2 * n + n / 2) / n - 2;
        v.push(['=', '-', '0', '1', '2'][(2 + d) as usize]);
        sum -= d * n;
        n /= 5;
    }
    v.into_iter().collect()
}

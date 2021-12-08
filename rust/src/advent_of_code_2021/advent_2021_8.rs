use crate::utils::read_lines;
use itertools::Itertools;
use std::cmp::max;
use std::collections::BTreeSet;
use std::iter::FromIterator;

pub fn solve_1() -> usize {
    let lines = read_lines("advent_2021/8.txt");
    lines
        .iter()
        .map(|l| l.split(" | ").collect_vec()[1].split(' '))
        .flatten()
        .filter(|&s| [2usize, 3, 4, 7].contains(&s.len()))
        .count()
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2021/8.txt");
    let digits = [
        vec![0usize, 1, 2, 4, 5, 6],
        vec![2, 5],
        vec![0, 2, 3, 4, 6],
        vec![0, 2, 3, 5, 6],
        vec![1, 2, 3, 5],
        vec![0, 1, 3, 5, 6],
        vec![0, 1, 3, 4, 5, 6],
        vec![0, 2, 5],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 5, 6],
    ];
    let mut sum = 0;
    for line in lines {
        let parts = line.split(" | ").map(|s| s.split(' ').collect_vec()).collect_vec();
        for perm in (0..7).permutations(7) {
            if parts[0].iter().all(|s| {
                let d = s.chars().map(|c| perm[c as usize - 'a' as usize]).sorted().collect_vec();
                digits.contains(&d)
            }) {
                let mut result = parts[1].iter().fold(0, |acc, s| {
                    let f =
                        s.chars().map(|c| perm[c as usize - 'a' as usize]).sorted().collect_vec();
                    let n = (0..10).find(|&n| digits[n] == f).unwrap();
                    acc * 10 + n
                });
                sum += result;
            }
        }
    }
    sum
}

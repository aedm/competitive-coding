use crate::utils::read_lines;
use itertools::Itertools;

fn solve(has_jokers: bool) -> usize {
    let figs = if has_jokers { "J23456789TQKA" } else { "_23456789TJQKA" };
    read_lines("advent_2023/7.txt")
        .into_iter()
        .map(|l| {
            let parts = l.split(' ').collect_vec();
            let cards = parts[0]
                .chars()
                .map(|c| figs.chars().find_position(|&f| f == c).unwrap().0)
                .collect_vec();
            let bid = parts[1].parse::<usize>().unwrap();
            let r = (1..figs.len())
                .map(|i| cards.iter().filter(|&&v| v == i).count())
                .sorted()
                .rev()
                .collect_vec();
            let j = if has_jokers { cards.iter().filter(|c| **c == 0).count() } else { 0 };
            let rank = match (r[0] + j, r[1]) {
                (5, _) => 6,
                (4, _) => 5,
                (3, 2) => 4,
                (3, _) => 3,
                (2, 2) => 2,
                (2, _) => 1,
                _ => 0,
            };
            (rank, cards, bid)
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum()
}

pub fn solve_1() -> usize {
    solve(false)
}

pub fn solve_2() -> usize {
    solve(true)
}

use crate::utils::read_lines;
use fraction::Integer;
use itertools::Itertools;
use std::mem;

// fn op(i: usize, v: i64) -> i64 {
//     match i {
//         0 => v * 19,
//         1 => v + 6,
//         2 => v * v,
//         3 => v + 3,
//         _ => panic!(),
//     }
// }

fn op(i: usize, v: i64) -> i64 {
    match i {
        0 => v * 3,
        1 => v + 8,
        2 => v * v,
        3 => v + 2,
        4 => v + 3,
        5 => v * 17,
        6 => v + 6,
        7 => v + 1,
        _ => panic!(),
    }
}

pub fn solve_1() -> i64 {
    let lines = read_lines("advent_2022/11.txt");
    let parts = lines
        .iter()
        .map(|l| l.split(&[' ', ',', ':']).map(|v| v.parse::<i64>()).flatten().collect_vec())
        .collect_vec();
    let mut monkeys = vec![];
    for i in 0..(lines.len() + 1) / 7 {
        monkeys.push((
            parts[i * 7 + 1].clone(),
            parts[i * 7 + 3][0],
            parts[i * 7 + 4][0],
            parts[i * 7 + 5][0],
            0,
        ));
    }
    for i in 0..20 {
        for mi in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[mi].0);
            for item in items {
                let mut v = item;
                v = op(mi, v) / 3;
                let dest = if v % monkeys[mi].1 == 0 { monkeys[mi].2 } else { monkeys[mi].3 };
                monkeys[dest as usize].0.push(v);
                monkeys[mi].4 += 1;
            }
        }
    }
    monkeys.iter().map(|m| m.4).sorted().rev().take(2).product::<i64>()
}

pub fn solve_2() -> i64 {
    let lines = read_lines("advent_2022/11.txt");
    let parts = lines
        .iter()
        .map(|l| l.split(&[' ', ',', ':']).map(|v| v.parse::<i64>()).flatten().collect_vec())
        .collect_vec();
    let mut monkeys = vec![];
    for i in 0..(lines.len() + 1) / 7 {
        monkeys.push((
            parts[i * 7 + 1].clone(),
            parts[i * 7 + 3][0],
            parts[i * 7 + 4][0],
            parts[i * 7 + 5][0],
            0,
        ));
    }
    let md = monkeys.iter().map(|m| m.1).product::<i64>();
    for i in 0..10_000 {
        for mi in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[mi].0);
            for item in items {
                let mut v = item;
                v = op(mi, v) % md;
                let dest = if v % monkeys[mi].1 == 0 { monkeys[mi].2 } else { monkeys[mi].3 };
                monkeys[dest as usize].0.push(v);
                monkeys[mi].4 += 1;
            }
        }
    }
    monkeys.iter().map(|m| m.4).sorted().rev().take(2).product::<i64>()
}

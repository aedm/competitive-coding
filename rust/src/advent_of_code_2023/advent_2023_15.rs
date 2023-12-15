use crate::utils::read_lines_split;
use itertools::Itertools;

pub fn solve_1() -> usize {
    read_lines_split("advent_2023/15.txt", &[','])[0]
        .iter()
        .map(|s| s.bytes().fold(0, |acc, c| ((acc + c as usize) * 17) % 256))
        .sum()
}

pub fn solve_2() -> usize {
    let mut boxes = vec![vec![]; 256];
    for s in &read_lines_split("advent_2023/15.txt", &[','])[0] {
        let parts = s.split(['-', '=']).collect_vec();
        let hash = parts[0].bytes().fold(0, |acc, c| ((acc + c as usize) * 17) % 256);
        if !parts[1].is_empty() {
            let focal_length = parts[1].parse::<usize>().unwrap();
            if let Some(item) = boxes[hash].iter_mut().find(|(n, _)| n == &parts[0]) {
                item.1 = focal_length;
            } else {
                boxes[hash].push((parts[0].to_string(), focal_length));
            }
        } else if let Some((pos, _)) = boxes[hash].iter().find_position(|(n, _)| n == &parts[0]) {
            boxes[hash].remove(pos);
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(bn, b)| {
            b.iter().enumerate().map(|(an, (_, fl))| (bn + 1) * (an + 1) * fl).sum::<usize>()
        })
        .sum()
}

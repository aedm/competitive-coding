use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve_1() -> i64 {
    let n = read_lines("advent_2022/8.txt")
        .iter()
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let (w, h) = (n[0].len(), n.len());
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    (0..h)
        .cartesian_product(0..w)
        .filter(|(y, x)| {
            dirs.iter().any(|(dx, dy)| {
                let (mut x, mut y) = (*x as i64, *y as i64);
                let d = n[y as usize][x as usize];
                let mut visible = true;
                x += dx;
                y += dy;
                while visible && x >= 0 && x < w as i64 && y >= 0 && y < h as i64 {
                    let c = n[y as usize][x as usize];
                    if d <= c {
                        visible = false;
                    }
                    x += dx;
                    y += dy;
                }
                visible
            })
        })
        .count() as i64
}

pub fn solve_2() -> i64 {
    let n = read_lines("advent_2022/8.txt")
        .iter()
        .map(|l| l.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let (w, h) = (n[0].len(), n.len());
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    (1..h - 1)
        .cartesian_product(1..w - 1)
        .map(|(y, x)| {
            let k: i64 = dirs
                .iter()
                .map(|(dx, dy)| {
                    let (mut x, mut y) = (x as i64, y as i64);
                    let d = n[y as usize][x as usize];
                    let mut score = 0;
                    loop {
                        x += dx;
                        y += dy;
                        if !(x >= 0 && x < w as i64 && y >= 0 && y < h as i64) {
                            break;
                        }
                        score += 1;
                        let c = n[y as usize][x as usize];
                        if d <= c {
                            break;
                        }
                    }
                    score
                })
                .product();
            k
        })
        .max()
        .unwrap() as i64
}

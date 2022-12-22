use crate::utils::read_lines;
use itertools::Itertools;
use std::array::from_fn;
use std::cmp::{max, min};
use std::collections::HashSet;

const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn read_input() -> (
    Vec<(u8, [(isize, isize, usize); 4])>,
    Vec<(i64, char)>,
    isize,
    isize,
    isize,
    isize,
) {
    let lines = read_lines("advent_2022/22.txt");
    let h = lines.iter().position(|s| s == "").unwrap();
    let w = lines[0..h].iter().map(|s| s.len()).max().unwrap() + 2;
    let mut m = (0..h as isize + 2)
        .cartesian_product(0..w as isize)
        .map(|(y, x)| (b' ', from_fn(|i| (x + DIRS[i].0, y + DIRS[i].1, i))))
        .collect_vec();

    for (y, s) in lines[0..h].iter().enumerate() {
        for (x, c) in s.bytes().enumerate() {
            m[(y + 1) * w + (x + 1)].0 = c;
        }
    }
    let mut y = 1isize;
    let mut x = lines[0].bytes().position(|c| c == b'.').unwrap() as isize + 1;

    let path = {
        let mut p = 0;
        let mut v = vec![];
        let b = lines[h + 1].chars().collect_vec();
        while let Some(n) = b[p..].iter().position(|c| c.is_ascii_alphabetic()) {
            v.push((
                b[p..p + n].iter().collect::<String>().parse::<i64>().unwrap(),
                b[p + n],
            ));
            p += n + 1;
        }
        v.push((
            b[p..].iter().collect::<String>().parse::<i64>().unwrap(),
            ' ',
        ));
        v
    };

    (m, path, w as isize, h as isize, x, y)
}

fn wrap_around(w: isize, h: isize, m: &mut Vec<(u8, [(isize, isize, usize); 4])>) {
    for y in 1..(h + 1) {
        for x in 1..(w - 1) {
            for dir in 0..4 {
                let (mut nx, mut ny) = (x + DIRS[dir].0, y + DIRS[dir].1);
                if m[(ny * w + nx) as usize].0 == b' ' {
                    while m[((nx - DIRS[dir].0) + w * (ny - DIRS[dir].1)) as usize].0 != b' ' {
                        nx -= DIRS[dir].0;
                        ny -= DIRS[dir].1;
                    }
                }
                m[(y * w + x) as usize].1[dir] = (nx, ny, dir);
            }
        }
    }
}

fn solve(
    w: isize,
    mut x: isize,
    mut y: isize,
    m: Vec<(u8, [(isize, isize, usize); 4])>,
    path: Vec<(i64, char)>,
) -> isize {
    let mut dir = 0;
    for p in path {
        for q in 0..p.0 {
            let (nx, ny, nd) = m[(y * w + x) as usize].1[dir];
            if m[(ny * w + nx) as usize].0 == b'#' {
                break;
            }
            (x, y, dir) = (nx, ny, nd);
        }
        dir = match p.1 {
            'L' => (dir + 3) % 4,
            'R' => (dir + 1) % 4,
            _ => dir,
        };
    }

    y * 1000 + x * 4 + dir as isize
}

fn wrap_cube(w: isize, h: isize, m: &mut Vec<(u8, [(isize, isize, usize); 4])>) {
    let mut f = vec![];
    let concave_corners = (1..h)
        .cartesian_product(1..w - 1)
        .cartesian_product(0..4)
        .filter(|&((y, x), d)| {
            let (ex, ey) = (x + DIRS[(d + 3) % 4].0, y + DIRS[(d + 3) % 4].1);
            let (nx, ny) = (ex + DIRS[(d + 2) % 4].0, ey + DIRS[(d + 2) % 4].1);
            m[(y * w + x) as usize].0 != b' '
                && m[(ey * w + ex) as usize].0 == b' '
                && m[(ny * w + nx) as usize].0 != b' '
        })
        .collect_vec();
    let ((mut y, mut x), mut dir) = concave_corners[0];
    while f.len() == 0 || (x, y, dir) != f[0] {
        f.push((x, y, dir));
        let (mut nx, mut ny) = (x + DIRS[dir].0, y + DIRS[dir].1);
        if m[(ny * w + nx) as usize].0 == b' ' {
            dir = (dir + 1) % 4;
        } else {
            let cdir = (dir + 3) % 4;
            let (cx, cy) = (nx + DIRS[cdir].0, ny + DIRS[cdir].1);
            (x, y, dir) =
                if m[(cy * w + cx) as usize].0 != b' ' { (cx, cy, cdir) } else { (nx, ny, dir) }
        }
    }

    for ((cy, cx), cd) in concave_corners {
        let mut i1 = f.iter().position(|&fp| fp == (cx, cy, cd)).unwrap();
        let mut i2 = (i1 + f.len() - 1) % f.len();
        loop {
            let in1 = (f[i1].2 + 1) % 4;
            let in2 = (f[i2].2 + 1) % 4;
            let out1 = (in1 + 2) % 4;
            let out2 = (in2 + 2) % 4;
            m[(f[i1].1 * w + f[i1].0) as usize].1[out1] = (f[i2].0, f[i2].1, in2);
            m[(f[i2].1 * w + f[i2].0) as usize].1[out2] = (f[i1].0, f[i1].1, in1);

            let o1 = (i1 + 1) % f.len();
            let o2 = (i2 + f.len() - 1) % f.len();
            if (f[i1].0 + DIRS[f[i1].2].0 != f[o1].0 || f[i1].1 + DIRS[f[i1].2].1 != f[o1].1)
                && (f[o2].0 + DIRS[f[o2].2].0 != f[i2].0 || f[o2].1 + DIRS[f[o2].2].1 != f[i2].1)
            {
                break;
            }
            (i1, i2) = (o1, o2);
        }
    }
}

pub fn solve_1() -> isize {
    let (mut m, path, w, h, mut x, mut y) = read_input();
    wrap_around(w, h, &mut m);
    solve(w, x, y, m, path)
}

pub fn solve_2() -> isize {
    let (mut m, path, w, h, mut x, mut y) = read_input();
    wrap_cube(w, h, &mut m);
    solve(w, x, y, m, path)
}

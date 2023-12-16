use crate::utils::array2d::{IVec2D, Map2D, DIRS4};
use crate::utils::read_lines;
use std::collections::VecDeque;

fn calculate_energized(m: &Map2D<char>, start: IVec2D, dir: usize) -> usize {
    let mut mdir = Map2D::from_map(m, |_| [false; 4]);
    let mut queue = VecDeque::from([(start, dir)]);
    while let Some((c, d)) = queue.pop_front() {
        mdir[c][d] = true;
        let dirs = match m[c] {
            '/' => vec![d ^ 3],
            '\\' => vec![d ^ 1],
            v if (v == '-' && d % 2 == 0) || (v == '|' && d % 2 == 1) => {
                vec![(d + 1) % 4, (d + 3) % 4]
            }
            _ => vec![d],
        };
        for nd in dirs {
            if let Some((c2, _)) = m.add_coord(c, DIRS4[nd]) {
                if !mdir[c2][nd] {
                    queue.push_back((c2, nd));
                }
            }
        }
    }
    mdir.filter(|_, v| v[0] || v[1] || v[2] || v[3]).count()
}

pub fn solve_1() -> usize {
    let m = Map2D::from_text(&read_lines("advent_2023/16.txt"));
    calculate_energized(&m, IVec2D::new(0, 0), 3)
}

pub fn solve_2() -> usize {
    let m = Map2D::from_text(&read_lines("advent_2023/16.txt"));
    let vert = (0..m.w).map(|x| [(IVec2D::new(x, 0), 2), (IVec2D::new(x, m.h - 1), 0)]);
    let hori = (0..m.h).map(|y| [(IVec2D::new(0, y), 3), (IVec2D::new(m.w - 1, y), 1)]);
    vert.chain(hori).flatten().map(|(c, d)| calculate_energized(&m, c, d)).max().unwrap()
}

use crate::utils::read_lines_split;
use itertools::Itertools;

fn make_fall(b: &[(i64, i64, i64, i64, i64, i64)], skip: Option<usize>) -> Vec<i64> {
    let mut fall = vec![];
    for i in 0..b.len() {
        let mut support = 0;
        for o in 0..i {
            if skip == Some(o)
                || b[o].1 < b[i].0
                || b[o].0 > b[i].1
                || b[o].3 < b[i].2
                || b[o].2 > b[i].3
            {
                continue;
            }
            support = support.max(fall[o]);
        }
        fall.push(support + b[i].5 - b[i].4 + 1);
    }
    fall
}

pub fn solve() -> (usize, usize) {
    let mut bricks = read_lines_split("advent_2023/22.txt", &[',', '~'])
        .into_iter()
        .map(|l| {
            let (x1, y1, z1, x2, y2, z2) =
                l.iter().map(|s| s.parse::<i64>().unwrap()).collect_tuple().unwrap();
            (
                x1.min(x2),
                x1.max(x2),
                y1.min(y2),
                y1.max(y2),
                z1.min(z2),
                z1.max(z2),
            )
        })
        .sorted_by_key(|(_, _, _, _, z1, _)| *z1)
        .collect_vec();

    let fall = make_fall(&bricks, None);

    let q1 = (0..bricks.len())
        .filter(|&i| {
            let pfall = make_fall(&bricks, Some(i));
            (0..bricks.len()).all(|j| j == i || fall[j] == pfall[j])
        })
        .count();

    let q2 = (0..bricks.len())
        .map(|i| {
            let pfall = make_fall(&bricks, Some(i));
            (0..bricks.len()).filter(|j| *j != i && fall[*j] != pfall[*j]).count()
        })
        .sum::<usize>();

    (q1, q2)
}

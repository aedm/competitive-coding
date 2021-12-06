use crate::utils::read_lines;

pub fn solve(first: bool) -> i64 {
    let lines = read_lines("advent_2021/4.txt");
    let nums: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let boards: Vec<_> = (0..((lines.len() - 1) / 6))
        .map(|i| {
            (0..5)
                .map(|x| {
                    lines[i * 6 + 2 + x]
                        .split(' ')
                        .map(|x| x.parse())
                        .flatten()
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect();

    let mut bcount = boards.len();
    let mut hits = vec![vec![vec![false; 5]; 5]; boards.len()];
    let mut bwon = vec![false; boards.len()];

    for num in nums {
        for (i, b) in boards.iter().enumerate() {
            if bwon[i] {
                continue;
            }
            for y in 0..5 {
                for x in 0..5 {
                    if b[y][x] == num {
                        hits[i][y][x] = true;
                        if (0..5).all(|q| hits[i][q][x]) || (0..5).all(|q| hits[i][y][q]) {
                            bwon[i] = true;
                            bcount -= 1;
                            if first || bcount == 0 {
                                let s: i64 = b
                                    .iter()
                                    .enumerate()
                                    .map(|(qy, row)| {
                                        row.iter()
                                            .enumerate()
                                            .filter(|&(qx, n)| !hits[i][qy][qx])
                                            .map(|(qx, n)| n)
                                            .sum::<i64>()
                                    })
                                    .sum();
                                return s * num;
                            }
                        }
                    }
                }
            }
        }
    }
    panic!();
}

pub fn solve_1() -> i64 {
    return solve(true);
}

pub fn solve_2() -> i64 {
    return solve(false);
}

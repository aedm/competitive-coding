use crate::utils::read_lines;
use itertools::Itertools;

pub fn solve(first: bool) -> i64 {
    let lines = read_lines("advent_2021/4.txt");
    let nums: Vec<i64> = lines[0].split(',').map(|x| x.parse().unwrap()).collect();
    let boards: Vec<_> = (0..((lines.len() - 1) / 6))
        .map(|i| {
            (0..5)
                .map(|x| {
                    lines[i * 6 + 2 + x]
                        .split(' ')
                        .filter_map(|x| x.parse().ok())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect();

    let mut boards_left = boards.len();
    let mut hits = vec![[[false; 5]; 5]; boards.len()];
    let mut is_board_finished = vec![false; boards.len()];

    for num in nums {
        for (i, board) in boards.iter().enumerate() {
            if !is_board_finished[i] {
                if let Some((x, y)) =
                    ((0..5).cartesian_product(0..5)).find(|&(x, y)| board[y][x] == num)
                {
                    hits[i][y][x] = true;
                    if (0..5).all(|q| hits[i][q][x]) || (0..5).all(|q| hits[i][y][q]) {
                        is_board_finished[i] = true;
                        boards_left -= 1;
                        if first || boards_left == 0 {
                            let score = ((0..5).cartesian_product(0..5))
                                .filter_map(|(x, y)| (!hits[i][y][x]).then(|| board[y][x]))
                                .sum::<i64>();
                            return score * num;
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

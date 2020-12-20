use crate::utils::read_lines;

pub fn solve_1() -> usize {
    read_lines("advent_2020/3.txt")
        .iter()
        .enumerate()
        .filter(|(i, line)| line.chars().nth(i * 3 % line.len()).unwrap() == '#')
        .count()
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2020/3.txt");
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)|
            (0..lines.len() / dy)
                .filter(|i| lines[i * dy].chars().nth(i * dx % lines[i * dy].len()).unwrap() == '#')
                .count()
        )
        .product()
}

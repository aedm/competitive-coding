use crate::utils::read_lines;
use std::error;
use sudoku::Sudoku;

pub fn solve() -> Result<i32, Box<dyn error::Error>> {
    let lines = read_lines("euler/p096_sudoku.txt");
    let mut sum = 0;
    for i in 0..(lines.len() / 10) {
        let k = lines[i * 10 + 1..i * 10 + 10].join("").replace('0', ".");
        let sudoku = Sudoku::from_str_line(&k).unwrap();
        let solution = sudoku.solve_unique().unwrap().to_string();
        sum += &solution[0..3].parse::<i32>().unwrap();
    }
    Ok(sum)
}

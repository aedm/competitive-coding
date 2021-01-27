use crate::utils::read_lines;
use std::fmt::Debug;

pub fn solve_1() {
    let max = 950003;
    let mut x = 0;
    while x < max - 15 {
        println!(
            "{}\n{}\nFizz\n{}\nBuzz\nFizz\n{}\n{}\nFizz\nBuzz\n{}\nFizz\n{}\n{}\nFizzBuzz",
            x + 1,
            x + 2,
            x + 4,
            x + 7,
            x + 8,
            x + 11,
            x + 13,
            x + 14
        );
        x += 15;
    }

    for i in x + 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}

pub fn solve() {
    for i in 1..=20 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{:?}", i),
        }
    }
}

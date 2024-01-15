use crate::utils::read_lines;
use itertools::Itertools;
use mathru::algebra::linear::matrix::{General, LUDecomposition};
use mathru::algebra::linear::vector::Vector;
use mathru::{
    algebra::linear::matrix::{LUDec, Solve},
    matrix, vector,
};
use regex::Regex;
use std::fmt::Debug;

fn calc_t(p1x: f64, p1y: f64, p2x: f64, p2y: f64, d1x: f64, d1y: f64, d2x: f64, d2y: f64) -> f64 {
    -((p2y - p1y) * d2x - (p2x - p1x) * d2y) / (d1x * d2y - d1y * d2x)
}

fn read_input() -> Vec<Vec<f64>> {
    let pattern: Regex =
        Regex::new(r"^\s*(-?\d+),\s*(-?\d+),\s*(-?\d+) @\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)$")
            .unwrap();
    read_lines("advent_2023/24.txt")
        .into_iter()
        .map(|l| {
            let caps = pattern.captures(&l).unwrap();
            (1..=6).map(|i| caps[i].parse::<f64>().unwrap()).collect_vec()
        })
        .collect_vec()
}

pub fn solve_1() -> impl Debug {
    let s = read_input();

    let wmin = 200_000_000_000_000f64;
    let wmax = 400_000_000_000_000f64;

    let mut n = 0;
    for i in 1..s.len() {
        for o in 0..i {
            let (a, b) = (&s[i], &s[o]);
            if a[3] * b[4] == a[4] * b[3] {
                continue;
            }
            let t1 = calc_t(a[0], a[1], b[0], b[1], a[3], a[4], b[3], b[4]);
            let t2 = calc_t(b[0], b[1], a[0], a[1], b[3], b[4], a[3], a[4]);
            let (cx, cy) = (a[0] + a[3] * t1, a[1] + a[4] * t1);
            if t1 >= 0.0 && t2 >= 0.0 && cx >= wmin && cx <= wmax && cy >= wmin && cy <= wmax {
                n += 1;
            }
        }
    }
    n
}

pub fn solve_2() -> impl Debug {
    let s = read_input();

    // (p-p[n]) × (v-v[n]) = 0
    // For p×v to be eliminated, subtract equations you get from p[n] and p[m].
    // Three hailstones lead to a simple linear equation system with 6 variables and 6 equations.
    let (p1x, p1y, p1z) = (s[1][0] - s[0][0], s[1][1] - s[0][1], s[1][2] - s[0][2]);
    let (v1x, v1y, v1z) = (s[1][3] - s[0][3], s[1][4] - s[0][4], s[1][5] - s[0][5]);
    let (p2x, p2y, p2z) = (s[2][0] - s[0][0], s[2][1] - s[0][1], s[2][2] - s[0][2]);
    let (v2x, v2y, v2z) = (s[2][3] - s[0][3], s[2][4] - s[0][4], s[2][5] - s[0][5]);

    let a: General<f64> = matrix![
        v1y, -v1x, 0.0, -p1y, p1x, 0.0;
        v1z, 0.0, -v1x, -p1z, 0.0, p1x;
        0.0, v1z, -v1y, 0.0, -p1z, p1y;
        v2y, -v2x, 0.0, -p2y, p2x, 0.0;
        v2z, 0.0, -v2x, -p2z, 0.0, p2x;
        0.0, v2z, -v2y, 0.0, -p2z, p2y];

    let b: Vector<f64> = vector![
         s[0][1] * s[0][3] - s[0][0] * s[0][4] + s[1][0] * s[1][4] - s[1][1] * s[1][3];
            s[0][2] * s[0][3] - s[0][0] * s[0][5] + s[1][0] * s[1][5] - s[1][2] * s[1][3];
            s[0][2] * s[0][4] - s[0][1] * s[0][5] + s[1][1] * s[1][5] - s[1][2] * s[1][4];
            s[0][1] * s[0][3] - s[0][0] * s[0][4] + s[2][0] * s[2][4] - s[2][1] * s[2][3];
            s[0][2] * s[0][3] - s[0][0] * s[0][5] + s[2][0] * s[2][5] - s[2][2] * s[2][3];
            s[0][2] * s[0][4] - s[0][1] * s[0][5] + s[2][1] * s[2][5] - s[2][2] * s[2][4]];

    // Solve the linear equation system.
    let r: Vector<f64> = a.solve(&b).unwrap();
    (r[0].round() + r[1].round() + r[2].round()) as i64
}

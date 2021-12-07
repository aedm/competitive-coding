use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
struct Input {
    room: Vec<Vec<i64>>,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let size = input.room.len() as i64;

    let mut threats = vec![vec![0i64; size as usize]; size as usize];
    let mut fields = Vec::new();

    let (kx, ky) = (0..size)
        .find_map(|x| {
            (0..size).find(|y| input.room[*y as usize][x as usize] == 6).and_then(|y| Some((x, y)))
        })
        .unwrap();

    let mut step = |mut x: i64, mut y: i64, dx: i64, dy: i64, once: bool, king: bool| loop {
        x += dx;
        y += dy;
        if x < 0 || x >= size || y < 0 || y >= size {
            return;
        }
        if king && (x - kx).abs() <= 1 && (y - ky).abs() <= 1 {
            return;
        }
        let f = input.room[y as usize][x as usize];
        if f != 0 {
            return;
        }
        threats[y as usize][x as usize] += 1;
        if once {
            return;
        }
    };

    let all_dir = [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
    ];

    for y in 0..(size) {
        for x in 0i64..(size) {
            let f = input.room[y as usize][x as usize];
            if f == 0 {
                fields.push((x, y));
            }
            match f {
                1 => step(x, y, 0, 1, true, false),
                2 => [(0, 1), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .for_each(|(dx, dy)| step(x, y, *dx, *dy, false, false)),
                3 => [
                    (2, 1),
                    (1, 2),
                    (-2, 1),
                    (-1, 2),
                    (2, -1),
                    (1, -2),
                    (-2, -1),
                    (-1, -2),
                ]
                .iter()
                .for_each(|(dx, dy)| step(x, y, *dx, *dy, true, false)),
                4 => [(1, 1), (1, -1), (-1, 1), (-1, -1)]
                    .iter()
                    .for_each(|(dx, dy)| step(x, y, *dx, *dy, false, false)),
                5 => all_dir.iter().for_each(|(dx, dy)| step(x, y, *dx, *dy, false, false)),
                7 => all_dir.iter().for_each(|(dx, dy)| step(x, y, *dx, *dy, true, true)),
                _ => {}
            }
        }
    }

    fields.sort_by(|a, b| {
        threats[a.1 as usize][a.0 as usize]
            .cmp(&threats[b.1 as usize][b.0 as usize])
            .then(a.0.cmp(&b.0))
            .then(a.1.cmp(&b.1))
    });
    println!("{:?}", fields);

    let solution: Vec<Vec<i64>> = fields.iter().map(|(x, y)| vec![*x, *y]).collect();
    json!({ "places_to_move_to": Value::from(solution) })
}

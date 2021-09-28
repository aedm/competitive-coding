use serde_json::{json, Map, Value};
use std::collections::VecDeque;

pub fn solve(v: &Value) -> Value {
    let width = v["width"].as_i64().unwrap();
    let height = v["height"].as_i64().unwrap();
    let size = (width * height) as usize;

    let maze: Vec<_> = v["maze"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap())
        .collect();
    let start = (
        v["startCell"]["x"].as_i64().unwrap(),
        v["startCell"]["y"].as_i64().unwrap(),
    );
    let mut end = (
        v["endCell"]["x"].as_i64().unwrap(),
        v["endCell"]["y"].as_i64().unwrap(),
    );

    let dirs = vec![
        ('R', (1i64, 0i64)),
        ('L', (-1, 0)),
        ('U', (0, -1)),
        ('D', (0, 1)),
    ];

    let mut visited = vec![false; size];
    let mut from = vec![-1i64; size];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    while let Some(next) = queue.pop_front() {
        for i in 0..4 {
            let d = &dirs[i];
            let x = next.0 + d.1 .0;
            let y = next.1 + d.1 .1;
            let coord = (y * width + x) as usize;
            if x < 0 || y < 0 || x >= width || y >= height || maze[coord] == 0 || visited[coord] {
                continue;
            }
            visited[coord] = true;
            from[coord] = i as i64;
            queue.push_back((x, y));
        }
    }
    let mut sol = VecDeque::new();
    loop {
        let d = &dirs[from[(end.1 * width + end.0) as usize] as usize];
        sol.push_front(d.0);
        end.0 -= d.1 .0;
        end.1 -= d.1 .1;
        if end == start {
            break;
        }
    }
    let solution: String = sol.into_iter().collect();

    json!({ "solution": Value::from(solution) })
}

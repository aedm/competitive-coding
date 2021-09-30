use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::collections::VecDeque;

#[derive(Deserialize, PartialEq, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Deserialize)]
struct Input {
    width: i64,
    height: i64,
    maze: Vec<u64>,
    #[serde(alias = "startCell")]
    start_cell: Coord,
    #[serde(alias = "endCell")]
    end_cell: Coord,
}

pub fn solve(v: &Value) -> Value {
    let input: Input = serde_json::from_value(v.clone()).unwrap();
    let size = (input.width * input.height) as usize;

    let dirs = vec![
        ('R', (1i64, 0i64)),
        ('L', (-1, 0)),
        ('U', (0, -1)),
        ('D', (0, 1)),
    ];

    let mut visited = vec![false; size];
    let mut from = vec![-1i64; size];
    let mut queue = VecDeque::new();

    queue.push_back(input.start_cell.clone());
    while let Some(next) = queue.pop_front() {
        for i in 0..4 {
            let d = &dirs[i];
            let x = next.x + d.1 .0;
            let y = next.y + d.1 .1;
            let c = (y * input.width + x) as usize;
            if x < 0
                || y < 0
                || x >= input.width
                || y >= input.height
                || input.maze[c] == 0
                || visited[c]
            {
                continue;
            }
            visited[c] = true;
            from[c] = i as i64;
            queue.push_back(Coord { x, y });
        }
    }
    let mut sol = VecDeque::new();
    let mut c = input.end_cell;
    loop {
        let d = &dirs[from[(c.y * input.width + c.x) as usize] as usize];
        sol.push_front(d.0);
        c.x -= d.1 .0;
        c.y -= d.1 .1;
        if c == input.start_cell {
            break;
        }
    }
    let solution: String = sol.into_iter().collect();

    json!({ "solution": Value::from(solution) })
}

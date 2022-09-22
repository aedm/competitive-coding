use crate::utils::neighbours4;
use itertools::Itertools;
use serde_json::{json, Map, Value};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn find_path(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let (w, h) = (maze[0].len(), maze.len());
    let mut queue = VecDeque::new();
    let mut from = HashMap::new();
    queue.push_back(start);

    'lp: while let Some(p) = queue.pop_front() {
        for (x, y) in neighbours4(p, 0, 0, w, h) {
            if from.contains_key(&(x, y)) {
                continue;
            }
            if maze[y][x] == '.' || end == (x, y) {
                from.insert((x, y), p);
                queue.push_back((x, y));
                if end == (x, y) {
                    break 'lp;
                }
            }
        }
    }

    let mut path = Vec::new();
    let mut p = end;
    while p != start {
        path.push(p);
        p = from[&p];
    }
    path
}

pub fn solve(v: &Value) -> Value {
    let maze_str: Vec<Vec<String>> = serde_json::from_value(v.clone()).unwrap();
    let maze: Vec<Vec<char>> =
        maze_str.iter().map(|x| x.iter().map(|x| x.chars().next().unwrap()).collect()).collect();

    let (w, h) = (maze[0].len(), maze.len());
    let start = ((0..w).cartesian_product(0..h)).find(|(x, y)| maze[*y][*x] == 'S').unwrap();
    let key = ((0..w).cartesian_product(0..h)).find(|(x, y)| maze[*y][*x] == 'K').unwrap();
    let end = ((0..w).cartesian_product(0..h)).find(|(x, y)| maze[*y][*x] == 'E').unwrap();

    let mut path = vec![];
    path.append(find_path(&maze, key, end).as_mut());
    path.append(find_path(&maze, start, key).as_mut());
    path.push(start);
    path.reverse();

    let res: Vec<_> = path.iter().map(|(x, y)| json!({"x": *x, "y": *y})).collect();

    json!(res)
}

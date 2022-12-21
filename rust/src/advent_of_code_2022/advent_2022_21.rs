use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::HashMap;

fn traverse(node: &str, ops: &mut HashMap<String, (u8, Option<i64>, String, String)>) -> i64 {
    let (op, val, left, right) = ops.get(node).unwrap().clone();
    if let Some(n) = val {
        return n;
    }
    let l = traverse(&left, ops);
    let r = traverse(&right, ops);
    let res = match op {
        b'+' => l + r,
        b'-' => l - r,
        b'*' => l * r,
        b'/' => l / r,
        _ => unreachable!(),
    };
    ops.insert(node.to_string(), (op, Some(res), left, right));
    res
}

fn push(node: &str, res: i64, ops: &mut HashMap<String, (u8, Option<i64>, String, String)>) -> i64 {
    println!("{} = {}", node, res);
    if node == "humn" {
        return res;
    }
    let (op, val, left, right) = ops.get(node).unwrap().clone();
    println!("{} = {} {} {} {}", node, left, op as char, right, res);
    let (_, lval, _, _) = ops.get(&left).unwrap().clone();
    let (_, rval, _, _) = ops.get(&right).unwrap().clone();
    println!("vals {rval:?} {lval:?}");
    if lval.is_some() && rval.is_some() {
        panic!();
    }
    if lval.is_none() {
        println!(" go left");
        let r = rval.unwrap();
        let l = match op {
            b'+' => res - r,
            b'-' => res + r,
            b'*' => res / r,
            b'/' => res * r,
            _ => unreachable!(),
        };
        push(&left, l, ops)
    } else {
        println!(" go right");
        let l = lval.unwrap();
        let r = match op {
            b'+' => res - l,
            b'-' => l - res,
            b'*' => res / l,
            b'/' => l / res,
            _ => unreachable!(),
        };
        push(&right, r, ops)
    }
}

fn traverse_2(
    node: &str,
    ops: &mut HashMap<String, (u8, Option<i64>, String, String)>,
) -> Option<i64> {
    if node == "humn" {
        return None;
    }
    let (op, val, left, right) = ops.get(node).unwrap().clone();
    if let Some(n) = val {
        return val;
    }
    let l = traverse_2(&left, ops);
    let r = traverse_2(&right, ops);
    if op == b'=' {
        if l.is_none() && r.is_none() {
            panic!();
        }
        return if l.is_none() {
            Some(push(&left, r.unwrap(), ops))
        } else {
            Some(push(&right, l.unwrap(), ops))
        };
    }
    if l.is_none() || r.is_none() {
        return None;
    }
    let l = l.unwrap();
    let r = r.unwrap();
    let res = match op {
        b'+' => l + r,
        b'-' => l - r,
        b'*' => l * r,
        b'/' => l / r,
        _ => unreachable!(),
    };
    ops.insert(node.to_string(), (op, Some(res), left, right));
    Some(res)
}

pub fn solve_1() -> i64 {
    let mut ops = HashMap::new();
    let mut deps = HashMap::new();
    for s in read_lines("advent_2022/21.txt") {
        let a = s.split(' ').collect_vec();
        let name = a[0][0..4].to_string();
        let op = if let Ok(n) = a[1].parse::<i64>() {
            (0u8, Some(n), "".to_string(), "".to_string())
        } else {
            let op = (a[2].as_bytes()[0], None, a[1].to_string(), a[3].to_string());
            deps.entry(op.2.clone()).or_insert_with(Vec::new).push(name.clone());
            deps.entry(op.3.clone()).or_insert_with(Vec::new).push(name.clone());
            op
        };
        ops.insert(name.clone(), op);
    }
    traverse("root", &mut ops)
}

pub fn solve_2() -> i64 {
    let mut ops = HashMap::new();
    let mut deps = HashMap::new();
    for s in read_lines("advent_2022/21.txt") {
        let a = s.split(' ').collect_vec();
        let name = a[0][0..4].to_string();
        let op = if let Ok(n) = a[1].parse::<i64>() {
            (0u8, Some(n), "".to_string(), "".to_string())
        } else {
            let op = (a[2].as_bytes()[0], None, a[1].to_string(), a[3].to_string());
            deps.entry(op.2.clone()).or_insert_with(Vec::new).push(name.clone());
            deps.entry(op.3.clone()).or_insert_with(Vec::new).push(name.clone());
            op
        };
        ops.insert(name.clone(), op);
    }
    ops.get_mut("root").unwrap().0 = b'=';
    ops.get_mut("humn").unwrap().1 = None;
    traverse_2("root", &mut ops).unwrap()
}

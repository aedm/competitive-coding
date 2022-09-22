use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use regex::Regex;
use std::any::Any;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Number(i64),
    Open,
    Close,
}

fn parse_node(s: &str) -> Vec<Node> {
    let pattern: Regex = Regex::new(r"^(\[*)(\d+)(\]*)$").unwrap();
    let mut v = vec![];
    for sm in s.split(',') {
        let caps = pattern.captures(sm).unwrap();
        let c = caps.iter().skip(1).map(|c| c.unwrap().as_str()).collect_vec();
        (0..c[0].len()).for_each(|_| v.push(Node::Open));
        v.push(Node::Number(c[1].parse::<i64>().unwrap()));
        (0..c[2].len()).for_each(|_| v.push(Node::Close));
    }
    v
}

fn explode_node(v: &mut Vec<Node>) -> bool {
    let mut level = 0;
    for i in 0..v.len() - 1 {
        match v[i] {
            Node::Number(v1) => {
                if level > 4 {
                    if let Node::Number(v2) = v[i + 1] {
                        for x in 1..i {
                            if let Node::Number(n) = v[i - x] {
                                v[i - x] = Node::Number(n + v1);
                                break;
                            }
                        }
                        for x in i + 2..v.len() {
                            if let Node::Number(n) = v[x] {
                                v[x] = Node::Number(n + v2);
                                break;
                            }
                        }
                        v[i] = Node::Number(0);
                        v.remove(i + 1);
                        v.remove(i + 1);
                        v.remove(i - 1);
                        return true;
                    }
                }
            }
            Node::Open => level += 1,
            Node::Close => level -= 1,
        }
    }
    false
}

fn split_node(v: &mut Vec<Node>) -> bool {
    let mut level = 0;
    for i in 0..v.len() - 1 {
        match v[i] {
            Node::Number(n) => {
                if n >= 10 {
                    v.insert(i + 1, Node::Close);
                    v.insert(i + 1, Node::Number((n + 1) / 2));
                    v[i] = Node::Number(n / 2);
                    v.insert(i, Node::Open);
                    return true;
                }
            }
            Node::Open => level += 1,
            Node::Close => level -= 1,
        }
    }
    false
}

fn magnitude<'a>(it: &mut impl Iterator<Item = &'a Node>) -> i64 {
    match it.next().unwrap() {
        Node::Number(c) => *c,
        Node::Open => {
            let left = magnitude(it);
            let right = magnitude(it);
            assert_eq!(*it.next().unwrap(), Node::Close);
            left * 3 + right * 2
        }
        _ => panic!(),
    }
}

pub fn solve_1() -> i64 {
    let mut lines = read_lines("advent_2021/18.txt");
    let mut n = lines.iter().map(|s| parse_node(s)).collect_vec();
    let mut v = n[0].clone();
    for b in &mut n[1..] {
        v.append(b);
        v.insert(0, Node::Open);
        v.push(Node::Close);
        while explode_node(&mut v) || split_node(&mut v) {}
    }
    magnitude(&mut v.iter())
}

pub fn solve_2() -> i64 {
    let mut lines = read_lines("advent_2021/18.txt");
    let mut n = lines.iter().map(|s| parse_node(s)).collect_vec();
    let mut maximum = 0;
    for a in 0..n.len() {
        for b in 0..n.len() {
            if a != b {
                let mut v = n[a].clone();
                v.append(&mut n[b].clone());
                v.insert(0, Node::Open);
                v.push(Node::Close);
                while explode_node(&mut v) || split_node(&mut v) {}
                maximum = max(maximum, magnitude(&mut v.iter()))
            }
        }
    }
    maximum
}

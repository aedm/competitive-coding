use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use std::any::Any;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

struct Packet {
    version: i64,
    type_id: i64,
    content: Content,
}

enum Content {
    Literal(i64),
    Op(Vec<Packet>),
}

fn read(d: &[u8], pos: &mut usize, len: usize) -> i64 {
    *pos += len;
    d[*pos - len..*pos].iter().fold(0, |acc, n| acc * 2 + *n as i64)
}

fn decode_packet(d: &[u8], pos: &mut usize) -> Packet {
    let version = read(d, pos, 3);
    let type_id = read(d, pos, 3);
    let content = match type_id {
        4 => {
            let mut value = 0;
            loop {
                let first = read(d, pos, 1);
                value = value * 16 + read(d, pos, 4);
                if first == 0 {
                    break;
                }
            }
            Content::Literal(value)
        }
        _ => {
            let length_type_id = read(d, pos, 1);
            let mut packets = vec![];
            if length_type_id == 0 {
                let bit_count = read(d, pos, 15);
                let end_pos = *pos + bit_count as usize;
                while *pos < end_pos {
                    packets.push(decode_packet(d, pos));
                }
            } else {
                let packet_count = read(d, pos, 11);
                for _ in 0..packet_count {
                    packets.push(decode_packet(d, pos));
                }
            }
            Content::Op(packets)
        }
    };
    Packet {
        version,
        type_id,
        content,
    }
}

fn version_sum(packet: &Packet) -> i64 {
    match &packet.content {
        Content::Literal(_) => packet.version,
        Content::Op(op) => packet.version + op.iter().map(|p| version_sum(p)).sum::<i64>(),
    }
}

pub fn solve_1() -> i64 {
    let mut lines = read_lines("advent_2021/16.txt");
    let mut d = lines[0]
        .chars()
        .flat_map(|c| {
            format!("{:04b}", i64::from_str_radix(&c.to_string(), 16).unwrap())
                .bytes()
                .map(|b| b - b'0')
                .collect_vec()
        })
        .collect_vec();

    let mut p = 0usize;
    let root = decode_packet(&d, &mut p);
    version_sum(&root)
}

fn eval(packet: &Packet) -> i64 {
    match &packet.content {
        Content::Literal(val) => *val,
        Content::Op(op) => match packet.type_id {
            0 => op.iter().map(|p| eval(p)).sum(),
            1 => op.iter().map(|p| eval(p)).product(),
            2 => op.iter().map(|p| eval(p)).min().unwrap(),
            3 => op.iter().map(|p| eval(p)).max().unwrap(),
            5 => (eval(&op[0]) > eval(&op[1])) as i64,
            6 => (eval(&op[0]) < eval(&op[1])) as i64,
            7 => (eval(&op[0]) == eval(&op[1])) as i64,
            _ => panic!(),
        },
    }
}

pub fn solve_2() -> i64 {
    let mut lines = read_lines("advent_2021/16.txt");
    let mut d = lines[0]
        .chars()
        .flat_map(|c| {
            format!("{:04b}", i64::from_str_radix(&c.to_string(), 16).unwrap())
                .bytes()
                .map(|b| b - b'0')
                .collect_vec()
        })
        .collect_vec();

    let mut p = 0usize;
    let root = decode_packet(&d, &mut p);
    eval(&root)
}

use crate::utils::{neighbours4, read_lines};
use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// first three bits encode the packet version, and the
// next three bits encode the packet type ID.

// Packets with type ID 4 represent a literal value.
#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    content: Content,
}

#[derive(Debug)]
enum Content {
    Literal(i64),
    Op(Vec<Packet>),
}

fn tv(d: &[u8], pos: &mut usize, len: usize) -> i64 {
    *pos += len;
    let val = d[*pos - len..*pos].iter().fold(0, |acc, n| acc * 2 + *n as i64);
    println!("{:2} bits at {:3}: {}", len, *pos - len, val);
    val
}

fn decode_packet(d: &[u8], pos: &mut usize) -> Packet {
    println!("-- start");
    let start_pos = *pos;
    let version = tv(d, pos, 3);
    let type_id = tv(d, pos, 3);
    let content = match type_id {
        4 => {
            let mut value = 0;
            loop {
                let s = tv(d, pos, 1);
                value = value * 16 + tv(d, pos, 4);
                if s == 0 {
                    break;
                }
            }
            // *pos += (4 - (*pos - start_pos) % 4) % 4;
            println!("-- literal: {}", value);
            Content::Literal(value)
        }
        _ => {
            let length_type_id = tv(d, pos, 1);
            let mut packets = vec![];
            if length_type_id == 0 {
                let bit_count = tv(d, pos, 15);
                let end_pos = *pos + bit_count as usize;
                while *pos < end_pos {
                    println!("---- pos: {} end: {}", *pos, end_pos);
                    packets.push(decode_packet(d, pos));
                }
            } else {
                let packet_count = tv(d, pos, 11);
                for _ in 0..packet_count {
                    packets.push(decode_packet(d, pos));
                }
            }
            println!("-- ops: {}", packets.len());
            Content::Op(packets)
        }
    };
    println!("-- end");

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
    println!("{} {:?}", d.len(), d);

    let root = decode_packet(&d, &mut p);
    println!("{:?}", root);

    version_sum(&root)
}

pub fn solve() -> i64 {
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
    println!("{} {:?}", d.len(), d);

    let root = decode_packet(&d, &mut p);
    println!("{:?}", root);

    version_sum(&root)
}

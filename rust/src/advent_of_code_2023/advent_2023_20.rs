use crate::utils::read_lines;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
enum Module {
    FlipFlop(bool),
    Conjunction(usize, usize),
    Broadcast,
    Sink,
}

pub fn solve() -> impl Debug {
    let mut labels = HashSet::new();
    let mut k = read_lines("advent_2023/20.txt")
        .into_iter()
        .map(|line| {
            let p = line.split(" -> ").collect_vec();
            let targets = p[1].split(", ").map(|s| s.to_string()).collect_vec();
            for t in &targets {
                labels.insert(t.clone());
            }
            let (label, module) = match p[0].chars().nth(0).unwrap() {
                '%' => (&p[0][1..], Module::FlipFlop(false)),
                '&' => (&p[0][1..], Module::Conjunction(0, 0)),
                _ => (p[0], Module::Broadcast),
            };
            labels.insert(label.to_string());
            (label.to_string(), (module, targets))
        })
        .collect::<HashMap<_, _>>();

    let mods = labels.into_iter().enumerate().map(|(i, v)| (v, i)).collect::<HashMap<_, _>>();

    let mut state: Vec<(Module, Vec<usize>)> = vec![(Module::Sink, vec![]); mods.len()];
    for (v, i) in &mods {
        if let Some((module, targets)) = k.get(v) {
            state[*i] = (*module, targets.iter().map(|t| mods[t]).collect_vec());
        }
    }

    let rx_input = k.iter().find(|(_, (_, t))| t.contains(&"rx".to_string())).unwrap().0;
    let mut vs = vec![];
    for i in 0..state.len() {
        for t in state[i].1.clone() {
            if let Module::Conjunction(_, mask) = &mut state[t].0 {
                *mask |= 1 << i;
            }
            if t == mods[rx_input] {
                vs.push(i);
            }
        }
    }

    let mut outputs = [0, 0];
    let mut cycles = vec![None; state.len()];

    'l: for n in 1usize.. {
        let mut queue = VecDeque::from([(mods["broadcaster"], false, 0)]);
        while let Some((i, input, source)) = queue.pop_front() {
            if n > 0 && vs.contains(&source) && input && cycles[source].is_none() {
                cycles[source] = Some(n);
            }
            if n <= 1000 {
                outputs[input as usize] += 1;
            } else if vs.iter().all(|vi| cycles[*vi].is_some()) {
                break 'l;
            }
            let (module, targets) = &mut state[i];
            let output = match (module, input) {
                (Module::FlipFlop(s), false) => {
                    *s = !*s;
                    *s
                }
                (Module::Conjunction(value, mask), true) => {
                    *value |= 1 << source;
                    *value != *mask
                }
                (Module::Conjunction(value, mask), false) => {
                    *value &= !(1 << source);
                    *value != *mask
                }
                (Module::Broadcast, _) => input,
                _ => continue,
            };
            for t in targets {
                queue.push_back((*t, output, i));
            }
        }
    }
    let q1 = outputs[0] * outputs[1];
    let q2: usize = vs.iter().map(|i| cycles[*i].unwrap()).product();
    (q1, q2)
}

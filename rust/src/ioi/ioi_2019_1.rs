use std::collections::{BTreeSet, VecDeque};

// const K: usize = 2;
// const INPUT: &'static [i64] = &[-2, -1, 1, 2, 1, -1];
// const INPUT: &'static [i64] = &[-2, 2, 2, -2, -2, 2];
// const INPUT: &'static [i64] = &[2, 1, -1, -2];

pub fn solve(input_lines: &Vec<String>) -> Vec<String> {
    // println!("{:?}", input_lines);
    let n: usize = input_lines[0].parse().unwrap();
    let input: Vec<i64> = input_lines[1].split(" ").map(|x| x.parse().unwrap()).collect();
    assert_eq!(input.len(), n * 2);

    let mut lists: Vec<VecDeque<usize>> = vec![VecDeque::new(); n * 2 + 3];
    input
        .iter()
        .enumerate()
        .for_each(|(index, x)| lists[(n as i64 + *x) as usize].push_back(index));
    // println!("{:?}", &lists);
    let mut missing: BTreeSet<usize> = BTreeSet::new();
    let mut empties = vec![false; input.len()];
    let mut swaps = 0_usize;
    for (i, v) in input.iter().enumerate() {
        if empties[i] {
            continue;
        }
        let c1 = (n as i64 + *v) as usize;
        let c2 = (n as i64 - *v) as usize;
        // assert_eq!(*lists[c1].front().unwrap(), i);
        lists[c1].pop_front();
        let i2 = lists[c2].pop_front().unwrap();
        let skips = missing.range(i..i2).count();
        let current_swaps = i2 - i - skips - if *v < 0 { 1 } else { 0 };
        //println!("pair: {} {} skips {} swaps {}", i, i2, skips, current_swaps);
        //println!("missing {:?}", missing);
        swaps += current_swaps;
        missing.insert(i2);
        empties[i2] = true;
    }
    vec!["OK".to_string(), format!("{}", swaps)]
}

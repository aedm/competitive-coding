use crate::utils::read_lines;
use regex::Regex;
use std::collections::HashMap;

pub fn solve_1() -> usize {
    let lines = read_lines("advent_2020/7.txt");
    let deps = parse_input(&lines);
    let mut contains_gold: HashMap<&str, bool> = HashMap::new();
    for dep in &deps {
        find_gold(dep.0, &mut contains_gold, &deps);
    }
    contains_gold.values().filter(|x| **x).count()
}

pub fn solve_2() -> usize {
    let lines = read_lines("advent_2020/7.txt");
    let deps = parse_input(&lines);
    let mut bag_count: HashMap<&str, usize> = HashMap::new();
    count_bags("shiny gold", &mut bag_count, &deps) - 1
}

fn count_bags<'a>(
    bag: &'a str,
    bag_count: &mut HashMap<&'a str, usize>,
    deps: &HashMap<&str, Vec<(usize, &'a str)>>,
) -> usize {
    if let Some(x) = bag_count.get(bag) {
        return *x;
    }
    let inner_count = deps
        .get(bag)
        .unwrap()
        .iter()
        .map(|d| d.0 * count_bags(d.1, bag_count, deps))
        .sum::<usize>();
    bag_count.insert(bag, inner_count + 1);
    inner_count + 1
}

fn find_gold<'a>(
    bag: &'a str,
    has_gold: &mut HashMap<&'a str, bool>,
    deps: &HashMap<&str, Vec<(usize, &'a str)>>,
) -> bool {
    if bag == "shiny gold" {
        return true;
    }
    if let Some(x) = has_gold.get(bag) {
        return *x;
    }
    let has = deps.get(bag).unwrap().iter().any(|d| find_gold(d.1, has_gold, deps));
    has_gold.insert(bag, has);
    has
}

fn parse_input(lines: &Vec<String>) -> HashMap<&str, Vec<(usize, &str)>> {
    let pattern: Regex = Regex::new(r"^(\d+) (.+) bag[s]?$").unwrap();
    let mut deps: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for line in lines {
        let parts: Vec<_> = line.split(" bags contain ").collect();
        if parts[1] == "no other bags." {
            deps.insert(parts[0], Vec::new());
        } else {
            let dep_parts: Vec<_> = parts[1]
                .trim_end_matches(".")
                .split(", ")
                .map(|x| {
                    let captures = pattern.captures(x).unwrap();
                    (
                        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        captures.get(2).unwrap().as_str(),
                    )
                })
                .collect();
            deps.insert(parts[0], dep_parts);
        }
    }
    deps
}

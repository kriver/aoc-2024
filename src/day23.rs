use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::util::load;

type Input = HashMap<String, Vec<String>>;

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day23.txt");
    let mut con = HashMap::new();
    for line in lines {
        let mut token_it = line.split("-");
        let a = token_it.next().unwrap().to_string();
        let b = token_it.next().unwrap().to_string();
        con.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
        con.entry(b).or_insert(Vec::new()).push(a);
    }
    con
}

pub fn part1(input: Input) -> usize {
    let mut triangles = HashSet::new();
    for cmp in input.keys().filter(|c| c.starts_with("t")) {
        for pair in input.get(cmp).unwrap().iter().combinations(2) {
            if input.get(pair[0]).unwrap().contains(pair[1]) {
                let mut v = vec![cmp, pair[0], pair[1]];
                v.sort();
                triangles.insert(v);
            }
        }
    }
    triangles.len()
}

pub fn part2(input: Input) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1476);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), "");
    }
}

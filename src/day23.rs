use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::util::load;

type Input = HashMap<String, HashSet<String>>;

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day23.txt");
    let mut con = HashMap::new();
    for line in lines {
        let mut token_it = line.split("-");
        let a = token_it.next().unwrap().to_string();
        let b = token_it.next().unwrap().to_string();
        con.entry(a.clone())
            .or_insert(HashSet::new())
            .insert(b.clone());
        con.entry(b).or_insert(HashSet::new()).insert(a);
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

/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Without_pivoting
// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}
pub fn part2(input: Input) -> String {
    fn bron_kerbosch(
        n: &Input, // neighbours
        r: HashSet<String>,
        mut p: HashSet<String>,
        mut x: HashSet<String>,
    ) -> Option<HashSet<String>> {
        if p.is_empty() && p.is_empty() {
            return Some(r);
        }
        let mut clique = HashSet::new();
        while !p.is_empty() {
            let v = p.iter().next().unwrap().to_string();
            if let Some(c) = bron_kerbosch(
                n,
                r.union(&HashSet::from([v.clone()])).cloned().collect(),
                p.intersection(n.get(&v).unwrap()).cloned().collect(),
                x.intersection(n.get(&v).unwrap()).cloned().collect(),
            ) {
                if c.len() > clique.len() {
                    clique = c
                }
            }
            p.remove(&v);
            x.insert(v.to_string());
        }
        Some(clique)
    }
    let r = HashSet::new();
    let p = input.keys().cloned().collect();
    let x = HashSet::new();
    let mut c = bron_kerbosch(&input, r, p, x)
        .unwrap()
        .into_iter()
        .collect::<Vec<String>>();
    c.sort();
    c.join(",")
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
        assert_eq!(part2(input()), "ca,dw,fo,if,ji,kg,ks,oe,ov,sb,ud,vr,xr");
    }
}

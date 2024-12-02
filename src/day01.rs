use std::{collections::HashMap, iter::zip};

use crate::util::load;

type Input = (Vec<i32>, Vec<i32>);

pub fn input() -> Input {
    let values: Vec<String> = load("data/day01.txt");
    values
        .into_iter()
        .fold((vec![], vec![]), |(mut v1, mut v2), s| {
            let n: Vec<_> = s
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect();
            v1.push(n[0]);
            v2.push(n[1]);
            (v1, v2)
        })
}

pub fn part1((mut v1, mut v2): Input) -> i32 {
    v1.sort();
    v2.sort();
    zip(v1, v2).fold(0, |acc, (a, b)| acc + (a - b).abs())
}

fn freq(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut freq = HashMap::new();
    v.into_iter().for_each(|i| *freq.entry(i).or_insert(0) += 1);
    freq
}

pub fn part2((v1, v2): Input) -> i32 {
    let f1 = freq(v1);
    let f2 = freq(v2);
    f1.into_iter()
        .fold(0, |acc, (k, v)| acc + k * v * f2.get(&k).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 936063);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 23150395);
    }
}

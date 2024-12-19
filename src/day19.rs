use std::collections::HashMap;

use crate::util::load;

type Input = (Vec<String>, Vec<String>);

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day19.txt");
    let towels = lines[0].split(", ").map(String::from).collect();
    (towels, lines[2..].to_vec())
}

fn possible_arrangements(
    pattern: &str,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let mut cnt = 0;
    if cache.contains_key(pattern) {
        return *cache.get(pattern).unwrap();
    }
    if pattern.is_empty() {
        return 1;
    }
    for towel in towels {
        if pattern.starts_with(towel) {
            let n = possible_arrangements(pattern[towel.len()..].as_ref(), towels, cache);
            if n > 0 {
                cache.insert(pattern[towel.len()..].to_string(), n);
            }
            cnt += n;
        }
    }
    cnt
}

pub fn part1(input: Input) -> usize {
    let mut cache = HashMap::new();
    input
        .1
        .into_iter()
        .filter(|pattern| possible_arrangements(pattern, &input.0, &mut cache) != 0)
        .count()
}

pub fn part2(input: Input) -> usize {
    let mut cache = HashMap::new();
    input
        .1
        .into_iter()
        .map(|pattern| possible_arrangements(&pattern, &input.0, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 213);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1016700771200474);
    }
}

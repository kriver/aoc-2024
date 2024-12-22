use std::collections::HashMap;

use crate::util::load;

type Input = Vec<i64>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day22.txt");
    values.into_iter().map(|s| s.parse().unwrap()).collect()
}

fn prng1(seed: i64) -> i64 {
    fn mix_prune(old: i64, mut new: i64) -> i64 {
        new ^= old;
        new % 16777216
    }
    let mut v = seed;
    v = mix_prune(v, v * 64);
    v = mix_prune(v, v / 32);
    v = mix_prune(v, v * 2048);
    v
}

fn prng(seed: i64, times: usize) -> Vec<i64> {
    (0..times)
        .fold((seed, vec![seed]), |(mut rnd, mut v), _| {
            rnd = prng1(rnd);
            v.push(rnd);
            (rnd, v)
        })
        .1
}

pub fn part1(initial: Input) -> i64 {
    initial
        .into_iter()
        .map(|i| prng(i, 2000))
        .map(|v| v[v.len() - 1])
        .sum()
}

pub fn part2(initial: Input) -> i64 {
    let mut all: HashMap<[i64; 4], i64> = HashMap::new();
    for i in initial {
        let secrets = prng(i, 2000);
        let bananas = secrets.into_iter().map(|s| s % 10).collect::<Vec<_>>();
        let deltas = bananas.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        let mut h = HashMap::new();
        for (i, w) in deltas.windows(4).enumerate() {
            h.entry(w).or_insert(bananas[i + 4]);
        }
        h.iter().for_each(|(k, v)| {
            all.entry((**k).try_into().unwrap())
                .and_modify(|s| *s += v)
                .or_insert(*v);
        });
    }
    *all.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 19927218456);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 2189);
    }
}

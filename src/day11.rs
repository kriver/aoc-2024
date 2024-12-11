use std::collections::HashMap;

use crate::util::load;

type Input = HashMap<String, usize>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day11.txt");
    values[0]
        .split_whitespace()
        .map(|s| (s.to_owned(), 1))
        .collect()
}

fn multi_blink(stones: Input, times: usize) -> Input {
    fn add(stones: &mut Input, stone: String, cnt: usize) {
        stones.entry(stone).and_modify(|c| *c += cnt).or_insert(cnt);
    }
    fn blink(stones: Input) -> Input {
        let mut new_stones = HashMap::new();
        for (stone, cnt) in stones {
            if stone == "0" {
                add(&mut new_stones, "1".to_string(), cnt);
            } else if stone.len() % 2 == 0 {
                let l = stone.len() / 2;
                add(&mut new_stones, stone[0..l].to_owned(), cnt);
                add(
                    &mut new_stones,
                    stone[l..].parse::<u64>().unwrap().to_string(),
                    cnt,
                );
            } else {
                add(
                    &mut new_stones,
                    (stone.parse::<u64>().unwrap() * 2024).to_string(),
                    cnt,
                );
            }
        }
        new_stones
    }

    (0..times)
        .into_iter()
        .fold(stones, |stones, _i| blink(stones))
}

pub fn part1(stones: Input) -> usize {
    multi_blink(stones, 25).values().sum()
}

pub fn part2(stones: Input) -> usize {
    multi_blink(stones, 75).values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 233875);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 277444936413293);
    }
}

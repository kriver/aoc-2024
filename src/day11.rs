use crate::util::load;

type Input = Vec<String>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day11.txt");
    values[0].split_whitespace().map(|s| s.to_owned()).collect()
}

fn multi_blink(stones: Input, times: usize) -> Input {
    fn blink(stones: Input) -> Input {
        let mut new_stones = vec![];
        for stone in stones {
            if stone == "0" {
                new_stones.push("1".to_owned());
            } else if stone.len() % 2 == 0 {
                let l = stone.len() / 2;
                new_stones.push(stone[0..l].to_owned());
                new_stones.push(stone[l..].parse::<u64>().unwrap().to_string());
            } else {
                new_stones.push((stone.parse::<u64>().unwrap() * 2024).to_string());
            }
        }
        new_stones
    }

    (0..times)
        .into_iter()
        .fold(stones, |stones, _i| blink(stones))
}

pub fn part1(stones: Input) -> usize {
    multi_blink(stones, 25).len()
}

pub fn part2(stones: Input) -> usize {
    0
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
        assert_eq!(part2(input()), 0);
    }
}

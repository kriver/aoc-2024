use crate::util::load;

type Input = Vec<(u64, Vec<u64>)>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day07.txt");
    values
        .into_iter()
        .map(|line| {
            let tokens = line.split(": ").collect::<Vec<_>>();
            let num = tokens[1]
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (tokens[0].parse::<u64>().unwrap(), num)
        })
        .collect()
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn is_valid(res: u64, val: u64, nums: &[u64], part2: bool) -> bool {
    if nums.len() == 0 {
        res == val
    } else {
        if is_valid(res, val + nums[0], &nums[1..], part2) {
            true
        } else if is_valid(res, val * nums[0], &nums[1..], part2) {
            true
        } else if part2 {
            is_valid(res, concat(val, nums[0]), &nums[1..], part2)
        } else {
            false
        }
    }
}

pub fn part1(input: Input) -> u64 {
    input
        .into_iter()
        .filter(|(res, nums)| is_valid(*res, nums[0], &nums[1..], false))
        .map(|t| t.0)
        .sum()
}

pub fn part2(input: Input) -> u64 {
    input
        .into_iter()
        .filter(|(res, nums)| is_valid(*res, nums[0], &nums[1..], true))
        .map(|t| t.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 7710205485870);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 20928985450275);
    }
}

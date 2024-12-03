use num_traits::signum;

use crate::util::load;

type Input = Vec<Vec<i32>>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day02.txt");
    values
        .into_iter()
        .map(|s| {
            s.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(v: &[i32], skip: Option<usize>) -> bool {
    let mut prev: Option<i32> = None;
    let mut sigsum = 0;
    for i in 0..v.len() {
        if let Some(k) = skip {
            if i == k {
                continue;
            }
        }
        if let Some(p) = prev {
            let delta = v[i] - p;
            if delta.abs() < 1 || delta.abs() > 3 {
                return false;
            }
            sigsum += signum(delta);
        }
        prev = Some(v[i]);
    }
    sigsum.abs() == v.len() as i32 - (if let None = skip { 1 } else { 2 })
}

fn is_safe_with_tolerance(v: &[i32]) -> bool {
    if is_safe(v, None) {
        return true;
    }
    for skip in 0..v.len() {
        if is_safe(v, Some(skip)) {
            return true;
        }
    }
    false
}

pub fn part1(input: Input) -> usize {
    input.into_iter().filter(|v| is_safe(v, None)).count()
}

pub fn part2(input: Input) -> usize {
    input
        .into_iter()
        .filter(|v| is_safe_with_tolerance(v))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 670);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 700);
    }
}

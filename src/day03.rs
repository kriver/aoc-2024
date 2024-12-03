use regex::Regex;

use crate::util::load;

type Input = Vec<String>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day03.txt");
    values
}

pub fn part1(values: Input) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    values
        .into_iter()
        .map(|line| {
            re.captures_iter(&line)
                .map(|c| {
                    let a = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let b = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                    a * b
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part2(values: Input) -> u32 {
    values.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 162813399);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

use regex::Regex;

use crate::util::load;

type Input = Vec<String>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day03.txt");
    values
}

fn process(values: Input, do_not: bool) -> u32 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    values
        .into_iter()
        .map(|line| {
            re.captures_iter(&line)
                .map(|c| {
                    let mut v = 0;
                    if let Some(_) = c.get(1) {
                        if enabled {
                            let a = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
                            let b = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
                            v = a * b
                        }
                    } else if let Some(_) = c.get(4) {
                        enabled = true;
                    } else if let Some(_) = c.get(5) {
                        enabled = if do_not { false } else { true };
                    }
                    v
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part1(values: Input) -> u32 {
    process(values, false)
}

pub fn part2(values: Input) -> u32 {
    process(values, true)
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
        assert_eq!(part2(input()), 53783319);
    }
}

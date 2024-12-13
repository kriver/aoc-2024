use regex::{Captures, Regex};

use crate::util::load;

const PRICE_BUTTON_A: u32 = 3;
const PRICE_BUTTON_B: u32 = 1;

#[derive(Debug)]
pub struct Claw {
    xa: u32,
    ya: u32,
    xb: u32,
    yb: u32,
    xp: u32,
    yp: u32,
}
type Input = Vec<Claw>;

pub fn input() -> Input {
    fn get(c: &Captures, i: usize) -> u32 {
        c.get(i).unwrap().as_str().parse().unwrap()
    }
    let re = Regex::new(r"^[^\d]*(\d+)[^\d]*(\d+)[^d]*$").unwrap();
    let lines: Vec<String> = load("data/day13.txt");
    let mut claws = vec![];
    for i in (0..lines.len()).step_by(4) {
        let ca = re.captures(&lines[i + 0]).unwrap();
        let cb = re.captures(&lines[i + 1]).unwrap();
        let cp = re.captures(&lines[i + 2]).unwrap();
        claws.push(Claw {
            xa: get(&ca, 1),
            ya: get(&ca, 2),
            xb: get(&cb, 1),
            yb: get(&cb, 2),
            xp: get(&cp, 1),
            yp: get(&cp, 2),
        });
    }
    claws
}

pub fn part1(claws: Input) -> u32 {
    fn cost(c: Claw) -> u32 {
        for a in 1..=100 {
            for b in 1..=100 {
                if (a * c.xa + b * c.xb == c.xp) && (a * c.ya + b * c.yb == c.yp) {
                    return a * PRICE_BUTTON_A + b * PRICE_BUTTON_B;
                }
            }
        }
        0
    }
    claws.into_iter().map(|c| cost(c)).sum()
}

pub fn part2(values: Input) -> u32 {
    values.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 28887);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

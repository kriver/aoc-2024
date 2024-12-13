use regex::{Captures, Regex};

use crate::util::load;

const PRICE_BUTTON_A: i64 = 3;
const PRICE_BUTTON_B: i64 = 1;

#[derive(Debug)]
pub struct Claw {
    xa: i64,
    ya: i64,
    xb: i64,
    yb: i64,
    xp: i64,
    yp: i64,
}
type Input = Vec<Claw>;

pub fn input() -> Input {
    fn get(c: &Captures, i: usize) -> i64 {
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

fn cost(c: Claw, offset: i64) -> i64 {
    let mut cost = 0;
    let b_num = (c.yp + offset) * c.xa - (c.xp + offset) * c.ya;
    let b_den = c.xa * c.yb - c.xb * c.ya;
    if b_num % b_den == 0 {
        let b = b_num / b_den;
        let a_num = (c.xp + offset) - c.xb * b;
        let a_den = c.xa;
        if a_num % a_den == 0 {
            let a = a_num / a_den;
            cost = a * PRICE_BUTTON_A + b * PRICE_BUTTON_B;
        }
    }
    cost
}

pub fn part1(claws: Input) -> i64 {
    claws.into_iter().map(|c| cost(c, 0)).sum()
}

pub fn part2(claws: Input) -> i64 {
    claws.into_iter().map(|c| cost(c, 10000000000000)).sum()
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
        assert_eq!(part2(input()), 96979582619758);
    }
}

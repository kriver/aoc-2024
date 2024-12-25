use crate::util::load;

type LockKey = [u32; 5];
type Input = (Vec<LockKey>, Vec<LockKey>);

pub fn input() -> Input {
    fn parse_block(lines: &[String]) -> LockKey {
        let mut lk = [0; 5];
        for j in 0..=6 {
            lines[j].chars().enumerate().for_each(|(p, c)| {
                if c == '#' {
                    lk[p] += 1;
                }
            });
        }
        lk
    }
    let lines: Vec<String> = load("data/day25.txt");
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for i in (0..lines.len()).step_by(8) {
        if lines[i] == "....." {
            keys.push(parse_block(&lines[i..]));
        } else {
            locks.push(parse_block(&lines[i..]));
        }
    }
    (locks, keys)
}

pub fn part1((locks, keys): Input) -> usize {
    let mut cnt = 0;
    for l in locks {
        for k in keys.iter() {
            let mut fit = true;
            for i in 0..5 {
                if l[i] + k[i] > 7 {
                    fit = false;
                    break;
                }
            }
            if fit {
                cnt += 1;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 3320);
    }
}

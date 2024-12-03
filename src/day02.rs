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

fn is_safe(v: Vec<i32>) -> bool {
    let incr = v[0] < v[1];
    for i in 1..v.len() {
        if incr && v[i - 1] > v[i] {
            return false;
        } else if !incr && v[i - 1] < v[i] {
            return false;
        }
        let delta = (v[i - 1] - v[i]).abs();
        if delta < 1 || delta > 3 {
            return false;
        }
    }
    return true;
}

pub fn part1(input: Input) -> usize {
    // FIXME fix clone()
    input.into_iter().filter(|v| is_safe(v.clone())).count()
}

pub fn part2(values: Input) -> u32 {
    values.len() as u32
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
        assert_eq!(part2(input()), 0);
    }
}

use crate::util::load;

type Input = Vec<String>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day05.txt");
    values
}

pub fn part1(values: Input) -> u32 {
    values.len() as u32
}

pub fn part2(values: Input) -> u32 {
    values.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

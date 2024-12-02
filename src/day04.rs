use crate::util::load;

pub fn input() -> Vec<String> {
    let values: Vec<String> = load("data/day04.txt");
    values
}

pub fn part1(values: Vec<String>) -> u32 {
    values.len() as u32
}

pub fn part2(values: Vec<String>) -> u32 {
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

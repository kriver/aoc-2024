use crate::util::{load, Coord2D};

type Input = Vec<Vec<char>>;
const DIRS1: [Coord2D<i32>; 8] = [
    Coord2D::new(1, 0),
    Coord2D::new(1, 1),
    Coord2D::new(0, 1),
    Coord2D::new(-1, 1),
    Coord2D::new(-1, 0),
    Coord2D::new(-1, -1),
    Coord2D::new(0, -1),
    Coord2D::new(1, -1),
];
const XMAS: &'static str = "XMAS";

pub fn input() -> Input {
    let values: Vec<String> = load("data/day04.txt");
    values.into_iter().map(|s| s.chars().collect()).collect()
}

pub fn part1(values: Input) -> u32 {
    fn is_match(values: &Input, x: usize, y: usize, d: &Coord2D<i32>, i: usize, c: char) -> bool {
        let x = x as i32 + d.x * i as i32;
        let y = y as i32 + d.y * i as i32;
        if x < 0 || x >= values[0].len() as i32 || y < 0 || y >= values.len() as i32 {
            return false;
        }
        values[y as usize][x as usize] == c
    }

    let mut cnt = 0;
    for y in 0..values.len() {
        for x in 0..values[0].len() {
            if values[y][x] == 'X' {
                for d in &DIRS1 {
                    let mut found = true;
                    for (i, c) in XMAS.chars().into_iter().skip(1).enumerate() {
                        found = is_match(&values, x, y, &d, i + 1, c);
                        if !found {
                            break;
                        }
                    }
                    if found {
                        cnt += 1;
                    }
                }
            }
        }
    }
    cnt
}

pub fn part2(values: Input) -> u32 {
    fn is_match(values: &Input, x: usize, y: usize) -> bool {
        if x < 1 || x >= values[0].len() - 1 || y < 1 || y >= values.len() - 1 {
            return false;
        }
        let lt = values[y - 1][x - 1];
        let rt = values[y - 1][x + 1];
        let lb = values[y + 1][x - 1];
        let rb = values[y + 1][x + 1];
        if ((lt == 'M' && rb == 'S') || (lt == 'S' && rb == 'M'))
            && ((rt == 'M' && lb == 'S') || (rt == 'S' && lb == 'M'))
        {
            return true;
        }
        false
    }
    let mut cnt = 0;
    for y in 0..values.len() {
        for x in 0..values[0].len() {
            if values[y][x] == 'A' {
                if is_match(&values, x, y) {
                    cnt += 1;
                }
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
        assert_eq!(part1(input()), 2554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1916);
    }
}

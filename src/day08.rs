use std::collections::{HashMap, HashSet};

use crate::util::{load, Coord2D};

// antenna with frequency to coordinates
type Coord = Coord2D<i32>;
type Antennas = HashMap<char, Vec<Coord>>;
type Input = (Coord, Antennas);

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day08.txt");
    let sz = Coord2D::new(lines[0].len() as i32, lines.len() as i32);
    let mut antennas: Antennas = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let coord = Coord2D::new(x as i32, y as i32);
                antennas
                    .entry(c)
                    .and_modify(|v| v.push(coord))
                    .or_insert(vec![coord]);
            }
        }
    }
    (sz, antennas)
}

fn is_valid(c: &Coord, upper: &Coord) -> bool {
    c.x >= 0 && c.x < upper.x && c.y >= 0 && c.y < upper.y
}

pub fn part1((sz, antennas): Input) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for coords in antennas.values() {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let c1 = coords[i];
                let c2 = coords[j];
                let delta = c1 - c2;
                let a1 = c1 + delta;
                if is_valid(&a1, &sz) {
                    antinodes.insert(a1);
                }
                let a2 = c2 - delta;
                if is_valid(&a2, &sz) {
                    antinodes.insert(a2);
                }
            }
        }
    }
    antinodes.len()
}

pub fn part2((sz, antennas): Input) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();
    for coords in antennas.values() {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let c1 = coords[i];
                let c2 = coords[j];
                let delta = c1 - c2;
                let mut c = c1;
                loop {
                    antinodes.insert(c);
                    c = c + delta;
                    if !is_valid(&c, &sz) {
                        break;
                    }
                }
                let mut c = c2;
                loop {
                    antinodes.insert(c);
                    c = c - delta;
                    if !is_valid(&c, &sz) {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 269);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 949);
    }
}

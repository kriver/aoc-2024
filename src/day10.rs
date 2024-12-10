use std::collections::HashSet;

use crate::util::{char2num, load, Coord2D};

type Coord = Coord2D<usize>;
type Map = Vec<Vec<u8>>;
type TrailHeads = Vec<Coord>;
type Input = (Map, TrailHeads);

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day10.txt");
    let mut trail_heads = vec![];
    let map = lines
        .into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = char2num(c);
                    if height == 0 {
                        trail_heads.push(Coord::new(x, y));
                    }
                    height
                })
                .collect()
        })
        .collect();
    (map, trail_heads)
}

fn calc_score(th: Coord, map: &Map) -> usize {
    fn walk(loc: HashSet<Coord>, map: &Map, height: u8) -> HashSet<Coord> {
        let mut new_loc = HashSet::new();
        for Coord { x, y } in loc {
            if x > 0 {
                if map[y][x - 1] == height {
                    new_loc.insert(Coord::new(x - 1, y));
                }
            }
            if x < map[0].len() - 1 {
                if map[y][x + 1] == height {
                    new_loc.insert(Coord::new(x + 1, y));
                }
            }
            if y > 0 {
                if map[y - 1][x] == height {
                    new_loc.insert(Coord::new(x, y - 1));
                }
            }
            if y < map.len() - 1 {
                if map[y + 1][x] == height {
                    new_loc.insert(Coord::new(x, y + 1));
                }
            }
        }
        new_loc
    }
    let mut loc = HashSet::new();
    loc.insert(th);
    for h in 1..=9 {
        loc = walk(loc, map, h);
    }
    loc.len()
}

pub fn part1((map, trail_heads): Input) -> usize {
    trail_heads.into_iter().map(|th| calc_score(th, &map)).sum()
}

pub fn part2(input: Input) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 531);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

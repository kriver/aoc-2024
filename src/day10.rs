use std::collections::{HashMap, HashSet};

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

fn calc_score(th: Coord, map: &Map) -> HashMap<Coord, usize> {
    fn insert(loc: &mut HashMap<Coord, usize>, c: Coord, cnt: usize) {
        loc.entry(c).and_modify(|c| *c += cnt).or_insert(cnt);
    }
    fn walk(loc: HashMap<Coord, usize>, map: &Map, height: u8) -> HashMap<Coord, usize> {
        let mut new_loc = HashMap::new();
        for (Coord { x, y }, cnt) in loc {
            if x > 0 {
                if map[y][x - 1] == height {
                    insert(&mut new_loc, Coord::new(x - 1, y), cnt);
                }
            }
            if x < map[0].len() - 1 {
                if map[y][x + 1] == height {
                    insert(&mut new_loc, Coord::new(x + 1, y), cnt);
                }
            }
            if y > 0 {
                if map[y - 1][x] == height {
                    insert(&mut new_loc, Coord::new(x, y - 1), cnt);
                }
            }
            if y < map.len() - 1 {
                if map[y + 1][x] == height {
                    insert(&mut new_loc, Coord::new(x, y + 1), cnt);
                }
            }
        }
        new_loc
    }
    let mut loc = HashMap::new();
    loc.insert(th, 1);
    for h in 1..=9 {
        loc = walk(loc, map, h);
    }
    loc
}

pub fn part1((map, trail_heads): Input) -> usize {
    trail_heads
        .into_iter()
        .map(|th| calc_score(th, &map))
        .map(|loc| loc.len())
        .sum()
}

pub fn part2((map, trail_heads): Input) -> usize {
    trail_heads
        .into_iter()
        .map(|th| calc_score(th, &map))
        .map(|loc| loc.values().sum::<usize>())
        .sum()
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
        assert_eq!(part2(input()), 1210);
    }
}

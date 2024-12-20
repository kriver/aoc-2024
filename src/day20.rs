use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::util::{load, Coord2D, Direction, Grid};

type Coord = Coord2D<i32>;
type Maze = Grid<i32, u32>;
type Walls = Grid<i32, u32>;
type Cheat = (Coord, Coord, u32);

pub struct Input {
    start: Coord,
    end: Coord,
    maze: Maze,
    walls: Walls,
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day20.txt");
    let (width, height) = (lines[0].len(), lines.len());
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    let mut squares = HashMap::new();
    let mut walls = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord::new(x as i32, y as i32);
            if c != '#' {
                squares.insert(coord, 0);
            }
            match c {
                'S' => start = coord,
                'E' => end = coord,
                '#' => {
                    if x != 0 && y != 0 && x != width - 1 && y != height - 1 {
                        walls.insert(coord, 0);
                    }
                }
                _ => (),
            };
        }
    }
    Input {
        start,
        end,
        maze: Grid {
            width: width as i32,
            height: height as i32,
            squares,
        },
        walls: Grid {
            width: width as i32,
            height: height as i32,
            squares: walls,
        },
    }
}

fn calc_costs(input: &mut Input) {
    let squares = &mut input.maze.squares;
    let mut cost = 0;
    let mut p = input.start;
    while p != input.end {
        p = Direction::iter()
            .map(|d| p + d.into())
            .find(|p| squares.contains_key(p) && squares[p] == 0)
            .unwrap();
        cost += 1;
        squares.insert(p, cost);
    }
}

fn build_cheat_list(input: &mut Input) -> Vec<Cheat> {
    let mut cheats = Vec::new();
    let squares = &input.maze.squares;
    let walls = &mut input.walls.squares;
    for (w, _) in walls {
        let lp = *w + Direction::Left.into();
        if let Some(_) = squares.get(&lp) {
            let rp = *w + Direction::Right.into();
            if let Some(_) = squares.get(&rp) {
                cheats.push((lp, rp, 2));
            }
        }
        let up = *w + Direction::Up.into();
        if let Some(_) = squares.get(&up) {
            let dp = *w + Direction::Down.into();
            if let Some(_) = squares.get(&dp) {
                cheats.push((up, dp, 2));
            }
        }
    }
    cheats
}

pub fn part1(mut input: Input) -> u32 {
    calc_costs(&mut input);
    let cheats = build_cheat_list(&mut input);
    let squares = &input.maze.squares;
    let mut cnt = 0;
    for (p1, p2, cost) in cheats {
        if cost != 2 {
            continue;
        }
        let c1 = squares[&p1];
        let c2 = squares[&p2];
        if c1.abs_diff(c2) - cost >= 100 {
            cnt += 1;
        }
    }
    cnt
}

pub fn part2(mut input: Input) -> u32 {
    calc_costs(&mut input);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1441);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

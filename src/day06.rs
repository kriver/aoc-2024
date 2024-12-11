use std::collections::{HashMap, HashSet};

use crate::util::{load, Coord2D, Direction};

type Obstacles = HashSet<Coord2D<i32>>;
type Path = HashMap<Coord2D<i32>, Vec<Direction>>;

pub struct Input {
    guard: Coord2D<i32>,
    dir: Direction,
    obstacles: Obstacles,
    sz: Coord2D<i32>,
}

fn move_dir(mut c: Coord2D<i32>, dir: &Direction) -> Coord2D<i32> {
    match dir {
        Direction::Up => c.y -= 1,
        Direction::Down => c.y += 1,
        Direction::Left => c.x -= 1,
        Direction::Right => c.x += 1,
    }
    c
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day06.txt");
    let sz = Coord2D::new(lines[0].len() as i32, lines.len() as i32);
    let mut guard = Coord2D::new(0, 0);
    let mut obstacles = HashSet::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, p) in line.chars().enumerate() {
            let c = Coord2D {
                x: x as i32,
                y: y as i32,
            };
            match p {
                '#' => {
                    obstacles.insert(c);
                }
                '^' => {
                    guard.x = c.x;
                    guard.y = c.y;
                }
                _ => (),
            }
        }
    }
    Input {
        guard,
        dir: Direction::Up,
        obstacles,
        sz,
    }
}

fn patrol(input: &mut Input) -> Path {
    let mut pos = input.guard;
    let mut dir = input.dir;
    let mut path = HashMap::new();
    path.insert(pos, vec![dir]);
    loop {
        let new_pos = move_dir(pos, &dir);
        let Coord2D { x, y } = new_pos;
        if x < 0 || y < 0 || x >= input.sz.x || y >= input.sz.y {
            break;
        }
        if input.obstacles.contains(&new_pos) {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            }
        } else {
            path.entry(new_pos)
                .and_modify(|v| v.push(dir))
                .or_insert(vec![dir]);
            pos.x = x;
            pos.y = y;
        }
    }
    path
}

pub fn part1(mut input: Input) -> usize {
    let path = patrol(&mut input);
    path.len()
}

pub fn part2(mut input: Input) -> u32 {
    let _path = patrol(&mut input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 5208);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

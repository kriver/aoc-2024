use std::collections::HashMap;

use crate::util::{load, Coord2D, Direction, Grid};

#[derive(PartialEq)]
pub enum Location {
    Box,
    Wall,
}
type Moves = Vec<Direction>;
type Coord = Coord2D<i32>;

type Input = (Coord, Grid<i32, Location>, Moves);

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => unreachable!("Unexpected direction"),
        }
    }
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day15.txt");
    let mut robot = Coord::new(0, 0);
    let mut squares: HashMap<Coord, Location> = HashMap::new();
    let mut moves = vec![];
    let mut grid = true;
    let (mut width, mut height) = (0, 0);
    for line in lines {
        if line.is_empty() {
            grid = false;
            continue;
        }
        if grid {
            width = line.len() as i32;
            for (x, c) in line.chars().enumerate() {
                match c {
                    '@' => {
                        robot.x = x as i32;
                        robot.y = height;
                    }
                    '#' => {
                        squares.insert(Coord::new(x as i32, height), Location::Wall);
                    }
                    'O' => {
                        squares.insert(Coord::new(x as i32, height), Location::Box);
                    }
                    '.' => (),
                    _ => unreachable!("Should not happen"),
                }
            }
            height += 1;
        } else {
            let mut m = line.chars().map(|c| c.into()).collect();
            moves.append(&mut m);
        }
    }
    (
        robot,
        Grid {
            width,
            height,
            squares,
        },
        moves,
    )
}

pub fn part1((mut robot, mut grid, moves): Input) -> i32 {
    for m in moves {
        let d = m.into();
        let robot_new = robot + d;
        let mut p = robot_new;
        while let Some(Location::Box) = grid.squares.get(&p) {
            p = p + d;
        }
        if None == grid.squares.get(&p) {
            if let Some(Location::Box) = grid.squares.remove(&robot_new) {
                grid.squares.insert(p, Location::Box);
            }
            robot = robot_new;
        }
    }
    grid.squares
        .into_iter()
        .filter(|(_, v)| *v == Location::Box)
        .map(|(c, _)| 100 * c.y + c.x)
        .sum()
}

pub fn part2(values: Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1552463);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

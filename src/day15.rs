use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::{load, Coord2D, Direction, Grid};

#[derive(PartialEq)]
pub enum Location {
    LBox,
    RBox,
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

pub fn input(double: bool) -> Input {
    let lines: Vec<String> = load("data/day15.txt");
    let mut robot = Coord::new(0, 0);
    let mut squares: HashMap<Coord, Location> = HashMap::new();
    let mut moves = vec![];
    let mut grid = true;
    let (width, mut height) = (lines[0].len() as i32, 0);
    for line in lines {
        if line.is_empty() {
            grid = false;
            continue;
        }
        if grid {
            for (x, c) in line.chars().enumerate() {
                let lx = if double { x * 2 } else { x } as i32;
                match c {
                    '@' => {
                        robot.x = lx;
                        robot.y = height;
                    }
                    '#' => {
                        squares.insert(Coord::new(lx, height), Location::Wall);
                        if double {
                            squares.insert(Coord::new(lx + 1, height), Location::Wall);
                        }
                    }
                    'O' => {
                        squares.insert(Coord::new(lx, height), Location::LBox);
                        if double {
                            squares.insert(Coord::new(lx + 1, height), Location::RBox);
                        }
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

fn gps(grid: Grid<i32, Location>) -> i32 {
    grid.squares
        .into_iter()
        .filter(|(_, v)| *v == Location::LBox)
        .map(|(c, _)| 100 * c.y + c.x)
        .sum()
}

pub fn part1((mut robot, mut grid, moves): Input) -> i32 {
    for m in moves {
        let d = m.into();
        let robot_new = robot + d;
        let mut p = robot_new;
        while let Some(Location::LBox) = grid.squares.get(&p) {
            p = p + d;
        }
        if None == grid.squares.get(&p) {
            if let Some(Location::LBox) = grid.squares.remove(&robot_new) {
                grid.squares.insert(p, Location::LBox);
            }
            robot = robot_new;
        }
    }
    gps(grid)
}

pub fn part2((mut robot, mut grid, moves): Input) -> i32 {
    for m in moves {
        let d = m.into();
        let robot_new = robot + d;
        match m {
            Direction::Left | Direction::Right => {
                let mut p = robot_new;
                loop {
                    match grid.squares.get(&p) {
                        None => {
                            while p != robot_new {
                                let new_p = p - d;
                                let b = grid.squares.remove(&new_p).unwrap();
                                grid.squares.insert(p, b);
                                p = new_p;
                            }
                            robot = robot_new;
                            break;
                        }
                        Some(Location::Wall) => break, // can't move
                        _ => {
                            p = p + d;
                        }
                    }
                }
            }
            Direction::Up | Direction::Down => {
                let mut to_move = vec![];
                let mut to_check = VecDeque::from([robot]);
                while !to_check.is_empty() {
                    let p = to_check.pop_front().unwrap();
                    to_move.push(p);
                    let new_p = p + d;
                    match grid.squares.get(&new_p) {
                        None => (),
                        Some(Location::Wall) => {
                            to_move.clear();
                            break;
                        }
                        Some(Location::LBox) => {
                            to_check.push_back(new_p);
                            to_check.push_back(new_p + Direction::Right.into());
                        }
                        Some(Location::RBox) => {
                            to_check.push_back(new_p);
                            to_check.push_back(new_p + Direction::Left.into());
                        }
                    }
                }
                let mut unique: HashSet<_> = to_move.iter().copied().collect();
                while !to_move.is_empty() {
                    let p = to_move.pop().unwrap();
                    if unique.remove(&p) {
                        if p == robot {
                            robot = robot_new;
                        } else {
                            let new_p = p + d;
                            let b = grid.squares.remove(&p).unwrap();
                            grid.squares.insert(new_p, b);
                        }
                    }
                }
            }
        }
    }
    gps(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input(false)), 1552463);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input(true)), 1554058);
    }
}

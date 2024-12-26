use std::collections::{HashMap, HashSet};

use crate::util::{load, Coord2D, Direction};

type Coord = Coord2D<i32>;
type Obstacles = HashSet<Coord>;
type Path = HashMap<Coord, HashSet<Direction>>;

pub struct Input {
    guard: Coord,
    dir: Direction,
    obstacles: Obstacles,
    sz: Coord,
}

impl Input {
    fn is_off_map(&self, Coord { x, y }: Coord) -> bool {
        x < 0 || y < 0 || x >= self.sz.x || y >= self.sz.y
    }

    /// returns true if patrol ends up in an endless loop
    fn try_patrol(
        &mut self,
        mut pos: Coord,
        mut dir: Direction,
        path: &mut Path,
        mut loop_obstacles: Option<&mut Obstacles>,
    ) -> bool {
        let mut lp: Path = Path::new();
        loop {
            let new_pos = pos + dir.into();
            if self.is_off_map(new_pos) {
                break false;
            }
            if self.obstacles.contains(&new_pos) {
                dir = dir.turn_right();
            } else {
                match loop_obstacles {
                    Some(ref mut lo) => {
                        // check whether an obstacle would result in a loop
                        if !path.contains_key(&new_pos) {
                            // only if path hasn't crossed here before!!!
                            self.obstacles.insert(new_pos);
                            if self.try_patrol(pos, dir, path, None) {
                                lo.insert(new_pos);
                            }
                            self.obstacles.remove(&new_pos);
                        }
                        path.entry(new_pos).or_insert(HashSet::new()).insert(dir);
                    }
                    None => {
                        let in_path = path.contains_key(&new_pos) && path[&new_pos].contains(&dir);
                        let in_lp = lp.contains_key(&new_pos) && lp[&new_pos].contains(&dir);
                        if in_path || in_lp {
                            break true;
                        } else {
                            lp.entry(new_pos).or_insert(HashSet::new()).insert(dir);
                        }
                    }
                }
                pos = new_pos;
            }
        }
    }

    fn patrol(&mut self) -> (Path, Obstacles) {
        let mut loop_obstacles = HashSet::new();
        let mut path = Path::new();
        path.insert(self.guard, HashSet::from([self.dir]));
        self.try_patrol(self.guard, self.dir, &mut path, Some(&mut loop_obstacles));
        (path, loop_obstacles)
    }
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

pub fn part1(mut input: Input) -> usize {
    let (path, _) = input.patrol();
    let distinct: HashSet<_> = path.into_iter().map(|(c, _)| c).collect();
    distinct.len()
}

pub fn part2(mut input: Input) -> usize {
    let (_, loop_obstacles) = input.patrol();
    loop_obstacles.len()
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
        assert_eq!(part2(input()), 1972);
    }
}

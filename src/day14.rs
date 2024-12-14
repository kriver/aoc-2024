use regex::{Captures, Regex};

use crate::util::{load, Coord2D};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const WIDTH_SZ: usize = WIDTH as usize;
const HEIGHT_SZ: usize = HEIGHT as usize;

type Coord = Coord2D<i32>;

pub struct Robot {
    p: Coord,
    v: Coord,
}

type Input = Vec<Robot>;

impl Robot {
    fn quadrant(&self) -> Option<usize> {
        let left = self.p.x < WIDTH / 2;
        let top = self.p.y < HEIGHT / 2;
        if self.p.x == WIDTH / 2 || self.p.y == HEIGHT / 2 {
            None
        } else if left & top {
            Some(0)
        } else if !left & top {
            Some(1)
        } else if left & !top {
            Some(2)
        } else {
            Some(3)
        }
    }

    fn patrol(&mut self, time: i32) {
        self.p.x = (self.p.x + (time * self.v.x)) % WIDTH;
        if self.p.x < 0 {
            self.p.x += WIDTH
        }
        self.p.y = (self.p.y + (time * self.v.y)) % HEIGHT;
        if self.p.y < 0 {
            self.p.y += HEIGHT;
        }
    }
}

fn from(c: &Captures, i: usize) -> i32 {
    c.get(i).unwrap().as_str().parse().unwrap()
}

impl From<String> for Robot {
    fn from(s: String) -> Self {
        let re = Regex::new(r"^p=(\d+),(\d+) v=([-\d]+),([-\d]+)$").unwrap();
        let c = re.captures(&s).unwrap();
        Robot {
            p: Coord::new(from(&c, 1), from(&c, 2)),
            v: Coord::new(from(&c, 3), from(&c, 4)),
        }
    }
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day14.txt");
    lines.into_iter().map(|line| line.into()).collect()
}

pub fn part1(robots: Input) -> u32 {
    let mut q = vec![0, 0, 0, 0];
    for mut robot in robots {
        robot.patrol(100);
        if let Some(quadrant) = robot.quadrant() {
            q[quadrant] += 1;
        }
    }
    q[0] * q[1] * q[2] * q[3]
}

pub fn part2(mut robots: Input) -> usize {
    fn display(robots: &Input, it: usize) -> bool {
        let mut grid = vec!['.'; (WIDTH_SZ + 1) * HEIGHT_SZ];
        (0..HEIGHT).for_each(|y| grid[(y as usize) * (WIDTH_SZ + 1) + WIDTH_SZ] = '\n');
        robots
            .iter()
            .for_each(|r| grid[(r.p.x + r.p.y * (WIDTH + 1)) as usize] = '#');
        let s = grid.into_iter().collect::<String>();
        let found = s.contains("########");
        if found {
            println!("=====[ {} ]=====", it);
            println!("{}", s);
        }
        found
    }
    let mut i = 0;
    loop {
        if display(&robots, i) {
            break;
        }
        i += 1;
        robots.iter_mut().for_each(|r| r.patrol(1));
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 229632480);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 7051);
    }
}

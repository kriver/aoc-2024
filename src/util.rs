use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::str::FromStr;

use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for Coord2D<i32> {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Coord2D::new(0, -1),
            Direction::Right => Coord2D::new(1, 0),
            Direction::Down => Coord2D::new(0, 1),
            Direction::Left => Coord2D::new(-1, 0),
        }
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Debug> Debug for Coord2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T> Coord2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Coord2D { x, y }
    }
}

impl<T: Add<Output = T>> Add for Coord2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Coord2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coord3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Coord3D { x, y, z }
    }
}

pub fn load<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

/**
 * T: coordinate type
 * S: single grid square type
 */
#[derive(Debug, Clone)]
pub struct Grid<T, S> {
    pub width: T,
    pub height: T,
    pub squares: HashMap<Coord2D<T>, S>,
}

impl<T, S> Grid<T, S> {
    pub fn from_file<F>(filename: &str, into_square: F) -> Self
    where
        T: Eq + Hash + From<u8>,
        F: Fn(char, &Coord2D<T>) -> Option<S>,
    {
        let lines = load::<String>(filename);
        let height = lines.len();
        let width = lines[0].len();
        Grid {
            width: (width as u8).try_into().unwrap(),
            height: (height as u8).try_into().unwrap(),
            squares: lines
                .into_iter()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .filter_map(|(x, c)| {
                            // try_into().unwrap() for usize -> T
                            let coord = Coord2D::new(
                                (x as u8).try_into().unwrap(),
                                (y as u8).try_into().unwrap(),
                            );
                            into_square(c, &coord).map(|s| (coord, s))
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect(),
        }
    }
}

pub fn char2num(ascii: char) -> u8 {
    ascii as u8 - '0' as u8
}

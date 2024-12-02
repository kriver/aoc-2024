use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coord2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord2D { x, y }
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

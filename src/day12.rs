use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::util::{Coord2D, Direction, Grid};

type Coord = Coord2D<i32>;

pub struct Plot {
    plant: char,
    visited: bool,
}
type Input = Grid<i32, Plot>;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Perimeter {
    coord: Coord,
    dir: Direction,
}

impl std::fmt::Debug for Perimeter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}",
            self.coord,
            match self.dir {
                Direction::Up => '↑',
                Direction::Right => '→',
                Direction::Down => '↓',
                Direction::Left => '←',
            }
        )
    }
}

impl From<Direction> for Coord {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Coord2D::new(0, -1),
            Direction::Right => Coord2D::new(1, 0),
            Direction::Down => Coord2D::new(0, 1),
            Direction::Left => Coord2D::new(-1, 0),
        }
    }
}

pub fn input() -> Input {
    Grid::from_file("data/day12.txt", |p, _| {
        Some(Plot {
            plant: p,
            visited: false,
        })
    })
}

fn grow(
    grid: &mut Input,
    coord: Coord,
    dir: Direction,
    plant: char,
) -> (usize, HashSet<Perimeter>) {
    let mut perimeter = HashSet::new();
    let plot = grid.squares.get_mut(&coord).unwrap();
    if plot.plant != plant {
        perimeter.insert(Perimeter { coord, dir });
        return (0, perimeter);
    }
    if plot.visited {
        return (0, perimeter);
    }
    plot.visited = true;

    let mut area = 1;
    for dir in Direction::iter() {
        let neighbour = coord + dir.into();
        if grid.squares.contains_key(&neighbour) {
            let (new_area, mut new_perimeter) = grow(grid, neighbour, dir, plant);
            area += new_area;
            new_perimeter.drain().for_each(|p| {
                perimeter.insert(p);
            });
        } else {
            perimeter.insert(Perimeter {
                coord: neighbour,
                dir,
            });
        }
    }
    (area, perimeter)
}

pub fn part1(mut grid: Input) -> usize {
    let mut cost = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let plant = grid.squares.get_mut(&Coord2D::new(x, y)).unwrap().plant;
            let (area, perimeter) = grow(&mut grid, Coord2D::new(x, y), Direction::Up, plant);
            cost += area * perimeter.len();
        }
    }
    cost
}

pub fn part2(mut grid: Input) -> usize {
    fn remove_neighbours(perimeter: &mut HashSet<Perimeter>, p: &Perimeter, d: Coord) -> bool {
        let mut removed_one = false;
        let mut coord = p.coord;
        loop {
            coord = coord + d;
            let p = Perimeter { coord, dir: p.dir };
            if !perimeter.remove(&p) {
                break;
            }
            removed_one = true;
        }
        removed_one
    }
    let mut cost = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let plant = grid.squares.get_mut(&Coord2D::new(x, y)).unwrap().plant;
            let (area, mut perimeter) = grow(&mut grid, Coord2D::new(x, y), Direction::Up, plant);
            let mut sides = 0;
            if area != 0 {
                loop {
                    if perimeter.is_empty() {
                        break;
                    }
                    sides += 1;
                    // take any plot on boundary (it's part of a side)
                    let p = perimeter.iter().next().cloned().unwrap();
                    perimeter.remove(&p);
                    // remove neighbours
                    match p.dir {
                        Direction::Up | Direction::Down => {
                            remove_neighbours(&mut perimeter, &p, Direction::Left.into());
                            remove_neighbours(&mut perimeter, &p, Direction::Right.into());
                        }
                        Direction::Right | Direction::Left => {
                            remove_neighbours(&mut perimeter, &p, Direction::Up.into());
                            remove_neighbours(&mut perimeter, &p, Direction::Down.into());
                        }
                    }
                }
            }
            cost += area * sides;
        }
    }
    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1446042);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 902742);
    }
}

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::util::{load, Coord2D, Direction, Grid};

type Coord = Coord2D<i32>;
type Square = HashMap<Direction, u32>; // minimum cost per direction

#[derive(Eq, PartialEq)]
struct State {
    pos: Coord,
    dir: Direction,
    cost: u32,
}
impl State {
    fn new(pos: Coord, dir: Direction, cost: u32) -> Self {
        State { pos, dir, cost }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub struct Input {
    start: Coord,
    end: Coord,
    maze: Grid<i32, Square>,
}

pub fn input() -> Input {
    let lines: Vec<String> = load("data/day16.txt");
    let (width, height) = (lines[0].len() as i32, lines.len() as i32);
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    let mut squares = HashMap::new();
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord::new(x as i32, y as i32);
            if c != '#' {
                squares.insert(coord, HashMap::new());
            }
            match c {
                'S' => start = coord,
                'E' => end = coord,
                _ => (),
            };
        }
    }
    Input {
        start,
        end,
        maze: Grid {
            width,
            height,
            squares,
        },
    }
}

pub fn part1(mut input: Input) -> u32 {
    fn update(
        square: &mut Square,
        pos: &Coord,
        dir: &Direction,
        cost: u32,
        todo: &mut BinaryHeap<State>,
    ) {
        if square.get(dir).unwrap_or(&u32::MAX) > &cost {
            todo.push(State::new(*pos, *dir, cost));
            square.insert(*dir, cost);
        }
    }
    let mut todo: BinaryHeap<State> = BinaryHeap::new();
    todo.push(State::new(input.start, Direction::Right, 0));
    todo.push(State::new(input.start, Direction::Up, 1000));
    while let Some(State { pos, dir, cost }) = todo.pop() {
        let new_pos = pos + dir.into();
        if let Some(square) = input.maze.squares.get_mut(&new_pos) {
            update(square, &new_pos, &dir, cost + 1, &mut todo);
            update(square, &new_pos, &dir.turn_left(), cost + 1001, &mut todo);
            update(square, &new_pos, &dir.turn_right(), cost + 1001, &mut todo);
        }
    }
    *input
        .maze
        .squares
        .get(&input.end)
        .map(|square| square.values().min().unwrap())
        .unwrap()
}

pub fn part2(mut input: Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 130536);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

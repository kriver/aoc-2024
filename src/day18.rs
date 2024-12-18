use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    u32,
};

use strum::IntoEnumIterator;

use crate::util::{load, Coord2D, Direction};

type Coord = Coord2D<i32>;
type Memory = HashMap<Coord, Option<(u32, Coord)>>;
type Input = Vec<Coord>;

#[derive(Debug, PartialEq, Eq)]
struct State {
    pos: Coord,
    prev: Coord,
    steps: u32,
}
impl State {
    fn new(pos: Coord, prev: Coord, steps: u32) -> Self {
        State { pos, prev, steps }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

pub fn input() -> Input {
    let values: Vec<String> = load("data/day18.txt");
    values
        .into_iter()
        .map(|line| {
            let tokens = line.split(",").collect::<Vec<_>>();
            Coord::new(
                tokens[0].parse::<i32>().unwrap(),
                tokens[1].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn find_path(corruption: &Input, num: usize) -> Option<u32> {
    let end = Coord::new(70, 70);
    let mut memory: Memory = HashMap::new();
    for y in 0..=end.y {
        for x in 0..=end.x {
            memory.insert(Coord::new(x, y), None);
        }
    }
    corruption.into_iter().take(num).for_each(|c| {
        memory.remove(&c);
    });
    let mut todo: BinaryHeap<State> = BinaryHeap::new();
    todo.push(State::new(Coord::new(0, 0), Coord::new(-1, -1), 0));
    loop {
        if todo.is_empty() {
            break None;
        }
        let State { pos, prev, steps } = todo.pop().unwrap();
        if pos == end {
            break Some(steps);
        }
        if let Some(state) = memory.get(&pos) {
            let ok = match state {
                None => true,
                Some((s, _)) => *s > steps,
            };
            if ok {
                memory.insert(pos, Some((steps, prev)));
                for dir in Direction::iter() {
                    todo.push(State::new(pos + dir.into(), pos, steps + 1));
                }
            }
        }
    }
}

pub fn part1(corruption: Input) -> u32 {
    find_path(&corruption, 1024).unwrap()
}

pub fn part2(corruption: Input) -> Coord {
    // brute forcing it :-/
    for num in 1025..corruption.len() {
        if let None = find_path(&corruption, num) {
            return corruption[num - 1];
        }
    }
    unreachable!("not found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 276);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), Coord::new(60, 37));
    }
}

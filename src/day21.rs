use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::util::{load, Coord2D};

type Input = Vec<String>;
type Pad = HashMap<char, Coord>;
type Coord = Coord2D<i32>;

// numpad
//   +---+---+---+
//   | 7 | 8 | 9 |
//   +---+---+---+
//   | 4 | 5 | 6 |
//   +---+---+---+
//   | 1 | 2 | 3 |
//   +---+---+---+
//       | 0 | A |
//       +---+---+
lazy_static! {
    static ref NUMPAD: HashMap<char, Coord> = {
        let mut np = HashMap::new();
        np.insert('7', Coord::new(0, 0));
        np.insert('8', Coord::new(1, 0));
        np.insert('9', Coord::new(2, 0));
        np.insert('4', Coord::new(0, 1));
        np.insert('5', Coord::new(1, 1));
        np.insert('6', Coord::new(2, 1));
        np.insert('1', Coord::new(0, 2));
        np.insert('2', Coord::new(1, 2));
        np.insert('3', Coord::new(2, 2));
        np.insert('0', Coord::new(1, 3));
        np.insert('A', Coord::new(2, 3));
        np
    };
}
// dirpad
//       +---+---+
//       | ^ | A |
//   +---+---+---+
//   | < | v | > |
//   +---+---+---+
lazy_static! {
    static ref DIRPAD: HashMap<char, Coord> = {
        let mut np = HashMap::new();
        np.insert('^', Coord::new(1, 0));
        np.insert('A', Coord::new(2, 0));
        np.insert('<', Coord::new(0, 1));
        np.insert('v', Coord::new(1, 1));
        np.insert('>', Coord::new(2, 1));
        np
    };
}

pub fn input() -> Input {
    let values: Vec<String> = load("data/day21.txt");
    values
}

fn dirpad(strokes: Vec<char>, levels: usize) -> Vec<char> {
    if levels == 0 {
        strokes
    } else {
        let mut new_strokes = Vec::new();
        let mut pos = DIRPAD[&'A'];
        for s in strokes {
            let new_pos = DIRPAD[&s];
            if pos != new_pos {
                let d = new_pos - pos;
                (0..d.y).for_each(|_| new_strokes.push('v'));
                (d.x..0).for_each(|_| new_strokes.push('<'));
                (0..d.x).for_each(|_| new_strokes.push('>'));
                (d.y..0).for_each(|_| new_strokes.push('^'));
            }
            new_strokes.push('A');
            pos = new_pos
        }
        dirpad(new_strokes, levels - 1)
    }
}

fn numpad(from: char, to: char, levels: usize) -> Vec<char> {
    let fp = NUMPAD[&from];
    let tp = NUMPAD[&to];
    let d = tp - fp;
    if fp.y == 3 && tp.x == 0 {
        let mut strokes = Vec::new();
        (d.y..0).for_each(|_| strokes.push('^'));
        (d.x..0).for_each(|_| strokes.push('<'));
        strokes.push('A');
        dirpad(strokes, levels)
    } else if fp.x == 0 && tp.y == 3 {
        let mut strokes = Vec::new();
        (0..d.x).for_each(|_| strokes.push('>'));
        (0..d.y).for_each(|_| strokes.push('v'));
        strokes.push('A');
        dirpad(strokes, levels)
    } else {
        let mut strokes1 = Vec::new();
        (d.y..0).for_each(|_| strokes1.push('^'));
        (0..d.y).for_each(|_| strokes1.push('v'));
        (d.x..0).for_each(|_| strokes1.push('<'));
        (0..d.x).for_each(|_| strokes1.push('>'));
        strokes1.push('A');
        let res1 = dirpad(strokes1, levels);
        if fp.x == tp.x || fp.y == tp.y {
            res1
        } else {
            let mut strokes2 = Vec::new();
            (d.x..0).for_each(|_| strokes2.push('<'));
            (0..d.x).for_each(|_| strokes2.push('>'));
            (d.y..0).for_each(|_| strokes2.push('^'));
            (0..d.y).for_each(|_| strokes2.push('v'));
            strokes2.push('A');
            let res2 = dirpad(strokes2, levels);
            if res1.len() < res2.len() {
                res1
            } else {
                res2
            }
        }
    }
}

fn encode(mut code: String, levels: usize) -> usize {
    code = "A".to_owned() + &code;
    code[0..code.len() - 1]
        .chars()
        .zip(code[1..].chars())
        .map(|(from, to)| numpad(from, to, levels))
        .map(|strokes| strokes.len())
        .sum()
}

pub fn part1(codes: Input) -> usize {
    codes
        .into_iter()
        .map(|code| {
            let num = code[0..3].parse::<usize>().unwrap();
            let len = encode(code, 2);
            num * len
        })
        .sum()
}

pub fn part2(codes: Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 94284);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

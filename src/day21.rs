use lazy_static::lazy_static;
use log::debug;
use std::{collections::HashMap, sync::Mutex};

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
    static ref NUMPAD: Pad = {
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
    static ref DIRPAD: Pad = {
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

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
    static ref LENGTHS: Mutex<HashMap<(usize, String), usize>> = Mutex::new(HashMap::new());
}

fn dirpad(stroke_chars: Vec<char>, levels: usize) -> usize {
    fn key_to_strokes(pos: Coord, c: char) -> (Coord, String) {
        let mut strokes = Vec::new();
        let new_pos = DIRPAD[&c];
        // It seems moving between 'A' and 'v' is more efficient after two levels
        if pos == Coord::new(2, 0) && c == 'v' {
            return (new_pos, "<vA".to_owned());
        }
        if pos == Coord::new(1, 1) && c == 'A' {
            return (new_pos, "^>A".to_owned());
        }
        if pos != new_pos {
            let d = new_pos - pos;
            (0..d.y).for_each(|_| strokes.push('v'));
            (d.x..0).for_each(|_| strokes.push('<'));
            (0..d.x).for_each(|_| strokes.push('>'));
            (d.y..0).for_each(|_| strokes.push('^'));
        }
        strokes.push('A');
        (new_pos, strokes.into_iter().collect())
    }
    fn strokes_to_vec(strokes: &String) -> Vec<String> {
        let mut new_strokes = Vec::new();
        let mut pos = DIRPAD[&'A'];
        for c in strokes.chars() {
            let (p, s) = key_to_strokes(pos, c);
            new_strokes.push(s);
            pos = p;
        }
        new_strokes
    }
    let strokes: String = stroke_chars.into_iter().collect();
    let mut queue = vec![(levels, strokes)];
    let mut total_length = 0;
    while !queue.is_empty() {
        let (level, s) = queue.pop().unwrap();
        debug!("[{:02}] Popped {}", level, s);
        if level == 0 {
            debug!(
                "[{:02}] \tAdding length {} for {} on level {}",
                level,
                s.len(),
                s,
                0
            );
            LENGTHS.lock().unwrap().insert((0, s.clone()), s.len());
            total_length += s.len();
            debug!(
                "[{:02}] \tTotal length incremented to = {}",
                level, total_length
            );
        } else {
            let mut cache = CACHE.lock().unwrap();
            let mut lengths = LENGTHS.lock().unwrap();
            let v = match cache.get(&s) {
                Some(v) => v.clone(),
                None => {
                    let new_v = strokes_to_vec(&s);
                    cache.insert(s.clone(), new_v.clone());
                    new_v
                }
            };
            let cached: Vec<_> = v
                .iter()
                .map(|e| lengths.get(&(level - 1, e.to_string())))
                .collect();
            debug!(
                "[{:02}] \tChecking lengths {:?} for level {} -> {:?}",
                level,
                v,
                level - 1,
                cached
            );
            if cached.iter().any(|e| e.is_none()) {
                v.iter()
                    .inspect(|e| debug!("[{:02}] \tQueuing {} for level {}", level, e, level - 1))
                    .for_each(|e| queue.push((level - 1, e.clone())));
            } else {
                debug!(
                    "[{:02}] \tFound in cache {:?} for level {}",
                    level,
                    v,
                    level - 1
                );
                let sum = cached.iter().map(|e| e.unwrap()).sum::<usize>();
                debug!(
                    "[{:02}] \tAdding length {} for {} on level {}",
                    level, sum, s, level
                );
                lengths.insert((level, s.clone()), sum);
                total_length += sum;
                debug!(
                    "[{:02}] \tTotal length incremented to = {}",
                    level, total_length
                );
            }
        }
    }
    debug!("[{:02}] \tTotal length = {}", levels, total_length,);
    total_length
}

fn numpad(from: char, to: char, levels: usize) -> usize {
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
            if res1 < res2 {
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
        .inspect(|e| debug!("### {} -> {}", code, e))
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

pub fn part2(codes: Input) -> usize {
    env_logger::init();
    codes
        .into_iter()
        .map(|code| {
            let num = code[0..3].parse::<usize>().unwrap();
            let len = encode(code, 25);
            debug!("Code -> {} * {} = {}", num, len, num * len);
            num * len
        })
        .sum()
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
        assert_eq!(part2(input()), 116821732384052);
    }
}

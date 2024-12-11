use std::cmp::min;

use crate::util::{char2num, load};

type Input = Vec<u8>;

pub fn input() -> Input {
    let values: Vec<String> = load("data/day09.txt");
    values[0].chars().map(|c| char2num(c)).collect()
}

pub fn part1(values: Input) -> usize {
    fn add_file_blocks(block: &mut usize, file_id: usize, num_blocks: usize) -> usize {
        let mut checksum = 0;
        for _ in 0..num_blocks {
            checksum += *block * file_id;
            *block += 1;
        }
        checksum
    }
    let mut checksum = 0;
    let mut block = 0;
    let (mut forward, mut backward) = (0, values.len() - 1);
    let (mut forward_id, mut backward_id) = (0, values.len() / 2);
    let mut backward_cnt = 0;
    let mut gap_sz = 0;
    let mut is_gap = false;
    loop {
        let (id, cnt, max_sz) = if is_gap {
            if gap_sz == 0 {
                gap_sz = values[forward] as usize;
            }
            if backward_cnt == 0 {
                backward_cnt = values[backward] as usize;
                backward -= 2; // skip free space when moving backwards
            }
            (backward_id, backward_cnt, gap_sz)
        } else {
            let file_sz = values[forward] as usize;
            let fill = (forward_id, file_sz, file_sz);
            forward_id += 1;
            forward += 1;
            fill
        };
        let sz = min(max_sz, cnt);
        checksum += add_file_blocks(&mut block, id, sz);
        if is_gap {
            backward_cnt -= sz;
            if backward_cnt == 0 {
                backward_id -= 1;
            }
            gap_sz -= sz;
            if gap_sz == 0 {
                forward += 1;
                is_gap = false;
            }
        } else {
            is_gap = true;
        }
        if forward > backward {
            // fill remainder
            if backward_cnt > 0 {
                checksum += add_file_blocks(&mut block, backward_id, backward_cnt);
            }
            break;
        }
    }
    checksum
}

pub fn part2(values: Input) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 6344673854800);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}

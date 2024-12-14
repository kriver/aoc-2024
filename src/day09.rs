use std::{cmp::min, collections::HashMap, usize};

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

pub fn part2(sizes: Input) -> usize {
    // mapping gap sizes to positions
    let mut gaps: HashMap<usize, Vec<usize>> = HashMap::new();
    // file positions with size and id
    let mut files: Vec<(usize, usize, usize)> = vec![];
    let mut pos = 0;
    let mut file_id = 0;
    for (i, sz) in sizes.iter().enumerate() {
        if i % 2 == 0 {
            // a file
            files.push((pos, *sz as usize, file_id));
            file_id += 1;
        } else {
            // a gap
            gaps.entry(*sz as usize)
                .and_modify(|v| v.push(pos))
                .or_insert(vec![pos]);
        }
        pos += *sz as usize;
    }
    // println!("Files {:?}", files);
    // println!("Gaps {:?}", gaps);

    for f in (0..files.len()).rev() {
        let (f_pos, f_sz, _) = files[f];
        // find first gap of matching size
        let mut first_pos = usize::MAX;
        let mut first_sz = 0;
        for g_sz in f_sz..=9 {
            if let Some(g) = gaps.get(&g_sz) {
                if g[0] < first_pos && g[0] < f_pos {
                    first_pos = g[0];
                    first_sz = g_sz;
                }
            }
        }
        if first_sz == 0 {
            continue; // nothing found
        }
        // move file
        let g = gaps.get_mut(&first_sz).unwrap();
        let g_pos = g.remove(0);
        if g.is_empty() {
            gaps.remove(&first_sz);
        }
        files[f].0 = g_pos; // set new position
        if first_sz > f_sz {
            let new_g_sz = first_sz - f_sz;
            let new_g_pos = g_pos + f_sz;
            gaps.entry(new_g_sz)
                .and_modify(|v| {
                    if let Err(idx) = v.binary_search(&new_g_pos) {
                        v.insert(idx, new_g_pos);
                    } else {
                        unreachable!("should not happen")
                    }
                })
                .or_insert(vec![new_g_pos]);
        }
    }
    files
        .into_iter()
        .map(|(pos, sz, id)| (pos..(pos + sz)).map(|p| p * id).sum::<usize>())
        .sum()
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
        assert_eq!(part2(input()), 6360363199987);
    }
}

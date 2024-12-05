use std::collections::{HashMap, HashSet};

use crate::util::load;

type Rules = HashMap<u32, HashSet<u32>>; // page to page
type Update = HashMap<u32, usize>; // page to index
type Updates = Vec<Update>;
type Input = (Rules, Updates);

pub fn input() -> Input {
    let mut rules: Rules = HashMap::new();
    let mut updates: Updates = vec![];
    let lines: Vec<String> = load("data/day05.txt");
    let mut parsing_rules = true;
    for line in lines {
        if line.len() == 0 {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let pages: Vec<u32> = line.split("|").map(|v| v.parse().unwrap()).collect();
            rules
                .entry(pages[0])
                .and_modify(|s| {
                    s.insert(pages[1]);
                })
                .or_insert({
                    let mut s = HashSet::new();
                    s.insert(pages[1]);
                    s
                });
        } else {
            updates.push(
                line.split(",")
                    .map(|v| v.parse().unwrap())
                    .enumerate()
                    .map(|(i, v)| (v, i))
                    .collect(),
            );
        }
    }
    (rules, updates)
}

fn is_valid(rules: &Rules, update: &Update) -> bool {
    for page in update.keys() {
        let index = update.get(page).unwrap();
        if let Some(before) = rules.get(&page) {
            for b in before {
                if let Some(other) = update.get(b) {
                    if index >= other {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn get_middle(update: &Update) -> u32 {
    let i = (update.len() - 1) / 2;
    for (page, j) in update {
        if i == *j {
            return *page;
        }
    }
    panic!("middle not found");
}

pub fn part1((rules, updates): Input) -> u32 {
    let mut result = 0;
    for update in updates {
        if is_valid(&rules, &update) {
            result += get_middle(&update);
        }
    }
    result
}

fn get_ordered_middle(update: Update, rules: &Rules) -> u32 {
    let mut ordered = vec![];
    for page in update.keys() {
        let mut inserted = false;
        let before = rules.get(page).unwrap();
        for index in 0..ordered.len() {
            if before.contains(ordered[index]) {
                ordered.insert(index, page);
                inserted = true;
                break;
            }
        }
        if !inserted {
            ordered.push(page);
        }
    }
    *ordered[(ordered.len() - 1) / 2]
}

pub fn part2((rules, updates): Input) -> u32 {
    let mut result = 0;
    for update in updates {
        if !is_valid(&rules, &update) {
            result += get_ordered_middle(update, &rules);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 5639);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 5273);
    }
}

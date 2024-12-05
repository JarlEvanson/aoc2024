use crate::Solution;

pub const SOLUTION: Solution = Solution {
    part_1,
    part_2,
    both,
};

#[inline]
pub fn part_1(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();

            (
                before.parse::<usize>().unwrap(),
                after.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    updates
        .lines()
        .filter_map(|line| {
            let update = line
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            for (before, after) in rules.iter().copied() {
                let Some(before_pos) = update.iter().position(|&val| val == before) else {
                    continue;
                };
                let Some(after_pos) = update.iter().position(|&val| val == after) else {
                    continue;
                };

                if before_pos > after_pos {
                    return None;
                }
            }

            Some(update[update.len() / 2])
        })
        .sum()
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();

            (
                before.parse::<usize>().unwrap(),
                after.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    updates
        .lines()
        .filter_map(|line| {
            let mut update = line
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut correct = true;
            loop {
                let mut changed = false;

                for (before, after) in rules.iter().copied() {
                    let Some(before_pos) = update.iter().position(|&val| val == before) else {
                        continue;
                    };
                    let Some(after_pos) = update.iter().position(|&val| val == after) else {
                        continue;
                    };

                    if before_pos > after_pos {
                        update.swap(before_pos, after_pos);
                        correct = false;
                        changed = true;
                    }
                }

                if !changed {
                    break;
                }
            }
            if correct {
                None
            } else {
                Some(update[update.len() / 2])
            }
        })
        .sum()
}

pub fn both(input: &str) -> (usize, usize) {
    (part_1(input), part_2(input))
}

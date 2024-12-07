use crate::Solution;

pub const SOLUTION: Solution = Solution {
    part_1,
    part_2,
    both,
};

#[inline]
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (target, remainder) = line.split_once(": ").unwrap();

            let target = target.parse::<usize>().unwrap();

            let numbers = remainder
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            part_1_step(target, numbers[0], &numbers[1..]) as usize * target
        })
        .sum()
}

fn part_1_step(target: usize, left: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 || left > target {
        return left == target;
    }
    part_1_step(target, left * remaining[0], &remaining[1..])
        || part_1_step(target, left + remaining[0], &remaining[1..])
}

#[inline]
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (target, remainder) = line.split_once(": ").unwrap();

            let target = target.parse::<usize>().unwrap();

            let numbers = remainder
                .split_ascii_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            part_2_step(target, numbers[0], &numbers[1..]) as usize * target
        })
        .sum()
}

fn part_2_step(target: usize, left: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 || left > target {
        return left == target;
    }

    part_2_step(target, left * remaining[0], &remaining[1..])
        || part_2_step(target, concat(left, remaining[0]), &remaining[1..])
        || part_2_step(target, left + remaining[0], &remaining[1..])
}

fn concat(left: usize, val: usize) -> usize {
    left * (10usize.pow(format!("{val}").len() as u32)) + val
}

pub fn both(input: &str) -> (usize, usize) {
    (part_1(input), part_2(input))
}

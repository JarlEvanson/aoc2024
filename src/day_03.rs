use regex::Regex;

use crate::Solution;

pub const SOLUTION: Solution = Solution {
    part_1,
    part_2,
    both,
};

pub fn part_1(input: &str) -> usize {
    input
        .split("mul(")
        .filter_map(|potential| {
            let Some((potential, _)) = potential.split_once(')') else {
                return None;
            };

            let Some((num_1, num_2)) = potential.split_once(',') else {
                return None;
            };

            let Some(num_1) = num_1.parse::<usize>().ok() else {
                return None;
            };

            let Some(num_2) = num_2.parse::<usize>().ok() else {
                return None;
            };

            Some(num_1 * num_2)
        })
        .sum::<usize>()
}

pub fn part_2(input: &str) -> usize {
    let mut input = input;
    let regex = Regex::new("do\\(\\)|don't\\(\\)|mul\\([0-9]+,[0-9]+\\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    while let Some(find) = regex.find(input) {
        match find.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            value if enabled => {
                let Some((_, potential)) = value.split_once("mul(") else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some((potential, _)) = potential.split_once(')') else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some((num_1, num_2)) = potential.split_once(',') else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some(num_1) = num_1.parse::<usize>().ok() else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some(num_2) = num_2.parse::<usize>().ok() else {
                    input = &input[find.end()..];
                    continue;
                };

                sum += num_1 * num_2;
            }
            _ => {}
        }
        input = &input[find.end()..];
    }

    sum
}

pub fn both(input: &str) -> (usize, usize) {
    let mut input = input;
    let regex = Regex::new("do\\(\\)|don't\\(\\)|mul\\([0-9]+,[0-9]+\\)").unwrap();

    let (mut part_1, mut part_2) = (0, 0);
    let mut enabled = true;
    while let Some(find) = regex.find(input) {
        match find.as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            value => {
                let Some((_, potential)) = value.split_once("mul(") else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some((potential, _)) = potential.split_once(')') else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some((num_1, num_2)) = potential.split_once(',') else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some(num_1) = num_1.parse::<usize>().ok() else {
                    input = &input[find.end()..];
                    continue;
                };

                let Some(num_2) = num_2.parse::<usize>().ok() else {
                    input = &input[find.end()..];
                    continue;
                };

                part_1 += num_1 * num_2;
                if enabled {
                    part_2 += num_1 * num_2;
                }
            }
        }
        input = &input[find.end()..];
    }

    (part_1, part_2)
}

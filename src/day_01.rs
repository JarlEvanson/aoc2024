use crate::Solution;

pub const SOLUTION: Solution = Solution {
    part_1,
    part_2,
    both,
};

pub fn part_1(input: &str) -> usize {
    let (list_1, list_2) = process_input(input);

    part_1_impl(&list_1, &list_2)
}

pub fn part_2(input: &str) -> usize {
    let (list_1, list_2) = process_input(input);

    part_2_impl(&list_1, &list_2)
}

pub fn both(input: &str) -> (usize, usize) {
    let (list_1, list_2) = process_input(input);

    (part_1_impl(&list_1, &list_2), part_2_impl(&list_1, &list_2))
}

pub fn process_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    let lines = input.lines();
    let locations = lines.map(|line| {
        let mut numbers = line.split_ascii_whitespace();

        (
            numbers.next().unwrap().parse::<isize>().unwrap(),
            numbers.next().unwrap().parse::<isize>().unwrap(),
        )
    });

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    for (location_1, location_2) in locations {
        list_1.push(location_1);
        list_2.push(location_2);
    }

    list_1.sort_unstable();
    list_2.sort_unstable();

    (list_1, list_2)
}

pub fn part_1_impl(list_1: &[isize], list_2: &[isize]) -> usize {
    list_1
        .into_iter()
        .zip(list_2.into_iter())
        .map(|(id_1, id_2)| (id_1 - id_2).abs())
        .sum::<isize>() as usize
}

pub fn part_2_impl(list_1: &[isize], list_2: &[isize]) -> usize {
    let mut list_2 = list_2.into_iter().copied().peekable();

    let mut last = -1;
    let mut count = 0;

    let mut sum = 0;
    for id_1 in list_1.into_iter().copied() {
        if id_1 != last {
            count = 0;

            while let Some(&id_2) = list_2.peek() {
                if id_2 > id_1 {
                    break;
                }

                if id_2 == id_1 {
                    count += 1;
                }

                let _ = list_2.next();
            }
        }

        last = id_1;
        sum += id_1 * count;
    }

    sum as usize
}

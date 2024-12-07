use fxhash::FxHashSet;

use crate::{
    Solution,
    utils::grid::{CardinalDirection, Grid},
};

pub const SOLUTION: Solution = Solution {
    part_1,
    part_2,
    both,
};

#[inline]
pub fn part_1(input: &str) -> usize {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .into_iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let mut grid = Grid::new(data, width, height);

    let mut guard_pos = 'find_guard: {
        for (y, row) in (0..grid.height)
            .map(|row| grid.row(row).unwrap())
            .enumerate()
        {
            for (x, &byte) in row.iter().enumerate() {
                if byte == b'^' {
                    break 'find_guard (x as isize, y as isize);
                }
            }
        }

        unreachable!()
    };
    let mut direction = CardinalDirection::North;

    let mut count = 0;
    loop {
        let value = grid.get_mut_signed(guard_pos.0, guard_pos.1).unwrap();
        count += (*value != b'&') as usize;
        *value = b'&';

        let Some((new_pos, new_direction)) = guard_step(&grid, guard_pos, direction) else {
            break;
        };

        guard_pos = new_pos;
        direction = new_direction;
    }

    count
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .into_iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let start_pos = 'find_guard: {
        for (y, row) in (0..grid.height)
            .map(|row| grid.row(row).unwrap())
            .enumerate()
        {
            for (x, &byte) in row.iter().enumerate() {
                if byte == b'^' {
                    break 'find_guard (x as isize, y as isize);
                }
            }
        }

        unreachable!()
    };
    let start_direction = CardinalDirection::North;

    let mut count = 0;
    let mut visited = FxHashSet::default();
    for (y, row) in (0..grid.height)
        .map(|row| grid.row(row).unwrap())
        .enumerate()
    {
        'l: for (x, &byte) in row.iter().enumerate() {
            if byte == b'#' {
                continue;
            }

            visited.clear();

            let mut grid = grid.clone();
            *grid.get_mut(x, y).unwrap() = b'#';

            let mut current_pos = start_pos;
            let mut current_dir = start_direction;
            while visited.insert((current_pos, current_dir)) {
                let Some((new_pos, new_direction)) = guard_step(&grid, current_pos, current_dir)
                else {
                    continue 'l;
                };

                current_pos = new_pos;
                current_dir = new_direction;
            }

            count += 1;
        }
    }

    count
}

pub fn guard_step(
    grid: &Grid<u8>,
    position: (isize, isize),
    direction: CardinalDirection,
) -> Option<((isize, isize), CardinalDirection)> {
    let test_pos = (
        position.0 + direction.offset().0,
        position.1 + direction.offset().1,
    );

    let Some(test_value) = grid.get_signed(test_pos.0, test_pos.1) else {
        return None;
    };

    if *test_value == b'#' {
        Some((position, direction.clockwise()))
    } else {
        Some((test_pos, direction))
    }
}

pub fn both(input: &str) -> (usize, usize) {
    (part_1(input), part_2(input))
}

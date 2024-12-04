use crate::{Solution, utils::grid::Grid};

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
        .flat_map(|line| line.into_iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .into_iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let mut count = 0;

    let string = [&b'X', &b'M', &b'A', &b'S'];
    let string_2 = [b'X', b'M', b'A', b'S'];
    for column in (0..grid.width).map(|column| grid.column(column).unwrap()) {
        let column = column.iter().copied().collect::<Vec<_>>();

        count += column
            .iter()
            .map_windows::<_, bool, 4>(|bytes| bytes.eq(&string))
            .filter(|val| *val)
            .count();

        count += column
            .iter()
            .rev()
            .map_windows::<_, bool, 4>(|bytes| bytes.eq(&string))
            .filter(|val| *val)
            .count();
    }

    for row in (0..grid.height).map(|row| grid.row(row).unwrap()) {
        let row = row.iter().copied().collect::<Vec<_>>();

        count += row
            .iter()
            .map_windows::<_, bool, 4>(|bytes| bytes.eq(&string))
            .filter(|val| *val)
            .count();

        count += row
            .iter()
            .rev()
            .map_windows::<_, bool, 4>(|bytes| bytes.eq(&string))
            .filter(|val| *val)
            .count();
    }

    for row in 0..grid.height {
        for column in 0..grid.width {
            let Some(mut bytes) = get_down_right(&grid, row, column) else {
                continue;
            };

            if bytes.eq(&string_2) {
                count += 1;
            }

            bytes.reverse();

            if bytes.eq(&string_2) {
                count += 1;
            }
        }

        for column in 0..grid.width {
            let Some(mut bytes) = get_down_left(&grid, row, column) else {
                continue;
            };

            if bytes.eq(&string_2) {
                count += 1;
            }

            bytes.reverse();

            if bytes.eq(&string_2) {
                count += 1;
            }
        }
    }

    count
}

fn get_down_right(grid: &Grid<u8>, x: usize, y: usize) -> Option<[u8; 4]> {
    Some([
        grid.get(x, y).copied()?,
        grid.get(x.checked_add(1)?, y.checked_add(1)?).copied()?,
        grid.get(x.checked_add(2)?, y.checked_add(2)?).copied()?,
        grid.get(x.checked_add(3)?, y.checked_add(3)?).copied()?,
    ])
}

fn get_down_left(grid: &Grid<u8>, x: usize, y: usize) -> Option<[u8; 4]> {
    Some([
        grid.get(x, y).copied()?,
        grid.get(x.checked_sub(1)?, y.checked_add(1)?).copied()?,
        grid.get(x.checked_sub(2)?, y.checked_add(2)?).copied()?,
        grid.get(x.checked_sub(3)?, y.checked_add(3)?).copied()?,
    ])
}

#[inline]
pub fn part_2(input: &str) -> usize {
    let data = input
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .flat_map(|line| line.into_iter().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let width = input
        .as_bytes()
        .into_iter()
        .position(|&byte| byte == b'\n')
        .unwrap();
    let height = data.len() / width;
    let grid = Grid::new(data, width, height);

    let mas = [b'M', b'A', b'S'];

    let mut count = 0;
    for row in 0..grid.height {
        for column in 0..grid.width {
            let Some(mut first) = get_down_right_2(&grid, row, column) else {
                continue;
            };

            let Some(mut second) = get_down_left_2(&grid, row, column) else {
                continue;
            };

            let mut first_valid = first.eq(&mas);
            first.reverse();
            first_valid = first_valid || first.eq(&mas);

            let mut second_valid = second.eq(&mas);
            second.reverse();
            second_valid = second_valid || second.eq(&mas);

            count += (first_valid && second_valid) as usize;
        }
    }

    count
}

fn get_down_right_2(grid: &Grid<u8>, x: usize, y: usize) -> Option<[u8; 3]> {
    Some([
        grid.get(x, y).copied()?,
        grid.get(x.checked_add(1)?, y.checked_add(1)?).copied()?,
        grid.get(x.checked_add(2)?, y.checked_add(2)?).copied()?,
    ])
}

fn get_down_left_2(grid: &Grid<u8>, x: usize, y: usize) -> Option<[u8; 3]> {
    Some([
        grid.get(x.checked_add(2)?, y).copied()?,
        grid.get(x.checked_add(1)?, y.checked_add(1)?).copied()?,
        grid.get(x, y.checked_add(2)?).copied()?,
    ])
}

pub fn both(input: &str) -> (usize, usize) {
    (part_1(input), part_2(input))
}

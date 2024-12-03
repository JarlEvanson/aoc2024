use std::{path::PathBuf, time::Duration};

pub mod day_01;
pub mod day_02;

pub const SOLUTIONS: [Solution; 2] = [day_01::SOLUTION, day_02::SOLUTION];

pub fn main() {
    let Some(mut folder) = std::env::args().nth(1).map(PathBuf::from) else {
        println!("a folder must be specified to load inputs from");
        println!("each day's input file must be of the form `day_[day].txt");
        return;
    };

    folder.push("day_00.txt");

    let mut total_time = Duration::new(0, 0);
    for (day, solution) in SOLUTIONS.into_iter().enumerate() {
        let input_filename = format!("day_{:02}.txt", day + 1);
        folder.set_file_name(input_filename);
        let Some(input) = std::fs::read_to_string(&folder).ok() else {
            println!("Skipping day {:02}", day + 1);
            continue;
        };

        println!("Day {:02}:", day + 1);

        let (part_1_elapsed, part_1) = time(&input, solution.part_1);
        println!("\tPart 1: {part_1} ({part_1_elapsed:?})");

        let (part_2_elapsed, part_2) = time(&input, solution.part_2);
        println!("\tPart 2: {part_2} ({part_2_elapsed:?})");

        let (both_elapsed, (part_1, part_2)) = time(&input, solution.both);
        println!("\tBoth: ({part_1}, {part_2}): ({both_elapsed:?})\n");

        total_time += both_elapsed;
    }

    println!("Total Processing Time: {:?}", total_time);
}

pub fn time<T>(input: &str, func: fn(&str) -> T) -> (Duration, T) {
    let start = std::time::Instant::now();

    let result = func(input);

    let end = std::time::Instant::now();
    let elapsed = end.duration_since(start);

    (elapsed, result)
}

pub struct Solution {
    pub part_1: fn(&str) -> usize,
    pub part_2: fn(&str) -> usize,
    pub both: fn(&str) -> (usize, usize),
}

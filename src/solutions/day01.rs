use itertools::Itertools;

use super::prelude::*;

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<u16>> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);
    let data: Vec<_> = reader.lines().map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();
    Ok(data)
}

fn part1(input_path: &PathBuf) -> crate::Result<String> {
    Ok(read_input(input_path)?
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum::<u16>()
        .to_string())
}

fn part2(input_path: &PathBuf) -> crate::Result<String> {
    Ok(read_input(input_path)?
        .windows(3)
        .map(|slice| slice.iter().sum::<u16>())
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum::<u16>()
        .to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(&"data/day01_input.txt".into()));
    runner.add_fn("part2", || part2(&"data/day01_input.txt".into()));
    runner
}

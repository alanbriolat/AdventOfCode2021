use std::cmp;

use num::Integer;

use super::prelude::*;
use crate::util::{parse_delimited, read_file, read_line};

fn triangular_number(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut crabs: Vec<i64> = parse_delimited(read_line(reader).as_str(), ",").collect();
    // Find median
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    // Sum distances to the median
    let fuel: u64 = crabs
        .iter()
        .copied()
        .map(|crab| (crab - median).abs() as u64)
        .sum();
    Ok(fuel.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let crabs: Vec<i64> = parse_delimited(read_line(reader).as_str(), ",").collect();
    let sum: i64 = crabs.iter().sum();
    let mean_floor: i64 = sum.div_floor(&(crabs.len() as i64));
    let mean_ceil: i64 = sum.div_ceil(&(crabs.len() as i64));
    let fuel_floor: u64 = crabs
        .iter()
        .copied()
        .map(|crab| triangular_number((crab - mean_floor).abs() as u64))
        .sum();
    let fuel_ceil: u64 = crabs
        .iter()
        .copied()
        .map(|crab| triangular_number((crab - mean_ceil).abs() as u64))
        .sum();
    let fuel = cmp::min(fuel_floor, fuel_ceil);
    Ok(fuel.to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day07_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day07_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::util::read_str;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(read_str(indoc! {"\
                16,1,2,0,4,2,7,1,2,14
            "}))
            .unwrap(),
            "37"
        );
        assert_eq!(part1(read_file("data/day07_input.txt")).unwrap(), "337488");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                16,1,2,0,4,2,7,1,2,14
            "}))
            .unwrap(),
            "168"
        );
        assert_eq!(
            part2(read_file("data/day07_input.txt")).unwrap(),
            "89647695"
        );
    }
}

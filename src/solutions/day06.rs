use std::collections::HashMap;

use super::prelude::*;
use crate::util::read_file;

const SPAWN_INTERVAL: u64 = 7;
const SPAWN_DELAY: u64 = 9;

fn read_input(line: &str) -> impl Iterator<Item = u64> + '_ {
    line.split(',').map(|x| x.parse::<u64>().unwrap())
}

fn run<I: Iterator<Item = u64>>(input: I, days: u64) -> u64 {
    let mut memo: HashMap<u64, u64> = HashMap::new();

    fn count(initial: u64, end: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if let Some(result) = memo.get(&initial).copied() {
            result
        } else {
            let result = (initial..end)
                // Get each time this lanternfish will spawn before the end
                .step_by(SPAWN_INTERVAL as usize)
                // Get the descendents of each of those, recursively
                .map(|spawn_at| count(spawn_at + SPAWN_DELAY, end, memo))
                .sum::<u64>()
                // Include self
                + 1;
            memo.insert(initial, result);
            result
        }
    }

    input.map(|t| count(t, days, &mut memo)).sum()
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let total = run(
        read_input(reader.lines().next().unwrap().unwrap().as_str()),
        80,
    );
    Ok(total.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let total = run(
        read_input(reader.lines().next().unwrap().unwrap().as_str()),
        256,
    );
    Ok(total.to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day06_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day06_input.txt")));
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
                3,4,3,1,2
            "}))
            .unwrap(),
            "5934"
        );
        assert_eq!(part1(read_file("data/day06_input.txt")).unwrap(), "394994");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                3,4,3,1,2
            "}))
            .unwrap(),
            "26984457539"
        );
        assert_eq!(
            part2(read_file("data/day06_input.txt")).unwrap(),
            "1765974267455"
        );
    }
}

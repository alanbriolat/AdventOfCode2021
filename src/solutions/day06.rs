use super::prelude::*;
use crate::util::{parse_lines, read_file};

const SPAWN_INTERVAL: usize = 7;
const SPAWN_DELAY: u64 = 9;

fn recursive_count(initial_timer: u64, end: u64) -> u64 {
    1u64 + (initial_timer..end)
        // Get each time this lanternfish with spawn
        .step_by(SPAWN_INTERVAL)
        // Get the descendents of each of those, recursively
        .map(|spawn_at| recursive_count(spawn_at + SPAWN_DELAY, end))
        .sum::<u64>()
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let input: Vec<u64> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let total: u64 = input.iter().copied().map(|t| recursive_count(t, 80)).sum();
    Ok(total.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
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
            "???"
        );
        assert_eq!(part2(read_file("data/day06_input.txt")).unwrap(), "???");
    }
}

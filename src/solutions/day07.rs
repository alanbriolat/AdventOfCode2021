use super::prelude::*;
use crate::util::{parse_delimited, read_file, read_line};

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut crabs: Vec<i64> = parse_delimited(read_line(reader).as_str(), ",").collect();
    // Find median
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    // Sum distances to the median
    let fuel: i64 = crabs
        .iter()
        .copied()
        .map(|crab| (crab - median).abs())
        .sum();
    Ok(fuel.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
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
                ???
            "}))
            .unwrap(),
            "???"
        );
        assert_eq!(part2(read_file("data/day07_input.txt")).unwrap(), "???");
    }
}

use itertools::Itertools;

use crate::util::{parse_lines, read_file};
use super::prelude::*;

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok(parse_lines(reader)
        .tuple_windows()
        .map(|(a, b): (u16, u16)| if b > a { 1 } else { 0 })
        .sum::<u16>()
        .to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok(parse_lines(reader)
        .collect_vec()
        .windows(3)
        .map(|slice| slice.iter().sum::<u16>())
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum::<u16>()
        .to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day01_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day01_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::util::read_str;
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(read_str(indoc!{"\
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "})).unwrap(), "7");
        assert_eq!(part1(read_file("data/day01_input.txt")).unwrap(), "1477");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_str(indoc!{"\
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "})).unwrap(), "5");
        assert_eq!(part2(read_file("data/day01_input.txt")).unwrap(), "1523");
    }
}

use std::str::FromStr;

use super::prelude::*;
use crate::error::ParseError;
use crate::util::{parse_lines, read_file};

struct Instruction {
    direction: String,
    amount: i64,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s
            .split_once(' ')
            .ok_or(ParseError::Other("expected delimiter".into()))?;
        let direction = direction.to_owned();
        let amount = amount.parse::<i64>()?;
        Ok(Instruction { direction, amount })
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut x = 0_i64;
    let mut y = 0_i64;
    for Instruction { direction, amount } in parse_lines(reader) {
        match direction.as_str() {
            "up" => {
                y -= amount;
            }
            "down" => {
                y += amount;
            }
            "forward" => {
                x += amount;
            }
            _ => panic!("unexpected direction: {}", direction),
        }
    }
    Ok((x * y).to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut aim = 0_i64;
    for Instruction { direction, amount } in parse_lines(reader) {
        match direction.as_str() {
            "up" => {
                aim -= amount;
            }
            "down" => {
                aim += amount;
            }
            "forward" => {
                x += amount;
                y += amount * aim
            }
            _ => panic!("unexpected direction: {}", direction),
        }
    }
    Ok((x * y).to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day02_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day02_input.txt")));
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
            part1(read_str(indoc! {"
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            "}))
            .unwrap(),
            "150"
        );
        assert_eq!(part1(read_file("data/day02_input.txt")).unwrap(), "1727835");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"
                forward 5
                down 5
                forward 8
                up 3
                down 8
                forward 2
            "}))
            .unwrap(),
            "900"
        );
        assert_eq!(
            part2(read_file("data/day02_input.txt")).unwrap(),
            "1544000595"
        );
    }
}

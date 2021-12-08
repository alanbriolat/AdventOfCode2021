use std::ops::Deref;
use std::str::FromStr;

use super::prelude::*;
use crate::error::ParseError;
use crate::util::{parse_delimited, parse_lines, read_file};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Digit(u8);

impl Digit {
    fn is_unique_segment_count(&self) -> bool {
        match self.0.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }
}

impl Deref for Digit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Digit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0u8;
        for b in s.bytes() {
            bits |= 1 << b - 'a' as u8;
        }
        Ok(Digit(bits))
    }
}

struct Sample {
    patterns: Vec<Digit>,
    value: Vec<Digit>,
}

impl FromStr for Sample {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns_raw, value_raw) = s.split_once(" | ").unwrap();
        Ok(Sample {
            patterns: parse_delimited(patterns_raw, " ").collect(),
            value: parse_delimited(value_raw, " ").collect(),
        })
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok(parse_lines::<Sample, R>(reader)
        .map(|sample| {
            sample
                .value
                .iter()
                .filter(|value| value.is_unique_segment_count())
                .count()
        })
        .sum::<usize>()
        .to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day08_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day08_input.txt")));
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
                be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
            "}))
            .unwrap(),
            "26"
        );
        assert_eq!(part1(read_file("data/day08_input.txt")).unwrap(), "352");
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
        assert_eq!(part2(read_file("data/day08_input.txt")).unwrap(), "???");
    }
}

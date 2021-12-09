use std::collections::HashMap;
use std::str::FromStr;

use bitflags::bitflags;

use super::prelude::*;
use crate::error::ParseError;
use crate::util::{parse_delimited, parse_lines, read_file};

bitflags! {
    /// Signal wires/segments, represented as a bitmask of `0GFEDCBA`.
    struct Signal: u8 {
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const C = 0b0000_0100;
        const D = 0b0000_1000;
        const E = 0b0001_0000;
        const F = 0b0010_0000;
        const G = 0b0100_0000;
    }
}

impl Signal {
    fn is_unique_bit_count(&self) -> bool {
        match self.bits.count_ones() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }
}

impl FromStr for Signal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = 0u8;
        for b in s.bytes() {
            bits |= 1 << b - 'a' as u8;
        }
        Ok(Signal::from_bits(bits).unwrap())
    }
}

#[derive(Debug, Default)]
struct SignalMapping {
    signal_to_digit: HashMap<Signal, u64>,
    digit_to_signal: HashMap<u64, Signal>,
    segment_to_wire: HashMap<Signal, Signal>,
}

impl SignalMapping {
    fn add_signal(&mut self, digit: u64, signal: Signal) {
        self.signal_to_digit.insert(signal, digit);
        self.digit_to_signal.insert(digit, signal);
    }

    fn get_signal(&self, digit: u64) -> Signal {
        self.digit_to_signal[&digit]
    }

    fn get_digit(&self, signal: Signal) -> u64 {
        self.signal_to_digit[&signal]
    }

    fn has_all_signals(&self) -> bool {
        self.signal_to_digit.len() == 10 && self.digit_to_signal.len() == 10
    }

    fn add_segment(&mut self, segment: Signal, wire: Signal) {
        self.segment_to_wire.insert(segment, wire);
    }

    fn get_segment(&self, segment: Signal) -> Signal {
        self.segment_to_wire[&segment]
    }

    fn has_all_segments(&self) -> bool {
        self.segment_to_wire.len() == 7
    }

    fn get_value<I: IntoIterator<Item = Signal>>(&self, signals: I) -> u64 {
        signals
            .into_iter()
            .fold(0, |acc, signal| acc * 10 + self.get_digit(signal))
    }
}

struct Display {
    patterns: Vec<Signal>,
    value: Vec<Signal>,
}

impl Display {
    fn get_mapping(&self) -> SignalMapping {
        let mut mapping = SignalMapping::default();

        // Step 1: map the digits that have a unique number of segments turned on
        for p in self.patterns.iter().copied() {
            match p.bits.count_ones() {
                2 => {
                    mapping.add_signal(1, p);
                }
                3 => {
                    mapping.add_signal(7, p);
                }
                4 => {
                    mapping.add_signal(4, p);
                }
                7 => {
                    mapping.add_signal(8, p);
                }
                _ => {}
            }
        }

        // Step 2: derive some digits directly based on the easier digits
        // (3 is the only 5-segment signal that contains all of the segments of 1)
        for p in self.patterns.iter().copied() {
            if p.bits.count_ones() == 5 && p.contains(mapping.get_signal(1)) {
                mapping.add_signal(3, p);
                break;
            }
        }
        // (9 is the overlap of 3 and 4)
        mapping.add_signal(9, mapping.get_signal(3) | mapping.get_signal(4));

        // Step 3: derive the position of some segments
        // (A is the only segment in 7 but not 1)
        mapping.add_segment(Signal::A, mapping.get_signal(7) & !mapping.get_signal(1));
        // (E is the only segment in 8 but not 9)
        mapping.add_segment(Signal::E, mapping.get_signal(8) & !mapping.get_signal(9));
        // (B is the only segment in 4 but not 3)
        mapping.add_segment(Signal::B, mapping.get_signal(4) & !mapping.get_signal(3));
        // (A and G are left if 4 is removed from 9, and we already know A
        mapping.add_segment(
            Signal::G,
            mapping.get_signal(9) & !(mapping.get_signal(4) | mapping.get_segment(Signal::A)),
        );

        // Step 4: build more digits from segments we know
        // (0 is 7 plus B, E & G)
        mapping.add_signal(
            0,
            mapping.get_signal(7)
                | mapping.get_segment(Signal::B)
                | mapping.get_segment(Signal::E)
                | mapping.get_segment(Signal::G),
        );
        // (2 is the only 5-segment that contains E, 5 is the only 5-segment that contains B)
        for p in self.patterns.iter().copied() {
            if p.bits.count_ones() == 5 {
                if p.contains(mapping.get_segment(Signal::B)) {
                    mapping.add_signal(5, p);
                } else if p.contains(mapping.get_segment(Signal::E)) {
                    mapping.add_signal(2, p);
                }
            }
        }
        // (6 is 5 plus E)
        mapping.add_signal(6, mapping.get_signal(5) | mapping.get_segment(Signal::E));

        // Should have all signals now
        assert!(mapping.has_all_signals());

        // Step 5: derive remaining segments
        // (C is the only segment in 8 but not 6)
        mapping.add_segment(Signal::C, mapping.get_signal(8) & !mapping.get_signal(6));
        // (D is the only segment in 8 but not 0)
        mapping.add_segment(Signal::D, mapping.get_signal(8) & !mapping.get_signal(0));
        // (F is the only segment in 7 but not 2)
        mapping.add_segment(Signal::F, mapping.get_signal(7) & !mapping.get_signal(2));

        // Should have all segments now
        assert!(mapping.has_all_segments());

        mapping
    }

    fn get_value(&self) -> u64 {
        self.get_mapping().get_value(self.value.iter().copied())
    }
}

impl FromStr for Display {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns_raw, value_raw) = s.split_once(" | ").unwrap();
        Ok(Display {
            patterns: parse_delimited(patterns_raw, " ").collect(),
            value: parse_delimited(value_raw, " ").collect(),
        })
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok(parse_lines::<Display, R>(reader)
        .map(|display| {
            display
                .value
                .iter()
                .filter(|value| value.is_unique_bit_count())
                .count()
        })
        .sum::<usize>()
        .to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok(parse_lines::<Display, R>(reader)
        .map(|display| display.get_value())
        .sum::<u64>()
        .to_string())
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
            "61229"
        );
        assert_eq!(part2(read_file("data/day08_input.txt")).unwrap(), "936117");
    }
}

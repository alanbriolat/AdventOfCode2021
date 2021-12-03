use crate::util::{parse_lines, read_file};
use super::prelude::*;

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let lines: Vec<String> = parse_lines(reader).collect();
    let bitcount = lines[0].len();
    let mut counts: Vec<u16> = vec![0; bitcount];
    counts.resize(bitcount, 0);
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }
    let mut gamma = 0u64;
    for (i, count) in counts.iter().rev().copied().enumerate() {
        if count as usize * 2 > lines.len() {
            gamma += 1 << i;
        }
    }
    let mask = 2u64.pow(bitcount as u32) - 1;
    let epsilon = !gamma & mask;
    Ok((gamma * epsilon).to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    Ok("".into())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day03_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day03_input.txt")));
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
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        "})).unwrap(), "198");
    }

    #[test]
    fn test_part2() {

    }
}

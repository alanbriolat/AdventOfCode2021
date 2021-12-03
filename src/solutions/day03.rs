use crate::util::{parse_lines, read_file};
use super::prelude::*;

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let lines: Vec<String> = parse_lines(reader).collect();
    let bitcount = lines[0].len();

    // Count how many 1s are in each position
    let mut counts: Vec<u16> = vec![0; bitcount];
    counts.resize(bitcount, 0);
    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }

    // Create a number by setting bits where more than half the samples are 1
    let mut gamma = 0u64;
    for (i, count) in counts.iter().rev().copied().enumerate() {
        if count as usize * 2 > lines.len() {
            gamma += 1 << i;
        }
    }

    // "epsilon" is just the inverse of "gamma", i.e. majority 0s = minority 1s = not majority 1s
    let mask = 2u64.pow(bitcount as u32) - 1;
    let epsilon = !gamma & mask;

    Ok((gamma * epsilon).to_string())
}

/// Partition `values` into `(least, most)` common value at bit `pos` (counting from the left).
/// If `least` and `most` are the same size, then `least` will contain zeroes and `most` ones.
fn partition_values(values: &[String], pos: usize) -> (&[String], &[String]) {
    // (Binary) search for the partition point between 0s and 1s in the current bit position
    let split = values.partition_point(|v| v.as_bytes()[pos] == '0' as u8);
    let (zeroes, ones) = values.split_at(split);
    // Ensure (least, most) common ordering
    if zeroes.len() > ones.len() {
        (ones, zeroes)
    } else {
        (zeroes, ones)
    }
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut lines: Vec<String> = parse_lines(reader).collect();
    // If the input is sorted, can apply "bit criteria" filter by partitioning the input
    lines.sort();
    let bitcount = lines[0].len();

    let mut oxygen = &lines[..];
    for i in 0..bitcount {
        oxygen = partition_values(oxygen, i).1;
        if oxygen.len() == 1 {
            break;
        }
    }
    assert_eq!(oxygen.len(), 1);
    let oxygen_rating = u64::from_str_radix(oxygen[0].as_str(), 2).unwrap();

    let mut scrubber = &lines[..];
    for i in 0..bitcount {
        scrubber = partition_values(scrubber, i).0;
        if scrubber.len() == 1 {
            break;
        }
    }
    assert_eq!(scrubber.len(), 1);
    let scrubber_rating = u64::from_str_radix(scrubber[0].as_str(), 2).unwrap();

    Ok((oxygen_rating * scrubber_rating).to_string())
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
        assert_eq!(part1(read_file("data/day03_input.txt")).unwrap(), "4191876");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_str(indoc!{"\
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
        "})).unwrap(), "230");
        assert_eq!(part2(read_file("data/day03_input.txt")).unwrap(), "3414905");
    }
}

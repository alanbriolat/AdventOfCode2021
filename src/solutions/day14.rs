use std::collections::HashMap;

use itertools::Itertools;

use super::prelude::*;
use crate::util::{read_file, Counter};

struct Polymer {
    template: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

impl Polymer {
    fn from_reader<R: BufRead>(reader: R) -> Polymer {
        let mut lines = reader.lines().map(|line| line.unwrap());
        let template = lines.next().unwrap().into_bytes();
        lines.next();
        let rules = lines
            .map(|line| {
                let (matcher, insert) = line.split_once(" -> ").unwrap();
                let matcher = (matcher.as_bytes()[0], matcher.as_bytes()[1]);
                let insert = insert.as_bytes()[0];
                (matcher, insert)
            })
            .collect();
        Polymer { template, rules }
    }

    fn step(&self, prev: &[u8]) -> Vec<u8> {
        let mut next = Vec::with_capacity(prev.len() * 2 - 1);
        for (a, b) in prev.iter().copied().tuple_windows() {
            next.push(a);
            next.push(self.rules.get(&(a, b)).copied().unwrap());
        }
        next.push(prev[prev.len() - 1]);
        next
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let polymer = Polymer::from_reader(reader);
    let last = std::iter::successors(Some(polymer.template.clone()), |prev| {
        Some(polymer.step(prev.as_ref()))
    })
    .nth(10)
    .unwrap();
    let mut counter = Counter::new();
    counter.count(last.iter().copied());
    let mut counts: Vec<_> = counter.into_iter().collect();
    counts.sort_by_key(|(_, k)| *k);
    let result = counts.last().unwrap().1 - counts.first().unwrap().1;
    Ok(result.to_string())
}

fn part2<R: BufRead>(_reader: R) -> crate::Result<String> {
    Err(crate::Error::Other("not implemented".into()))
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day14_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day14_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(read_file("data/day14_example1.txt")).unwrap(), "1588");
        assert_eq!(part1(read_file("data/day14_input.txt")).unwrap(), "2360");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_file("data/day14_example1.txt")).unwrap(), "???");
        assert_eq!(part2(read_file("data/day14_input.txt")).unwrap(), "???");
    }
}

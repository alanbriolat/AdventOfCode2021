use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use crate::error::ParseError;
use crate::util::{parse_lines, read_file};
use crate::vector::Vector;
use super::prelude::*;

type Point = Vector<i16, 2>;

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParseError::Other("expected ','".into()))?;
        Ok([x.parse()?, y.parse()?].into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Orientation {
    Diagonal,
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn orientation(&self) -> Orientation {
        match ((self.end[0] - self.start[0]).abs(), (self.start[1] - self.end[1]).abs()) {
            (0, _) => Orientation::Vertical,
            (_, 0) => Orientation::Horizontal,
            (a, b) if a == b => Orientation::Diagonal,
            _ => panic!("non-45-degree diagonal"),
        }
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        match self.orientation() {
            Orientation::Diagonal => {
                let start = self.start;
                let direction: Point = [
                    (self.end[0] - self.start[0]).signum(),
                    (self.end[1] - self.start[1]).signum(),
                ].into();
                let steps = (self.end[0] - self.start[0]).abs();
                Box::new((0..=steps).map(move |i| start + direction * [i, i]))
            }
            Orientation::Horizontal => {
                let y = self.start[1];
                let (mut start, mut end) = (self.start[0], self.end[0]);
                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }
                Box::new((start..=end).map(move |x| [x, y].into()))
            }
            Orientation::Vertical => {
                let x = self.start[0];
                let (mut start, mut end) = (self.start[1], self.end[1]);
                if start > end {
                    std::mem::swap(&mut start, &mut end);
                }
                Box::new((start..=end).map(move |y| [x, y].into()))
            }
        }
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(ParseError::Other("expected ' -> '".into()))?;
        Ok(Line { start: start.parse()?, end: end.parse()? })
    }
}

struct Counter<T: Clone + Eq + Hash>(HashMap<T, usize>);

impl<T: Clone + Eq + Hash> Counter<T> {
    fn new() -> Self {
        Counter(HashMap::new())
    }

    fn count<I: IntoIterator<Item = T>>(&mut self, items: I) {
        for item in items {
            *self.0.entry(item).or_default() += 1;
        }
    }
}

impl<T: Clone + Eq + Hash> std::ops::Deref for Counter<T> {
    type Target = HashMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let data: Vec<Line> = parse_lines(reader)
        .filter(|line: &Line| line.orientation() != Orientation::Diagonal)
        .collect();
    let mut counter = Counter::new();
    for line in data.iter() {
        counter.count(line.points());
    }
    let count = counter.iter().filter(|(_, &v)| v > 1).count();
    Ok(count.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let data: Vec<Line> = parse_lines(reader).collect();
    let mut counter = Counter::new();
    for line in data.iter() {
        counter.count(line.points());
    }
    let count = counter.iter().filter(|(_, &v)| v > 1).count();
    Ok(count.to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day05_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day05_input.txt")));
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
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        "})).unwrap(), "5");
        assert_eq!(part1(read_file("data/day05_input.txt")).unwrap(), "7380");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_str(indoc!{"\
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        "})).unwrap(), "12");
        assert_eq!(part2(read_file("data/day05_input.txt")).unwrap(), "21373");
    }
}
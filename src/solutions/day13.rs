use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

use super::prelude::*;
use crate::error::ParseError;
use crate::util::{parse_lines, read_file};
use crate::vector::Vector;

type Point = Vector<u16, 2>;

fn fold_coordinate(a: u16, f: u16) -> u16 {
    if a < f {
        a
    } else {
        2 * f - a
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(u16),
    Y(u16),
}

impl Fold {
    fn apply(&self, point: Point) -> Point {
        match self {
            Fold::X(position) => Point::from([fold_coordinate(point[0], *position), point[1]]),
            Fold::Y(position) => Point::from([point[0], fold_coordinate(point[1], *position)]),
        }
    }
}

impl FromStr for Fold {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (leading, spec) = s.split_at(11);
        assert_eq!(leading, "fold along ");
        let (direction, position) = spec.split_once('=').unwrap();
        match direction {
            "x" => Ok(Fold::X(position.parse()?)),
            "y" => Ok(Fold::Y(position.parse()?)),
            unknown => Err(ParseError::Other(format!("unknown direction: {}", unknown))),
        }
    }
}

#[derive(Clone)]
struct Page {
    dots: HashSet<Point>,
    folds: VecDeque<Fold>,
}

impl Page {
    fn from_reader<R: BufRead>(reader: R) -> Page {
        let lines: Vec<String> = parse_lines(reader).collect();
        let (split, _) = lines.iter().find_position(|line| line.len() == 0).unwrap();
        let dots = lines[0..split]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
        let folds = lines[(split + 1)..]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
        Page { dots, folds }
    }

    fn count_dots(&self) -> usize {
        self.dots.len()
    }

    fn step(&mut self) -> bool {
        if let Some(fold) = self.folds.pop_front() {
            let new = self
                .dots
                .iter()
                .copied()
                .map(|point| fold.apply(point))
                .collect();
            self.dots = new;
            true
        } else {
            false
        }
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut page = Page::from_reader(reader);
    page.step();
    Ok(page.count_dots().to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day13_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day13_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(read_file("data/day13_example1.txt")).unwrap(), "17");
        assert_eq!(part1(read_file("data/day13_input.txt")).unwrap(), "618");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_file("data/day13_example1.txt")).unwrap(), "???");
        assert_eq!(part2(read_file("data/day13_input.txt")).unwrap(), "???");
    }
}

use std::collections::VecDeque;

use super::prelude::*;
use crate::grid::{DynamicGrid2D, GridOps};
use crate::util::{parse_lines, read_file};

type Grid = DynamicGrid2D<u8>;
type Point = <Grid as crate::grid::GridOps>::Point;

struct State {
    grid: Grid,
}

impl State {
    fn from_reader<R: BufRead>(reader: R) -> State {
        let raw: Vec<String> = parse_lines(reader).collect();
        let (width, height) = (raw[0].len(), raw.len());
        State {
            grid: Grid::new(
                width,
                height,
                raw.iter()
                    .flat_map(|line| line.bytes().map(|b| b - '0' as u8)),
            ),
        }
    }

    fn step(&mut self) -> u64 {
        let mut flashes: u64 = 0;
        let mut flashing: VecDeque<Point> = VecDeque::new();
        for p in self.grid.iter_points() {
            self.grid[p] += 1;
            // If this octopus went over energy 9, flash, potentially starting a chain reaction
            if self.grid[p] == 10 {
                flashing.push_back(p);
            }
            // Processing flashing octopuses until the chain reaction stops
            while let Some(p) = flashing.pop_front() {
                flashes += 1;
                for adj in self.grid.iter_adjacent_8(&p) {
                    self.grid[adj] += 1;
                    if self.grid[adj] == 10 {
                        flashing.push_back(adj);
                    }
                }
            }
        }
        // Reset any octopuses that flashed during this step
        for p in self.grid.iter_points() {
            if self.grid[p] > 9 {
                self.grid[p] = 0;
            }
        }
        flashes
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut state = State::from_reader(reader);
    Ok(std::iter::repeat_with(|| state.step())
        .take(100)
        .sum::<u64>()
        .to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let mut state = State::from_reader(reader);
    Ok(std::iter::repeat_with(|| state.step())
        .enumerate()
        .find_map(|(i, flashes)| if flashes == 100 { Some(i + 1) } else { None })
        .unwrap()
        .to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day11_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day11_input.txt")));
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
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526
            "}))
            .unwrap(),
            "1656"
        );
        assert_eq!(part1(read_file("data/day11_input.txt")).unwrap(), "1640");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                5483143223
                2745854711
                5264556173
                6141336146
                6357385478
                4167524645
                2176841721
                6882881134
                4846848554
                5283751526
            "}))
            .unwrap(),
            "195"
        );
        assert_eq!(part2(read_file("data/day11_input.txt")).unwrap(), "312");
    }
}

use std::collections::{HashSet, VecDeque};

use crate::error::ParseError;
use crate::util::read_file;
use super::prelude::*;

struct Game {
    draw: VecDeque<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn read<R: BufRead>(reader: &mut R) -> crate::Result<Game> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        buf.truncate(buf.len() - 1);    // Drop newline
        let draw: VecDeque<_> = buf.split(',').map(|v| v.parse::<u8>().unwrap()).collect();

        let mut boards = Vec::new();
        while let Ok(1..) = reader.read_line(&mut buf) {
            boards.push(Board::read(reader)?);
        }

        Ok(Game { draw, boards })
    }

    /// Apply the next drawn number to all the boards, returning copies of any boards that won due
    /// to that draw. Returns `None` if no more numbers are left to draw.
    fn step(&mut self) -> Option<Vec<(Board, u8)>> {
        let next = self.draw.pop_front()?;
        Some(self.boards.iter_mut().filter_map(|board| {
            if board.won || !board.apply(next) {
                None
            } else {
                Some((board.clone(), next))
            }
        }).collect())
    }

    /// Run through the drawn numbers, yielding boards in the order they won, along with the drawn
    /// number that caused them to win.
    fn run(&mut self) -> impl Iterator<Item = (Board, u8)> + '_ {
        std::iter::from_fn(|| self.step()).flatten()
    }

    fn run_until_first_win(&mut self) -> (Board, u8) {
        self.run().next().unwrap()
    }

    fn run_until_last_win(&mut self) -> (Board, u8) {
        self.run().last().unwrap()
    }
}

#[derive(Clone)]
struct Board {
    rows: Vec<HashSet<u8>>,
    cols: Vec<HashSet<u8>>,
    won: bool,
}

impl Board {
    fn read<R: BufRead>(reader: &mut R) -> crate::Result<Board> {
        let mut rows = vec![HashSet::new(); 5];
        let mut cols = vec![HashSet::new(); 5];
        let mut buf = String::with_capacity(3 * 5);

        for i in 0..5 {
            buf.clear();
            reader.read_line(&mut buf)?;
            buf.truncate(buf.len() - 1);    // Drop newline
            for j in 0..5 {
                let start: usize = j * 3;
                let cell = buf[start..start+2].trim_start().parse::<u8>().map_err(ParseError::from)?;
                rows[i].insert(cell);
                cols[j].insert(cell);
            }
        }

        Ok(Board { rows, cols, won: false })
    }

    /// Apply `v` to the board, returning `true` if this number caused the board to win. (If the
    /// board already won, then nothing is changed.)
    fn apply(&mut self, v: u8) -> bool {
        if self.won {
            return false;
        }

        for row in self.rows.iter_mut() {
            // We won if we removed the last number from a row
            if row.remove(&v) {
                if row.is_empty() {
                    self.won = true;
                }
            }
        }
        for col in self.cols.iter_mut() {
            // We won if we removed the last number from a column
            if col.remove(&v) {
                if col.is_empty() {
                    self.won = true;
                }
            }
        }
        return self.won;
    }

    fn score(&self) -> u64 {
        self.rows.iter().map(|row| row.iter().map(|&v| v as u64).sum::<u64>()).sum()
    }
}

fn part1<R: BufRead>(mut reader: R) -> crate::Result<String> {
    let mut game = Game::read(&mut reader)?;
    let (board, last) = game.run_until_first_win();
    let score = board.score() * last as u64;
    Ok(score.to_string())
}

fn part2<R: BufRead>(mut reader: R) -> crate::Result<String> {
    let mut game = Game::read(&mut reader)?;
    let (board, last) = game.run_until_last_win();
    let score = board.score() * last as u64;
    Ok(score.to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day04_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day04_input.txt")));
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
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "})).unwrap(), "4512");
        assert_eq!(part1(read_file("data/day04_input.txt")).unwrap(), "31424");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_str(indoc!{"\
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "})).unwrap(), "1924");
        assert_eq!(part2(read_file("data/day04_input.txt")).unwrap(), "23042");
    }
}

use std::collections::{HashSet, VecDeque}
;
use crate::error::ParseError;
use crate::util::{parse_lines, read_file};
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

    fn step(&mut self) -> Option<(Board, u8)> {
        let next = self.draw.pop_front()?;
        for board in self.boards.iter_mut() {
            let win = board.remove(next);
            if win {
                return Some((board.clone(), next));
            }
        }
        None
    }

    fn run(&mut self) -> (Board, u8) {
        loop {
            if let Some((board, last)) = self.step() {
                return (board, last);
            }
        }
    }
}

#[derive(Clone)]
struct Board {
    rows: Vec<HashSet<u8>>,
    cols: Vec<HashSet<u8>>,
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

        Ok(Board { rows, cols })
    }

    fn remove(&mut self, v: u8) -> bool {
        let mut win = false;
        for row in self.rows.iter_mut() {
            if row.remove(&v) {
                if row.is_empty() {
                    win = true;
                }
            }
        }
        for col in self.cols.iter_mut() {
            if col.remove(&v) {
                if col.is_empty() {
                    win = true;
                }
            }
        }
        win
    }

    fn score(&self) -> u64 {
        self.rows.iter().map(|row| row.iter().map(|&v| v as u64).sum::<u64>()).sum()
    }
}

fn part1<R: BufRead>(mut reader: R) -> crate::Result<String> {
    let mut game = Game::read(&mut reader)?;
    let (board, last) = game.run();
    let score = board.score() * last as u64;
    Ok(score.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
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
            ???
        "})).unwrap(), "???");
        assert_eq!(part2(read_file("data/day04_input.txt")).unwrap(), "???");
    }
}

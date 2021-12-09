use std::collections::{HashSet, VecDeque};
use std::ops::Index;

use num::CheckedSub;

use super::prelude::*;
use crate::util::{parse_lines, read_file};
use crate::vector::Vector;

type Coord = Vector<usize, 2>;

struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl HeightMap {
    fn from_reader<R: BufRead>(reader: R) -> HeightMap {
        let raw: Vec<String> = parse_lines(reader).collect();
        let (width, height) = (raw[0].len(), raw.len());
        let data: Vec<u8> = raw
            .iter()
            .flat_map(|line| line.bytes().map(|b| b - '0' as u8))
            .collect();
        HeightMap {
            width,
            height,
            data,
        }
    }

    fn valid_coord(&self, coord: &Coord) -> bool {
        (0..self.width).contains(&coord[0]) && (0..self.height).contains(&coord[1])
    }

    fn index_from_coord(&self, coord: &Coord) -> Option<usize> {
        if self.valid_coord(coord) {
            Some(coord[1] * self.width + coord[0])
        } else {
            None
        }
    }

    fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        let (width, height) = (self.width, self.height);
        (0..height).flat_map(move |y| (0..width).map(move |x| [x, y].into()))
    }

    /// Iterate over *valid* coordinates that are 4-adjacent to `coord`.
    fn iter_adjacent_4_coords(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        let candidates = [
            coord.checked_sub(&[1, 0].into()),
            Some(coord + [1, 0]),
            coord.checked_sub(&[0, 1].into()),
            Some(coord + [0, 1]),
        ];
        candidates
            .into_iter()
            .flatten()
            .filter(|coord| self.valid_coord(coord))
    }

    fn iter_adjacent_4_values(&self, coord: Coord) -> impl Iterator<Item = u8> + '_ {
        self.iter_adjacent_4_coords(coord).map(|coord| self[coord])
    }

    fn iter_low_points(&self) -> impl Iterator<Item = (Coord, u8)> + '_ {
        self.iter_coords().filter_map(|coord| {
            let height = self[coord];
            let is_low_point = self
                .iter_adjacent_4_values(coord)
                .all(|adjacent_height| adjacent_height > height);
            if is_low_point {
                Some((coord, height))
            } else {
                None
            }
        })
    }

    /// Get the size of the basin that drains to `low_point`.
    ///
    /// Because all basins all drain to a low point, and all locations below height 9 are part of a
    /// basin, we can apply a flood-fill algorithm from each low point that refuses to traverse a
    /// height of 9.
    fn get_basin_size(&self, low_point: Coord) -> u64 {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut next: VecDeque<Coord> = VecDeque::new();
        let mut size: u64 = 0;

        next.push_back(low_point);
        visited.insert(low_point);

        while let Some(coord) = next.pop_front() {
            match self[coord] {
                0..=8 => {
                    size += 1;
                    next.extend(
                        self.iter_adjacent_4_coords(coord)
                            .filter(|c| !visited.contains(c)),
                    );
                    visited.extend(self.iter_adjacent_4_coords(coord));
                }
                _ => {}
            }
        }

        size
    }
}

impl Index<Coord> for HeightMap {
    type Output = u8;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.data[self.index_from_coord(&index).unwrap()]
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let heightmap = HeightMap::from_reader(reader);
    let result: u64 = heightmap
        .iter_low_points()
        .map(|(_, height)| (height + 1) as u64)
        .sum();
    Ok(result.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let heightmap = HeightMap::from_reader(reader);
    let mut basin_sizes: Vec<u64> = heightmap
        .iter_low_points()
        .map(|(coord, _)| heightmap.get_basin_size(coord))
        .collect();
    basin_sizes.sort();
    let result: u64 = basin_sizes[(basin_sizes.len() - 3)..].iter().product();
    Ok(result.to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day09_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day09_input.txt")));
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
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
            "}))
            .unwrap(),
            "15"
        );
        assert_eq!(part1(read_file("data/day09_input.txt")).unwrap(), "436");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                2199943210
                3987894921
                9856789892
                8767896789
                9899965678
            "}))
            .unwrap(),
            "1134"
        );
        assert_eq!(part2(read_file("data/day09_input.txt")).unwrap(), "1317792");
    }
}

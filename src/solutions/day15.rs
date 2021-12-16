use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::rc::Rc;

use super::prelude::*;
use crate::grid;
use crate::util::{read_file, read_number_grid};

type Grid = grid::Grid<u8, 2>;
type Point = grid::Point<2>;

struct Cave {
    grid: Grid,
}

impl Cave {
    fn from_reader<R: BufRead>(reader: R) -> Cave {
        Cave {
            grid: read_number_grid(reader),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Path {
    head: Point,
    tail: Option<Rc<Path>>,
}

impl Path {
    fn new(head: Point) -> Rc<Path> {
        Rc::new(Path { head, tail: None })
    }

    fn append(self: &Rc<Self>, head: Point) -> Rc<Path> {
        Rc::new(Path {
            head,
            tail: Some(self.clone()),
        })
    }

    fn iter(self: &Rc<Self>) -> PathIter {
        PathIter {
            next: Some(self.clone()),
        }
    }
}

struct PathIter {
    next: Option<Rc<Path>>,
}

impl Iterator for PathIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(path) = self.next.take() {
            self.next = path.tail.clone();
            Some(path.head)
        } else {
            None
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    path: Rc<Path>,
}

impl State {
    fn new(start: Point) -> State {
        State {
            cost: 0,
            path: Path::new(start),
        }
    }

    #[inline]
    fn position(&self) -> Point {
        self.path.head
    }

    fn append(&self, point: Point, cost: usize) -> State {
        State {
            cost: cost + self.cost,
            path: self.path.append(point),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering, so BinaryHeap can be used as a priority queue with lowest cost first
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position().cmp(&other.position()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut path: Vec<_> = self.path.iter().collect();
        path.reverse();
        write!(f, "State {{ cost: {}, path: {:?} }}", self.cost, path)
    }
}

fn shortest_path_dijkstra<F, I>(start: Point, end: Point, next: F) -> Option<State>
where
    F: Fn(Point) -> I,
    I: Iterator<Item = (Point, usize)>,
{
    let mut distance: HashMap<Point, usize> = HashMap::new();
    distance.insert(start, 0);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State::new(start));

    // Expand the shortest path each time
    while let Some(state) = heap.pop() {
        // If we found the goal, return the successful shortest path
        if state.position() == end {
            return Some(state);
        }

        // If we already have a shorter path to this position, discard
        if let Some(cost) = distance.get(&state.position()) {
            if state.cost > *cost {
                continue;
            }
        }

        // For each possible next position, if we now have a lower-cost (or only) path to that
        // position, keep the path
        for (point, cost) in next(state.position()) {
            let next_state = state.append(point, cost);
            match distance.get(&next_state.position()) {
                Some(cost) if next_state.cost >= *cost => {}
                _ => {
                    distance.insert(point, next_state.cost);
                    heap.push(next_state);
                }
            }
        }
    }

    None
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let cave = Cave::from_reader(reader);
    let start = cave.grid.min_point();
    let end = cave.grid.max_point();
    let path = shortest_path_dijkstra(start, end, |point| {
        cave.grid
            .iter_adjacent_4_points(point)
            .map(|p| (p, cave.grid[p] as usize))
    });
    Ok(path.unwrap().cost.to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    todo!()
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day15_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day15_input.txt")));
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
                1163751742
                1381373672
                2136511328
                3694931569
                7463417111
                1319128137
                1359912421
                3125421639
                1293138521
                2311944581
            "}))
            .unwrap(),
            "40"
        );
        assert_eq!(part1(read_file("data/day15_input.txt")).unwrap(), "447");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(read_str(indoc! {"\
                ???
            "}))
            .unwrap(),
            "???"
        );
        assert_eq!(part2(read_file("data/day15_input.txt")).unwrap(), "???");
    }
}

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::prelude::*;
use crate::error::ParseError;
use crate::util::{parse_lines, read_file};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node(String);

impl FromStr for Node {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node(s.to_owned()))
    }
}

impl Node {
    fn is_large(&self) -> bool {
        self.0.as_bytes()[0].is_ascii_uppercase()
    }
}

struct Edge(Node, Node);

impl FromStr for Edge {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').unwrap();
        Ok(Edge(a.parse()?, b.parse()?))
    }
}

#[derive(Clone, Debug)]
struct Path(Vec<Node>, u8);

impl Path {
    fn new(initial: Node) -> Path {
        Path(vec![initial], 0)
    }

    fn with_revisits(self, n: u8) -> Path {
        Path(self.0, n)
    }

    fn last(&self) -> &Node {
        self.0.last().unwrap()
    }

    fn try_add(&self, node: Node) -> Option<Path> {
        let mut next = self.clone();
        if node.is_large() {
            next.0.push(node);
        } else if !next.0.contains(&node) {
            next.0.push(node);
        } else if next.1 > 0 {
            next.1 -= 1;
            next.0.push(node);
        } else {
            return None;
        }
        Some(next)
    }
}

struct CaveMap {
    edges: HashMap<Node, HashSet<Node>>,
}

impl CaveMap {
    fn from_reader<R: BufRead>(reader: R) -> CaveMap {
        // Parse all edges as bidirectional
        let mut edges: HashMap<Node, HashSet<Node>> = HashMap::new();
        for Edge(a, b) in parse_lines::<Edge, R>(reader) {
            edges.entry(a.clone()).or_default().insert(b.clone());
            edges.entry(b).or_default().insert(a);
        }

        // Remove edges towards "start" or away from "end", since both can only be visited once
        let start = Node("start".into());
        let end = Node("end".into());
        edges.remove(&end);
        for (_, next) in edges.iter_mut() {
            next.remove(&start);
        }

        CaveMap { edges }
    }

    fn iter_paths(&self, path: Path) -> Box<dyn Iterator<Item = Path> + '_> {
        match self.edges.get(path.last()) {
            Some(candidates) => Box::new(
                candidates
                    .iter()
                    .filter_map(move |next| {
                        path.try_add(next.clone())
                            .map(|next_path| self.iter_paths(next_path))
                    })
                    .flatten(),
            ),
            None => Box::new(std::iter::once(path)),
        }
    }

    fn iter_all_paths(&self, revisits: u8) -> Box<dyn Iterator<Item = Path> + '_> {
        let initial = Path::new(Node("start".into())).with_revisits(revisits);
        self.iter_paths(initial)
    }
}

fn part1<R: BufRead>(reader: R) -> crate::Result<String> {
    let cave_map = CaveMap::from_reader(reader);
    Ok(cave_map.iter_all_paths(0).count().to_string())
}

fn part2<R: BufRead>(reader: R) -> crate::Result<String> {
    let cave_map = CaveMap::from_reader(reader);
    Ok(cave_map.iter_all_paths(1).count().to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(read_file("data/day12_input.txt")));
    runner.add_fn("part2", || part2(read_file("data/day12_input.txt")));
    runner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(read_file("data/day12_example1.txt")).unwrap(), "10");
        assert_eq!(part1(read_file("data/day12_example2.txt")).unwrap(), "19");
        assert_eq!(part1(read_file("data/day12_example3.txt")).unwrap(), "226");
        assert_eq!(part1(read_file("data/day12_input.txt")).unwrap(), "4495");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(read_file("data/day12_example1.txt")).unwrap(), "36");
        assert_eq!(part2(read_file("data/day12_example2.txt")).unwrap(), "103");
        assert_eq!(part2(read_file("data/day12_example3.txt")).unwrap(), "3509");
        assert_eq!(part2(read_file("data/day12_input.txt")).unwrap(), "131254");
    }
}

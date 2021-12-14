use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_file<P: AsRef<Path>>(path: P) -> impl BufRead {
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
}

pub fn read_str(data: &str) -> impl BufRead + '_ {
    io::BufReader::new(data.as_bytes())
}

pub fn read_line<R: BufRead>(mut reader: R) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.truncate(line.len() - 1);
    line
}

pub fn parse_lines<T, R>(reader: R) -> impl Iterator<Item = T>
where
    R: BufRead,
    T: FromStr,
    T::Err: fmt::Debug,
{
    reader
        .lines()
        .map(|line| line.unwrap().parse::<T>().unwrap())
}

pub fn parse_delimited<'a, T>(input: &'a str, pattern: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    input.split(pattern).map(|x| x.parse::<T>().unwrap())
}

pub trait BufReadExt: BufRead {}

pub struct Counter<T: Clone + Eq + Hash>(HashMap<T, usize>);

impl<T: Clone + Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Counter(HashMap::new())
    }

    pub fn count<I: IntoIterator<Item = T>>(&mut self, items: I) {
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

impl<T: Clone + Eq + Hash> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

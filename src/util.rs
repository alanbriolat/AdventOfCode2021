use std::fmt;
use std::fs::File;
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

pub trait BufReadExt: BufRead {}

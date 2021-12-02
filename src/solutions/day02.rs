use super::prelude::*;

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<(String, i64)>> {
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);
    let data: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, amount) = line.split_once(' ').unwrap();
            (direction.to_owned(), amount.parse::<i64>().unwrap())
        })
        .collect();
    Ok(data)
}

fn part1(input_path: &PathBuf) -> crate::Result<String> {
    let mut x = 0_i64;
    let mut y = 0_i64;
    for (direction, amount) in read_input(input_path)? {
        match direction.as_str() {
            "up" => { y -= amount; },
            "down" => { y += amount; },
            "forward" => { x += amount; },
            _ => panic!("unexpected direction: {}", direction),
        }
    }
    Ok((x * y).to_string())
}

fn part2(input_path: &PathBuf) -> crate::Result<String> {
    let mut x = 0_i64;
    let mut y = 0_i64;
    let mut aim = 0_i64;
    for (direction, amount) in read_input(input_path)? {
        match direction.as_str() {
            "up" => { aim -= amount; },
            "down" => { aim += amount; },
            "forward" => { x += amount; y += amount * aim },
            _ => panic!("unexpected direction: {}", direction),
        }
    }
    Ok((x * y).to_string())
}

pub fn build_runner() -> crate::Runner {
    let mut runner = crate::Runner::default();
    runner.add_fn("part1", || part1(&"data/day02_input.txt".into()));
    runner.add_fn("part2", || part2(&"data/day02_input.txt".into()));
    runner
}

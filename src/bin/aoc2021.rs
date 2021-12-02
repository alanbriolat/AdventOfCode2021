use std::env;
use std::time;

use aoc2021::solutions::build_runner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let runner = build_runner();
    for key in runner.list() {
        if args.len() > 0 && !args.iter().any(|pattern| key.starts_with(pattern)) {
            println!("SKIP: {}", key);
            continue;
        }
        let start = time::Instant::now();
        let result = runner.run(key);
        let elapsed = time::Instant::now().duration_since(start);
        match result {
            Ok(output) => println!("RUN : {}: {} ({:?})", key, output, elapsed),
            Err(err) => println!("ERR : {}: {}", key, err),
        }
    }
}

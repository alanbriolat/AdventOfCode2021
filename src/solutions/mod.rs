mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub mod prelude;

macro_rules! register_solutions {
    ($($name:ident,)+) => {
        pub fn build_runner() -> crate::Runner {
            let mut runner = crate::Runner::default();
            $(runner.merge(stringify!($name), $name::build_runner());)+
            runner
        }
    };
}

#[rustfmt::skip]
register_solutions!(
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
);

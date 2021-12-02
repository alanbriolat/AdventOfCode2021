pub mod prelude;

macro_rules! register_solutions {
    ($($name:ident,)+) => {
        $(mod $name;)+

        pub fn build_runner() -> crate::Runner {
            let mut runner = crate::Runner::default();
            $(runner.merge(stringify!($name), $name::build_runner());)+
            runner
        }
    };
}

register_solutions!(
    day01,
    // day02,
);

use crate::{Day, SolutionFnT};

#[path = "helpers/mod.rs"]
mod helpers;

#[macro_export]
macro_rules! days {
    ($($day: ident), *) => {
        $(
        pub mod $day;
        )*
        pub(crate) fn days() -> Vec<(&'static str, (SolutionFnT, SolutionFnT))> {
            [

                $(
                    (
                        stringify!($day),
                        ($day::Solution::first_star as fn(&str) -> String, $day::Solution::second_star as fn(&str) -> String),
                    ),
                )*
                ].to_vec()
            }
    }
    }

days!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18
);

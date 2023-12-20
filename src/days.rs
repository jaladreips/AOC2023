use crate::{SolutionFnT, Day};

#[path = "day01/impl.rs"] pub mod day01;
#[path = "day02/impl.rs"] pub mod day02;
#[path = "day03/impl.rs"] pub mod day03;
#[path = "day04/impl.rs"] pub mod day04;
#[path = "day05/impl.rs"] pub mod day05;
#[path = "day06/impl.rs"] pub mod day06;
#[path = "day07/impl.rs"] pub mod day07;
#[path = "day08/impl.rs"] pub mod day08;
#[path = "day09/impl.rs"] pub mod day09;
#[path = "day10/impl.rs"] pub mod day10;
#[path = "day11/impl.rs"] pub mod day11;
#[path = "day12/impl.rs"] pub mod day12;
#[path = "day13/impl.rs"] pub mod day13;
#[path = "day14/impl.rs"] pub mod day14;
#[path = "day15/impl.rs"] pub mod day15;

pub(crate) fn days() -> [(&'static str, (SolutionFnT, SolutionFnT)); 15] {
    [
        (
            "day01",
            (
                day01::Solution::first_star,
                day01::Solution::second_star,
            ),
        ),
        (
            "day02",
            (
                day02::Solution::first_star,
                day02::Solution::second_star,
            ),
        ),
        (
            "day03",
            (
                day03::Solution::first_star,
                day03::Solution::second_star,
            ),
        ),
        (
            "day04",
            (
                day04::Solution::first_star,
                day04::Solution::second_star,
            ),
        ),
        (
            "day05",
            (
                day05::Solution::first_star,
                day05::Solution::second_star,
            ),
        ),
        (
            "day06",
            (
                day06::Solution::first_star,
                day06::Solution::second_star,
            ),
        ),
        (
            "day07",
            (
                day07::Solution::first_star,
                day07::Solution::second_star,
            ),
        ),
        (
            "day08",
            (
                day08::Solution::first_star,
                day08::Solution::second_star,
            ),
        ),
        (
            "day09",
            (
                day09::Solution::first_star,
                day09::Solution::second_star,
            ),
        ),
        (
            "day10",
            (
                day10::Solution::first_star,
                day10::Solution::second_star,
            ),
        ),
        (
            "day11",
            (
                day11::Solution::first_star,
                day11::Solution::second_star,
            ),
        ),
        (
            "day12",
            (
                day12::Solution::first_star,
                day12::Solution::second_star,
            ),
        ),
        (
            "day13",
            (
                day13::Solution::first_star,
                day13::Solution::second_star,
            ),
        ),
        (
            "day14",
            (
                day14::Solution::first_star,
                day14::Solution::second_star,
            ),
        ),
        (
            "day15",
            (
                day15::Solution::first_star,
                day15::Solution::second_star,
            ),
        ),
    ]
}

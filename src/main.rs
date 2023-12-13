use std::{fs, io};

mod days;

type SolutionFnT = fn(&str) -> String;
trait Day {
    fn first_star(input: &str) -> String;
    fn second_star(input: &str) -> String;
}

fn main() {
    let days = days();

    for (day_name, day_impl) in days {
        let Ok((first, second)) = run(day_name, day_impl) else {
            unimplemented!()
        };

        println!("[{day_name}] First answer: {first} | Second answer: {second}");
    }
}

fn run(dir: &str, solutions: (SolutionFnT, SolutionFnT)) -> io::Result<(String, String)> {
    let mut input_file = std::env::current_exe()?;
    input_file.pop();
    input_file.pop();
    input_file.pop();
    input_file.push("src");
    input_file.push(dir);
    input_file.push("input.txt");

    let input = fs::read_to_string(&input_file);
    let input =
        input.unwrap_or_else(|_| panic!("Failed to read from {}", input_file.to_str().unwrap()));

    Ok((
        std::panic::catch_unwind(|| solutions.0(&input)).unwrap_or(String::from("unimplemented?")),
        std::panic::catch_unwind(|| solutions.1(&input)).unwrap_or(String::from("unimplemented?")),
    ))
}

fn days() -> [(&'static str, (SolutionFnT, SolutionFnT)); 8] {
    [
        (
            "day01",
            (
                days::day01::Solution::first_star,
                days::day01::Solution::second_star,
            ),
        ),
        (
            "day02",
            (
                days::day02::Solution::first_star,
                days::day02::Solution::second_star,
            ),
        ),
        (
            "day03",
            (
                days::day03::Solution::first_star,
                days::day03::Solution::second_star,
            ),
        ),
        (
            "day04",
            (
                days::day04::Solution::first_star,
                days::day04::Solution::second_star,
            ),
        ),
        (
            "day05",
            (
                days::day05::Solution::first_star,
                days::day05::Solution::second_star,
            ),
        ),
        (
            "day06",
            (
                days::day06::Solution::first_star,
                days::day06::Solution::second_star,
            ),
        ),
        (
            "day07",
            (
                days::day07::Solution::first_star,
                days::day07::Solution::second_star,
            ),
        ),
        (
            "day08",
            (
                days::day08::Solution::first_star,
                days::day08::Solution::second_star,
            ),
        ),
    ]
}
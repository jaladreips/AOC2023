
use std::{fs, io};

mod days;

type SolutionFnT = fn(&str) -> String;
trait Day {
    fn first_star(input: &str) -> String;
    fn second_star(input: &str) -> String;
}

fn main() {
    let days: [(&str, (SolutionFnT, SolutionFnT)); 3] = [
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
    ];

    for (day_name, day_impl) in days {
        let Ok((first, second)) = run(day_name, day_impl) else {
            unimplemented!()
        };
        println!("[{day_name}] First answer: {first} | Second answer: {second}");
    }
}
fn run(
    dir: &str,
    solutions: (SolutionFnT, SolutionFnT),
) -> io::Result<(String, String)> {
    let mut input_file = std::env::current_exe()?;
    input_file.pop();
    input_file.pop();
    input_file.pop();
    input_file.push("src");
    input_file.push(dir);
    input_file.push("input.txt");

    let input = fs::read_to_string(&input_file);
    let input = input.unwrap_or_else(|_| panic!("Failed to read from {}",
        input_file.to_str().unwrap()));
    Ok((solutions.0(&input), solutions.1(&input)))
}

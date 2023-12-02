use std::path::{Path, PathBuf};
use std::{fs, io};

mod days;

trait Day {
    fn first_star(input: &str) -> String;
    fn second_star(input: &str) -> String;
}

fn run(
    dir: &str,
    solutions: (fn(&str) -> String, fn(&str) -> String),
) -> io::Result<(String, String)> {
    let mut input_file = std::env::current_exe()?;
    input_file.pop();
    input_file.pop();
    input_file.pop();
    input_file.push("src");
    input_file.push(dir);
    input_file.push("input.txt");

    let input = fs::read_to_string(&input_file);
    let input = input.expect(&format!(
        "Failed to read from {}",
        input_file.to_str().unwrap()
    ));
    Ok((solutions.0(&input), solutions.1(&input)))
}

fn main() {
    let days: [(&str, (fn(&str) -> String, fn(&str) -> String)); 2] = [
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
    ];

    for (day_name, day_impl) in days {
        let Ok((first, second)) = run(day_name, day_impl) else {
            unimplemented!()
        };
        println!("[{day_name}] First answer: {first} | Second answer: {second}");
    }
}

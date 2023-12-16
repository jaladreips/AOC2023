#![feature(map_many_mut)]

use std::{fs, io};

mod days;

type SolutionFnT = fn(&str) -> String;
trait Day {
    fn first_star(input: &str) -> String;
    fn second_star(input: &str) -> String;
}

fn main() {
    let days = days::days();

    for (day_name, day_impl) in if std::env::args().any(|x| x == "--last-only") {
        days[(days.len() - 1)..].iter()
    } else {
        days.iter()
    } {
        let start = std::time::Instant::now();
        let Ok((first, second)) = run(day_name, *day_impl) else {
            unimplemented!()
        };

        println!("[{day_name}] First answer: {first} | Second answer: {second} | Time elapsed: {}s", start.elapsed().as_secs_f64());
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
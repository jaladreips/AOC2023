use std::fs;
use std::path::Path;

fn main() {
    let input_file = Path::new("input.txt");
    let input = fs::read_to_string(input_file).unwrap();

    let output = first_star(&input);
    println!("First answer: {}", output);
    
    let output = second_star(&input);
    println!("Second answer: {}", output);
}

fn first_star(input: &str) -> String {
    let mut total = 0;

    for line in input.split("\n") {
        let all_cubes = line.split(":").collect::<Vec<_>>()[1];
        for cubes in all_cubes.split(";") {
            for color in cubes.split(",") {

            }
        }


        let id = line.split(":").collect::<Vec<_>>()[0].split(char::is_whitespace).collect::<Vec<_>>()[1];
    }

    total.to_string()
}

fn second_star(input: &str) -> String {
    let mut total = 0;

    total.to_string()
}

mod test {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(first_star(INPUT), "8");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(second_star(INPUT), "281");
    }
}

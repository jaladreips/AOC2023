use std::collections::HashMap;

pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        let mut total = 0;

        for line in input.split("\n") {
            let mut is_ok = true;

            let all_cubes = line.split(": ");
            let all_cubes = all_cubes.collect::<Vec<_>>()[1];

            for cubes in all_cubes.split("; ") {
                for color in cubes.split(", ") {
                    let count_color = color.split(" ");
                    let count_color: Vec<_> = count_color.collect();

                    let count = i32::from_str_radix(count_color[0], 10);
                    let count =
                        count.expect(&format!("[count] {} should be a number", count_color[0]));

                    let color = count_color[1];

                    if count > color_limits[color] {
                        is_ok = false;
                        break;
                    }
                }
            }

            if !is_ok {
                continue;
            }

            let id = line.split(":");
            let id = id.collect::<Vec<_>>()[0];
            let id = id.split(char::is_whitespace);
            let id = id.collect::<Vec<_>>()[1];
            let id = i32::from_str_radix(id, 10);
            let id = id.expect("[id] should be a number");

            total += id;
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut total = 0;

        for line in input.split("\n") {
            let mut color_requirements = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            let all_cubes = line.split(": ");
            let all_cubes = all_cubes.collect::<Vec<_>>()[1];

            for cubes in all_cubes.split("; ") {
                for color in cubes.split(", ") {
                    let count_color = color.split(" ");
                    let count_color: Vec<_> = count_color.collect();

                    let count = i32::from_str_radix(count_color[0], 10);
                    let count =
                        count.expect(&format!("[count] {} should be a number", count_color[0]));

                    let color = count_color[1];
                    let val = color_requirements.get_mut(color).expect("Unknown color?");
                    *val = if count > *val { count } else { *val }
                }
            }

            let mut power = 1;
            for v in color_requirements.values() {
                power *= v
            }

            total += power;
        }
        total.to_string()
    }
}

mod test {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "8");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "2286");
    }
}

use std::collections::HashMap;

pub struct Solution;

impl Solution {}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut total = 0;
        for partial_input in input.split("\n\n") {
            let mut v = None;
            let map = partial_input
                .split("\n")
                .enumerate()
                .flat_map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(move |(x, s)| ((x as i64, y as i64), s == '#'))
                })
                .collect::<HashMap<_, _>>();

            let size_x = *map.keys().map(|(x, _)| x).max().unwrap();
            let size_y = *map.keys().map(|(_, y)| y).max().unwrap();
            for column in 0..(size_x - 1) {
                if v.is_some() {
                    break;
                }

                let trimmed_map = map
                    .clone()
                    .into_iter()
                    .filter(|&((x, _), _)| x <= 2 * column + 1 && x > 2 * column - size_x)
                    .map(|((x, y), v)| ((x - column, y), v))
                    .map(|((x, y), v)| ((if x <= 0 { x - 1 } else { x }, y), v))
                    .collect::<HashMap<_, _>>();
                let flipped_trimmed_map = trimmed_map
                    .clone()
                    .into_iter()
                    .map(|((x, y), v)| ((-x, y), v))
                    .collect::<HashMap<_, _>>();

                if flipped_trimmed_map == trimmed_map {
                    v = Some(column + 1);
                    break;
                }
            }

            total += v.unwrap();
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        todo!()
    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;

        assert_eq!(
            test_fn(
                "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
            ),
            "405"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
    }
}

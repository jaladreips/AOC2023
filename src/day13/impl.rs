use std::collections::HashMap;

pub struct Solution;

impl Solution {}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut total = 0;
        for partial_input in input.split("\n\n") {
            let mut v = None;
            let map = partial_input
                .split('\n')
                .enumerate()
                .flat_map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(move |(x, s)| ((x as i64, y as i64), s == '#'))
                })
                .collect::<HashMap<_, _>>();

            {
                let size_x = *map.keys().map(|(x, _)| x).max().unwrap() + 1;

                for column in 0..(size_x - 1) {
                    if v.is_some() {
                        break;
                    }

                    let trimmed_map = map
                        .clone()
                        .into_iter()
                        .filter(|&((x, _), _)| x <= 2 * column + 1 && x > 2 * column + 1 - size_x)
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
            }
            {
                let size_y = *map.keys().map(|(_, y)| y).max().unwrap() + 1;
                for row in 0..(size_y - 1) {
                    if v.is_some() {
                        break;
                    }

                    let trimmed_map = map
                        .clone()
                        .into_iter()
                        .filter(|&((_, y), _)| y <= 2 * row + 1 && y > 2 * row + 1 - size_y)
                        .map(|((x, y), v)| ((x, y - row), v))
                        .map(|((x, y), v)| ((x, if y <= 0 { y - 1 } else { y }), v))
                        .collect::<HashMap<_, _>>();

                    let flipped_trimmed_map = trimmed_map
                        .clone()
                        .into_iter()
                        .map(|((x, y), v)| ((x, -y), v))
                        .collect::<HashMap<_, _>>();

                    if flipped_trimmed_map == trimmed_map {
                        v = Some(100 * (row + 1));
                        break;
                    }
                }
            }
            total += v.unwrap();
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut total = 0;
        for partial_input in input.split("\n\n") {
            let mut v = None;
            let map = partial_input
                .split('\n')
                .enumerate()
                .flat_map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(move |(x, s)| ((x as i64, y as i64), s == '#'))
                })
                .collect::<HashMap<_, _>>();

            {
                let size_x = *map.keys().map(|(x, _)| x).max().unwrap() + 1;

                for column in 0..(size_x - 1) {
                    if v.is_some() {
                        break;
                    }

                    let trimmed_map = map
                        .clone()
                        .into_iter()
                        .filter(|&((x, _), _)| x <= 2 * column + 1 && x > 2 * column + 1 - size_x)
                        .map(|((x, y), v)| ((x - column, y), v))
                        .map(|((x, y), v)| ((if x <= 0 { x - 1 } else { x }, y), v))
                        .collect::<HashMap<_, _>>();

                    let flipped_trimmed_map = trimmed_map
                        .clone()
                        .into_iter()
                        .map(|((x, y), v)| ((-x, y), v))
                        .collect::<HashMap<_, _>>();

                        let sum = trimmed_map
                        .into_iter()
                        .map(|(xy, v)| {
                            (flipped_trimmed_map
                                .get(&xy)
                                .unwrap_or_else(|| panic!("WTF {xy:?}"))
                                ^ v) as usize
                        })
                        .sum::<usize>();

                        if sum == 2
                    {
                        v = Some(column + 1);
                        break;
                    }
                }
            }
            {
                let size_y = *map.keys().map(|(_, y)| y).max().unwrap() + 1;
                for row in 0..(size_y - 1) {
                    if v.is_some() {
                        break;
                    }

                    let trimmed_map = map
                        .clone()
                        .into_iter()
                        .filter(|&((_, y), _)| y <= 2 * row + 1 && y > 2 * row + 1 - size_y)
                        .map(|((x, y), v)| ((x, y - row), v))
                        .map(|((x, y), v)| ((x, if y <= 0 { y - 1 } else { y }), v))
                        .collect::<HashMap<_, _>>();

                    let flipped_trimmed_map = trimmed_map
                        .clone()
                        .into_iter()
                        .map(|((x, y), v)| ((x, -y), v))
                        .collect::<HashMap<_, _>>();

                    let sum = trimmed_map
                    .into_iter()
                    .map(|(xy, v)| {
                        (flipped_trimmed_map
                            .get(&xy)
                            .unwrap_or_else(|| panic!("WTF {xy:?}"))
                            ^ v) as usize
                    })
                    .sum::<usize>();

                    if sum == 2
                    {
                        v = Some(100 * (row + 1));
                        break;
                    }
                }
            }
            total += v.unwrap_or_else(|| panic!("WTF\n{partial_input}\n"));
        }

        total.to_string()
    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;

        assert_eq!(
            test_fn(
                ".#....#
                #...##.
                #...##."
            ),
            "200"
        );

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
        let _test_fn = <Solution as crate::Day>::second_star;
    }
}

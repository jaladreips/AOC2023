pub struct Solution;

impl Solution {}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Rock {
    Mobile,
    Immobile,
    None,
}

impl core::fmt::Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mobile => write!(f, "O"),
            Self::Immobile => write!(f, "#"),
            Self::None => write!(f, "."),
        }
    }
}

impl Ord for Rock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Rock::Mobile, Rock::Immobile) => Equal,
            (Rock::Mobile, Rock::None) => Less,
            (Rock::Immobile, Rock::Mobile) => Equal,
            (Rock::Immobile, Rock::None) => Equal,
            (Rock::None, Rock::Mobile) => Greater,
            (Rock::None, Rock::Immobile) => Equal,
            _ => Equal,
        }
    }
}

impl PartialOrd for Rock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bubble_sort<T: std::cmp::PartialOrd>(vec_x: &mut Vec<T>) {
    loop {
        let mut changed = false;
        for i in 0..(vec_x.len() - 1) {
            let [x, y] = vec_x.get_many_mut([i, i+1]).unwrap();
            if x > y {
                changed = true;

                std::mem::swap(x, y);
            }
        }

        if !changed {
            break;
        }
    }
}
impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut rock_map = input
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().map(move |(x, s)| {
                    (
                        (x, y),
                        match s {
                            '#' => Rock::Immobile,
                            'O' => Rock::Mobile,
                            _ => Rock::None,
                        },
                    )
                })
            })
            .fold(vec![vec![]], |mut res, ((x, y), rock)| {
                if x >= res.len() {
                    res.resize(x + 1, vec![]);
                }

                if y >= res[x].len() {
                    res[x].resize(y + 1, Rock::None);
                }
                res[x][y] = rock;
                res
            });

        let mut total = 0;

        while let Some(mut rock_column) = rock_map.pop() {
            bubble_sort(&mut rock_column);
            for (i, rock) in rock_column.iter().enumerate() {
                if rock == &Rock::Mobile {
                    total += rock_column.len() - i;
                }

            }
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut rock_map = input
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().map(move |(x, s)| {
                    (
                        (x, y),
                        match s {
                            '#' => Rock::Immobile,
                            'O' => Rock::Mobile,
                            _ => Rock::None,
                        },
                    )
                })
            })
            .fold(vec![vec![]], |mut res, ((x, y), rock)| {
                if x >= res.len() {
                    res.resize(x + 1, vec![]);
                }

                if y >= res[x].len() {
                    res[x].resize(y + 1, Rock::None);
                }
                res[x][y] = rock;
                res
            });

        let mut cache = std::collections::HashMap::new();
        let mut current_cycle = 0;
        let num_cycles = 1000000000_usize; // 1000000000_usize;
        for i in 0..num_cycles {
            if current_cycle >= num_cycles {
                break;
            }
            
            if let Some(&cycle_start) = cache.get(&rock_map) {
                let cycle_length = current_cycle - cycle_start;
                current_cycle += ((num_cycles - 1 - current_cycle) / cycle_length) * cycle_length;
            } else {
                cache.insert(rock_map.clone(), i);
            }
            
            if current_cycle == num_cycles {
                break;
            }

            for _ in 0..4 {
                let mut len = None;
                for rock_column in rock_map.iter_mut() {
                    bubble_sort(rock_column);
                    rock_column.reverse();
                    if len.is_none() {
                        len = Some(rock_column.len());
                    }
                }

                let mut new_rock_map = vec![vec![Rock::None; rock_map.len()]; len.unwrap()];

                for new_rock_column in new_rock_map.iter_mut().rev() {
                    for (column, rock_column) in rock_map.iter_mut().enumerate() {
                        if let Some(rock) = rock_column.pop() {
                            new_rock_column[column] = rock;
                        } else {
                            panic!("WTF");
                        }
                    }
                }

                rock_map = new_rock_map;
            }

            current_cycle += 1;
        }

        let mut total = 0;

        while let Some(rock_column) = rock_map.pop() {
            for (i, rock) in rock_column.iter().enumerate() {
                if rock == &Rock::Mobile {
                    total += rock_column.len() - i;
                }

            }
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
                "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
            ),
            "136"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;

        assert_eq!(test_fn("O"), "1");
        assert_eq!(test_fn("#"), "0");
        assert_eq!(test_fn("."), "0");

        assert_eq!(
            test_fn(
                ".....
            .###.
            .#O#.
            .#.##
            ....."
            ),
            "1"
        );

        assert_eq!(
            test_fn(
                ".#.
            .#.
            O#."
            ),
            "1"
        );

        assert_eq!(
            test_fn(
                ".#.
            ###
            .#O"
            ),
            "1"
        );

        assert_eq!(
            test_fn(
           "##
            O."
            ),
            "1"
        );

        assert_eq!(
            test_fn(
           "OO
            OO"
            ),
            "6"
        );

        assert_eq!(
            test_fn(
           "O.
            .."
            ),
            "1"
        );

        assert_eq!(
            test_fn(
           "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
            ),
            "64"
        );
    }
}

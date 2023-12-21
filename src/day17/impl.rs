use std::collections::HashMap;

pub struct Solution;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let map = input
            .split('\n')
            .map(|x| x.trim())
            .enumerate()
            .flat_map(|(i, x)| {
                x.chars().enumerate().map(move |(j, y)| {
                    (
                        Point {
                            x: j as i64,
                            y: i as i64,
                        },
                        y.to_digit(10).unwrap()
                    )
                })
            })
            .collect::<HashMap<_, _>>();


        todo!()
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
                r"2413432311323
                  3215453535623
                  3255245654254
                  3446585845452
                  4546657867536
                  1438598798454
                  4457876987766
                  3637877979653
                  4654967986887
                  4564679986453
                  1224686865563
                  2546548887735
                  4322674655533"
            ),
            "102"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
    }
}

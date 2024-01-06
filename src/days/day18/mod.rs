

use super::helpers::*;
pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let pattern = regex::Regex::new(r"(\w) (\d+) \(\S(\S{6})\)").unwrap();
        let mut prev_point = Point::from(0, 0);
        let mut main_area = 0;
        let mut extra_area = 0;
        let mut corners_area = 0;
        let mut prev_dir = Dir::Up;
        for line in input.lines() {
            let captures = pattern.captures(line).unwrap();
            let dir = match captures[1].chars().next().unwrap() {
                'R' => Dir::Right,
                'D' => Dir::Down,
                'L' => Dir::Left,
                'U' => Dir::Up,
                _ => unimplemented!(),
            };

            let count = captures[2].parse::<usize>().unwrap();

            let point = prev_point + dir.to_point() * count as i64;

            main_area += point.y * prev_point.x - point.x * prev_point.y;
            extra_area += count as i64;
            match (prev_dir, dir) {
                (Dir::Down, Dir::Left) => corners_area += 1,
                (Dir::Down, Dir::Right) => corners_area -= 1,
                (Dir::Up, Dir::Left) => corners_area -= 1,
                (Dir::Up, Dir::Right) => corners_area += 1,
                (Dir::Left, Dir::Down) => corners_area -= 1,
                (Dir::Left, Dir::Up) => corners_area += 1,
                (Dir::Right, Dir::Down) => corners_area += 1,
                (Dir::Right, Dir::Up) => corners_area -= 1,
                _ => unimplemented!()
            }
            prev_point = point;
            prev_dir = dir;
        }

        let area = main_area / 2 + extra_area / 2 + corners_area / 4;
        
        area.to_string()
    }

    fn second_star(input: &str) -> String {
        let pattern = regex::Regex::new(r"\w \d+ \S\S(\S{5})(\d)\S").unwrap();
        let mut prev_point = Point::from(0, 0);
        let mut prev_dir = Dir::Up;
        let mut area = 0;
        for line in input.lines() {
            let captures = pattern.captures(line).unwrap();
            let dir = match captures[2].chars().next().unwrap() {
                '0' => Dir::Right,
                '1' => Dir::Down,
                '2' => Dir::Left,
                '3' => Dir::Up,
                _ => unimplemented!(),
            };

            let count = i64::from_str_radix(&captures[1], 16).unwrap();

            let point = prev_point + dir.to_point() * count;

            area += 2 * point.y * prev_point.x - 2 * point.x * prev_point.y;
            area += 2 * count;
            match (prev_dir, dir) {
                (Dir::Down, Dir::Left) => area += 1,
                (Dir::Down, Dir::Right) => area -= 1,
                (Dir::Up, Dir::Left) => area -= 1,
                (Dir::Up, Dir::Right) => area += 1,
                (Dir::Left, Dir::Down) => area -= 1,
                (Dir::Left, Dir::Up) => area += 1,
                (Dir::Right, Dir::Down) => area += 1,
                (Dir::Right, Dir::Up) => area -= 1,
                _ => unimplemented!()
            }
            prev_point = point;
            prev_dir = dir;
        }

        area /= 4;
        
        area.to_string()
    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;
        assert_eq!(
            test_fn(
                r"R 1 (#70c710)
                  D 1 (#0dc571)
                  L 1 (#5713f0)
                  U 1 (#d2c081)"
            ),
            "4"
        );
        assert_eq!(
            test_fn(
                r"R 6 (#70c710)
                  D 5 (#0dc571)
                  L 2 (#5713f0)
                  D 2 (#d2c081)
                  R 2 (#59c680)
                  D 2 (#411b91)
                  L 5 (#8ceee2)
                  U 2 (#caa173)
                  L 1 (#1b58a2)
                  U 2 (#caa171)
                  R 2 (#7807d2)
                  U 3 (#a77fa3)
                  L 2 (#015232)
                  U 2 (#7a21e3)"
            ),
            "62"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
        assert_eq!(
            test_fn(
                r"R 6 (#70c710)
                  D 5 (#0dc571)
                  L 2 (#5713f0)
                  D 2 (#d2c081)
                  R 2 (#59c680)
                  D 2 (#411b91)
                  L 5 (#8ceee2)
                  U 2 (#caa173)
                  L 1 (#1b58a2)
                  U 2 (#caa171)
                  R 2 (#7807d2)
                  U 3 (#a77fa3)
                  L 2 (#015232)
                  U 2 (#7a21e3)"
            ),
            "952408144115"
        );
    }
}

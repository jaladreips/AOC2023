use std::{cell::RefCell, collections::HashSet, rc::Rc};

use super::helpers::*;
pub struct Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path<const MIN: usize, const MAX: usize> {
    points: Vec<Point>,
    current_cost: usize,
    cache: Rc<RefCell<HashSet<[Point; MAX]>>>,
}

impl<const MIN: usize, const MAX: usize> Ord for Path<MIN, MAX> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.current_cost.cmp(&self.current_cost)
    }
}

impl<const MIN: usize, const MAX: usize> PartialOrd for Path<MIN, MAX> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const MIN: usize, const MAX: usize> Path<MIN, MAX> {
    fn new() -> Self {
        Self {
            points: Vec::from([Point::from(0, 0)]),
            current_cost: 0,
            cache: Rc::from(RefCell::from(HashSet::new())),
        }
    }

    fn propose_points(&self) -> Vec<Point> {
        let len = self.points.len();
        let mut dirs = Vec::from([Dir::Up, Dir::Down, Dir::Left, Dir::Right]);
        let current_point = *self.points.last().unwrap();

        if len > 1 {
            let current_dir = current_point - self.points[len - 2];
            let current_dir = current_dir.to_dir();
            let reverse_dir = current_dir.reverse();
            dirs.retain(|d| *d != reverse_dir);

            if MIN > 0
                && (len <= MIN
                    || self.points[len - MIN]
                        != current_point + reverse_dir.to_point() * (MIN - 1) as i64)
            {
                dirs = Vec::from([current_dir]);
            }

            let p = current_point + reverse_dir.to_point() * (MAX - 1 )as i64;

            if len >= MAX && self.points[len - MAX] == p {
                dirs.retain(|d| *d != current_dir);
            }
        }

        dirs.into_iter()
            .map(|d| current_point + d.to_point())
            .collect()
    }

    fn process_point(&self, p: Point, cost: usize) -> Option<Self> {
        let mut path = self.clone();
        path.points.push(p);
        path.current_cost += cost;

        if path.points.len() < MAX {
            return Some(path);
        }
        let last_points = &path.points[path.points.len() - MAX..path.points.len()];
        let last_points: [Point; MAX] = last_points.try_into().unwrap();

        if path.cache.borrow().contains(&last_points) {
            return None;
        }

        path.cache.try_borrow_mut().unwrap().insert(last_points);
        Some(path)
    }

    fn is_done(&self, dest: Point) -> bool {
        let len = self.points.len();
        let current_point = *self.points.last().unwrap();

        if current_point != dest {
            return false;
        }

        if len > 1 {
            let current_dir = current_point - self.points[len - 2];
            let current_dir = current_dir.to_dir();
            let reverse_dir = current_dir.reverse();

            if MIN > 0
                && (len <= MIN
                    || self.points[len - MIN]
                        != current_point + reverse_dir.to_point() * (MIN - 1) as i64)
            {
                return false;
            }
        }

        true
    }
}

impl Solution {
    fn solve<const MIN: usize, const MAX: usize>(input: &str) -> String {
        let map = input_to_2d_map(input, |c| c.to_digit(10).unwrap() as usize);
        let dest = *map.keys().max().unwrap();
        let mut paths = Vec::from([Path::<MIN, MAX>::new()]);

        while let Some(path) = paths.pop() {
            let points = path
                .propose_points()
                .into_iter()
                .filter(|p| map.get(p).is_some());

            let new_paths = points
                .filter_map(|p| Path::process_point(&path, p, *map.get(&p).unwrap()))
                .collect::<Vec<_>>();

            if let Some(p) = new_paths.iter().find(|&p| p.is_done(dest)) {
                return p.current_cost.to_string();
            }

            paths.extend(new_paths);
            paths.sort();
        }

        unimplemented!()
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        Solution::solve::<0, 4>(input)
    }

    fn second_star(input: &str) -> String {
        Solution::solve::<5, 11>(input)
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
        assert_eq!(
            test_fn(
                r"111111111111
                  999999999991
                  999999999991
                  999999999991
                  999999999991"
            ),
            "71"
        );

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
            "94"
        );

    }
}

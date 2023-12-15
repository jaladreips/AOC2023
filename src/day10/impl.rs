use std::collections::HashMap;

use regex::Regex;

pub struct Solution;

#[allow(non_snake_case)]
mod Dir {
    pub(super) const NONE: u8 = 0;
    pub(super) const N: u8 = 1 << 0;
    pub(super) const S: u8 = 1 << 1;
    pub(super) const W: u8 = 1 << 2;
    pub(super) const E: u8 = 1 << 3;
    pub(super) const ALL: u8 = N | W | S | E;
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let dir_map = HashMap::from([
            ('S', Dir::ALL),
            ('|', Dir::N | Dir::S),
            ('-', Dir::E | Dir::W),
            ('L', Dir::N | Dir::E),
            ('J', Dir::N | Dir::W),
            ('7', Dir::S | Dir::W),
            ('F', Dir::S | Dir::E),
            ('.', Dir::NONE),
        ]);

        let map = input
            .split('\n')
            .map(|x| x.trim())
            .enumerate()
            .flat_map(|(i, x)| {
                let dir_map = dir_map.clone();
                x.chars().enumerate().map(move |(j, y)| {
                    let dir = dir_map[&y];
                    (
                        (i as i64, j as i64),
                        (dir, if dir == Dir::ALL { 0 } else { u64::MAX }),
                    )
                })
            });

        let mut map: HashMap<_, _> = HashMap::from_iter(map);

        let mut current_nodes = map
            .iter()
            .filter(|x| (x.1 .0 & Dir::ALL) == Dir::ALL)
            .map(|x| *x.0)
            .collect::<Vec<_>>();

        while let Some(node) = current_nodes.pop() {
            let (src_dirs, src_dist) = map[&node];
            for (d, src_dir, dst_dir) in [
                ((1, 0), Dir::S, Dir::N),
                ((-1, 0), Dir::N, Dir::S),
                ((0, 1), Dir::E, Dir::W),
                ((0, -1), Dir::W, Dir::E),
            ] {
                let mb_next_node = (node.0 + d.0, node.1 + d.1);
                if let Some((dst_dirs, dist)) = map.get_mut(&mb_next_node) {
                    if *dist > src_dist + 1 && src_dirs & src_dir > 0 && *dst_dirs & dst_dir > 0 {
                        *dist = src_dist + 1;
                        current_nodes.push(mb_next_node);
                    }
                }
            }
        }

        let mut distances = map.values().filter(|(dir, dist)| *dist < u64::MAX).map(|(dir, dist)| *dist).collect::<Vec<_>>();
        distances.sort();
        distances.last().unwrap().to_string()        
    }

    fn second_star(input: &str) -> String {
        todo!()
    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(".....
        .S-7.
        .|.|.
        .L-J.
        ....."), "4");

        assert_eq!(<Solution as crate::Day>::first_star("..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ..."), "8");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star("...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."), "4");

        assert_eq!(<Solution as crate::Day>::second_star(".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."), "8");

        assert_eq!(<Solution as crate::Day>::second_star("FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"), "10");
    }
}

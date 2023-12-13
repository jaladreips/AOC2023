use std::collections::HashMap;

use regex::Regex;

pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut current = "AAA".to_string();
        let pattern = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").expect("WTF regex");

        let map: std::collections::HashMap<String, (String, String)> =
            std::collections::HashMap::from_iter(
                pattern
                    .captures_iter(input)
                    .map(|capture| {
                        (
                            capture[1].to_string(),
                            (capture[2].to_string(), capture[3].to_string()),
                        )
                    })
                    .collect::<Vec<_>>(),
            );

        let mut lines = input.split('\n');
        let directions = lines.next().expect("WTF line");
        let mut steps = 0;

        loop {
            for dir in directions.chars() {
                if dir == 'L' {
                    current = map[&current].0.clone();
                    steps += 1;
                } else if dir == 'R' {
                    current = map[&current].1.clone();
                    steps += 1;
                }

                if current == "ZZZ" {
                    break;
                }
            }

            if current == "ZZZ" {
                break;
            }
        }

        steps.to_string()
    }

    fn second_star(input: &str) -> String {
        let pattern = Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)").expect("WTF regex");

        let str_map: std::collections::HashMap<String, (String, String)> =
            std::collections::HashMap::from_iter(
                pattern
                    .captures_iter(input)
                    .map(|capture| {
                        (
                            capture[1].to_string(),
                            (capture[2].to_string(), capture[3].to_string()),
                        )
                    })
                    .collect::<Vec<_>>(),
            );

        let num_to_str: std::collections::HashMap<usize, _> =
            std::collections::HashMap::from_iter(str_map.keys().enumerate());

        let str_to_num: std::collections::HashMap<_, usize> =
            std::collections::HashMap::from_iter(num_to_str.iter().map(|x| (*x.1, *x.0)));

        let starts: std::collections::HashSet<_> = std::collections::HashSet::from_iter(
            str_to_num
                .iter()
                .filter(|x| x.0.ends_with('A'))
                .map(|x| *x.1),
        );

        let ends: std::collections::HashSet<_> = std::collections::HashSet::from_iter(
            str_to_num
                .iter()
                .filter(|x| x.0.ends_with('Z'))
                .map(|x| *x.1),
        );

        let mut num_map = vec![(0, 0); str_map.len()];

        for i in 0..str_map.len() {
            let strs = &str_map[num_to_str[&i]];
            num_map[i] = (str_to_num[&strs.0], str_to_num[&strs.1]);
        }

        let mut lines = input.split('\n');
        let directions = lines.next().expect("WTF line");
        let directions = directions.trim().chars().collect::<Vec<_>>();

        let dir_size = directions.len();

        let mut jump_map: HashMap<(usize, usize), _> = HashMap::new();

        for &end in ends.iter().chain(starts.iter()) {
            for dir in 0..dir_size {
                let mut current = end;
                jump_map.insert((dir, end), (0_usize, current));
                let step = jump_map.get_mut(&(dir, end)).expect("WTF just inserted it");
                let mut history = std::collections::HashSet::new();
                loop {
                    if !history.insert((step.0 % dir_size, current)) {
                        break;
                    }
                    current = match directions[(dir + step.0) % dir_size] {
                        'L' => num_map[current].0,
                        'R' => num_map[current].1,
                        _ => {
                            unimplemented!()
                        }
                    };

                    step.0 += 1;
                    step.1 = current;
                    if ends.contains(&current) {
                        break;
                    }
                }
            }
        }

        let mut all_paths = starts.iter().map(|&x| (0_usize, x)).collect::<Vec<_>>();
        loop {
            let min = all_paths.iter_mut().min().expect("WTF min");
            let jump = jump_map[&(min.0 % dir_size, min.1)];
            min.0 += jump.0;
            min.1 = jump.1;

            let steps = min.0;

            if all_paths.iter().all(|x| x.0 == steps) {
                break steps;
            }
        }
        .to_string()
    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(
            <Solution as crate::Day>::first_star(
                "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)"
            ),
            "2"
        );

        assert_eq!(
            <Solution as crate::Day>::first_star(
                "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"
            ),
            "6"
        );
    }

    #[test]
    fn second_star() {
        use super::*;

        assert_eq!(
            <Solution as crate::Day>::second_star(
                "LR

                11A = (11B, XXX)
                11B = (XXX, 11Z)
                11Z = (11B, XXX)
                22A = (22B, XXX)
                22B = (22C, 22C)
                22C = (22Z, 22Z)
                22Z = (22B, 22B)
                XXX = (XXX, XXX)"
            ),
            "6"
        );
    }
}

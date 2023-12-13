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
        todo!()
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

pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut total = 0;
        for line in input.split('\n') {
            let mut first = 0xFFFFFFFF;
            let mut last = 0xFFFFFFFF;
            for char in line.chars() {
                match char.to_digit(10) {
                    Some(d) => {
                        first = if first == 0xFFFFFFFF { d } else { first };
                        last = d;
                    }
                    None => {}
                }
            }

            total += first * 10 + last;
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut total = 0;

        use std::collections::HashMap;

        let str2val = HashMap::from([
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        for line in input.split('\n') {
            let mut first = None;
            let mut last = None;

            for k in str2val.keys() {
                let mut digits: Vec<_> = line.match_indices(k).collect();

                if digits.is_empty() {
                    continue;
                }

                digits.sort();

                match first {
                    None => {
                        first = Some(digits[0]);
                    }
                    Some(v) => {
                        first = if v.0 > digits[0].0 {
                            Some(digits[0])
                        } else {
                            first
                        }
                    }
                };

                match last {
                    None => {
                        last = Some(digits[digits.len() - 1]);
                    }
                    Some(v) => {
                        last = if v.0 < digits[digits.len() - 1].0 {
                            Some(digits[digits.len() - 1])
                        } else {
                            last
                        }
                    }
                };
            }

            let num = str2val[first.unwrap().1] * 10 + str2val[last.unwrap().1];
            total += num;
        }

        total.to_string()
    }
}

mod test {
    #[test]

    fn first_star() {
        use super::*;
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(<Solution as crate::Day>::first_star(input), "142");
    }

    #[test]
    fn second_star() {
        use super::*;
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(<Solution as crate::Day>::second_star(input), "281");
    }
}

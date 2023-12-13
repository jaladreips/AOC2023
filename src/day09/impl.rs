use regex::Regex;

pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        todo!()
    }

    fn second_star(input: &str) -> String {
        todo!()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = 
    "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "114");
    }

    #[test]
    fn second_star() {
        use super::*;
        todo!()
    }
}

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
    const INPUT1: &str = 
   "7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ";

    #[allow(dead_code)]
    const INPUT2: &str = 
   "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(INPUT1), "4");
        assert_eq!(<Solution as crate::Day>::first_star(INPUT2), "8");
    }

    #[test]
    fn second_star() {
        use super::*;
    }
}

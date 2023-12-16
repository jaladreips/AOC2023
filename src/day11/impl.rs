use std::collections::HashMap;

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
    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;
        assert_eq!(
            test_fn(
                "...#......
                .......#..
                #.........
                ..........
                ......#...
                .#........
                .........#
                ..........
                .......#..
                #...#....."
            ),
            "4"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
    }
}

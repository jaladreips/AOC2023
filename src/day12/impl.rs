pub struct Solution;

impl Solution {
}

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
        assert_eq!(test_fn("???.### 1,1,3"), "1");
        assert_eq!(test_fn(".??..??...?##. 1,1,3"), "4");
        assert_eq!(test_fn("?#?#?#?#?#?#?#? 1,3,1,6"), "1");
        assert_eq!(test_fn("????.#...#... 4,1,1"), "1");
        assert_eq!(test_fn("????.######..#####. 1,6,5"), "4");
        assert_eq!(test_fn("?###???????? 3,2,1"), "10");
        assert_eq!(
            test_fn(
                "???.### 1,1,3
                .??..??...?##. 1,1,3
                ?#?#?#?#?#?#?#? 1,3,1,6
                ????.#...#... 4,1,1
                ????.######..#####. 1,6,5
                ?###???????? 3,2,1"
            ),
            "21"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
    }
}

pub struct Solution;

impl Solution {
    fn stars(input: &str, expansion_rate: i64) -> String {
        let mut map = input
        .split('\n')
        .map(|x| x.trim())
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i64, j as i64), c))
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| x)
        })
        .collect::<Vec<_>>();
    
    let max_dim_x = *map.iter().map(|(x, _)| x).max().unwrap() + 1;
    let max_dim_y = *map.iter().map(|(_, y)| y).max().unwrap() + 1;
    
    for i in (0..std::cmp::max(max_dim_x, max_dim_y)).rev() {
        if !map.iter().any(|(x, _)| *x == i) {
            for (x, _) in map.iter_mut().filter(|(x, _)| *x > i) {
                *x += expansion_rate - 1;
            }
        }
    
        if !map.iter().any(|(_, y)| *y == i) {
            for (_, y) in map.iter_mut().filter(|(_, y)| *y > i) {
                *y += expansion_rate - 1;
            }
        }
    }
    
    let total = map.iter()
        .flat_map(|x| map.iter().map(move |y| (x, y)))
        .fold(0, |acc, ((x1, y1), (x2, y2))| {
            acc + (x2 - x1).abs() + (y2 - y1).abs()
        }) >> 1;
    
    total.to_string()
    }

}

impl crate::Day for Solution {


    fn first_star(input: &str) -> String {
        Solution::stars(input, 2)
    }

    fn second_star(input: &str) -> String {
        Solution::stars(input, 1000000)
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
            "374"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = Solution::stars;
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
            , 10),
            "1030"
        );
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
            , 100),
            "8410"
        );
    }
}

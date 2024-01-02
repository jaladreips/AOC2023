use regex::Regex;

pub struct Solution;

#[derive(Debug)]
struct Boat {
    wind_up: u64,
    total_time: u64,
}

impl Boat {
    fn distance(&self) -> u64 {
        (self.total_time - self.wind_up) * self.wind_up
    }

    fn new_max_distance(total_time: u64) -> Self {
        Boat {
            wind_up: total_time / 2,
            total_time,
        }
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let pattern = Regex::new(r"(\d+)").expect("WTF pattern");

        let mut lines = input.split('\n');
        let times_line = lines.next().expect("WTF time");
        let distances_line = lines.next().expect("WTF distances");

        let iter = std::iter::zip(pattern.find_iter(times_line), pattern.find_iter(distances_line))
            .map(|x| (x.0.as_str().parse::<u64>().expect("WTF parse"), x.1.as_str().parse::<u64>().expect("WTF parse")));

        let mut total = 1;
        for (time, distance) in iter {
            let best_boat = Boat::new_max_distance(time);
            let mut deviation = 0;
            let max_deviation = loop {
                let boat = Boat { wind_up: best_boat.wind_up + deviation, total_time: time};
                deviation += 1;

                if boat.distance() <= distance {
                    break deviation - 1
                }
            };

            total *= max_deviation - 1 - (time % 2) + std::cmp::min(max_deviation, time);
        }
        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let pattern = Regex::new(r"(\d+)").expect("WTF pattern");

        let spaceless_input = input.replace(' ', "");
        let mut lines = spaceless_input.split('\n');
        let times_line = lines.next().expect("WTF time");
        let distances_line = lines.next().expect("WTF distances");

        let iter = std::iter::zip(pattern.find_iter(times_line), pattern.find_iter(distances_line))
            .map(|x| (x.0.as_str().parse::<u64>().expect("WTF parse"), x.1.as_str().parse::<u64>().expect("WTF parse")));

        let mut total = 1;
        for (time, distance) in iter {
            let best_boat = Boat::new_max_distance(time);
            let mut deviation = 0;
            let max_deviation = loop {
                let boat = Boat { wind_up: best_boat.wind_up + deviation, total_time: time};
                deviation += 1;

                if boat.distance() <= distance {
                    break deviation - 1
                }
            };

            total *= max_deviation - 1 - (time % 2) + std::cmp::min(max_deviation, time);
        }
        total.to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star("Time: 7\nDistance: 9"), "4");
        assert_eq!(<Solution as crate::Day>::first_star("Time: 15\nDistance: 40"), "8");
        assert_eq!(<Solution as crate::Day>::first_star("Time: 30\nDistance: 200"), "9");
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "288");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "71503");
    }
}

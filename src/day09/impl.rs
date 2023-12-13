use regex::Regex;

pub struct Solution;

struct Series {
    elements: Vec<i64>,
}

impl Series {
    fn extrapolate(&self) -> Self {
        let mut elements = self.elements.clone();
        elements.push(if self.is_zeros() {
            0
        } else {
            self.elements.last().unwrap() + self.get_subseries().extrapolate().elements.last().unwrap()
        });
        Series { elements }
    }

    fn get_subseries(&self) -> Self {
        Series {
            elements: self.elements[1..]
                .iter()
                .enumerate()
                .map(|(i, x)| x - self.elements[i])
                .collect(),
        }
    }

    fn is_zeros(&self) -> bool {
        self.elements.iter().all(|&x| x == 0)
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let lines = input.split('\n').map(|x| x.trim());
        let pattern = Regex::new(r"\-?\d+").expect("WTF regex");
        let mut total = 0_i64;
        for line in lines {
            let series = Series {
                elements: pattern
                .find_iter(line)
                .map(|x| x.as_str().parse().expect("WTF series"))
                .collect(),
            };
            total += series.extrapolate().elements.last().expect("WTF last");
        }
        
        total.to_string()
    }
    
    fn second_star(input: &str) -> String {
        let lines = input.split('\n').map(|x| x.trim());
        let pattern = Regex::new(r"\-?\d+").expect("WTF regex");
        let mut total = 0_i64;
        for line in lines {
            let series = Series {
                elements: pattern
                    .find_iter(line)
                    .map(|x| x.as_str().parse().expect("WTF series"))
                    .collect::<Vec<_>>().into_iter().rev().collect(),
            };
            total += series.extrapolate().elements.last().expect("WTF last");
        }
        
        total.to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "0 3 6 9 12 15
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
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "2");
    }
}

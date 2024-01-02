use std::collections::HashSet;

use regex::Regex;

pub struct Solution {}

impl Solution {
    fn number_of_matches(card: &str) -> u32 {
        let numbers = card.split(':');
        let numbers = numbers.collect::<Vec<_>>()[1];

        let numbers_pattern = Regex::new(r"(\d+)").unwrap();

        let winning_numbers = numbers.split('|').collect::<Vec<_>>()[0];
        let winning_numbers = Regex::find_iter(&numbers_pattern, winning_numbers);
        let winning_numbers = winning_numbers.map(|x| x.as_str().to_owned());
        let winning_numbers: HashSet<String> = HashSet::from_iter(winning_numbers);

        let generated_numbers = numbers.split('|').collect::<Vec<_>>()[1];
        let generated_numbers = Regex::find_iter(&numbers_pattern, generated_numbers);
        let generated_numbers = generated_numbers.map(|x| x.as_str().to_owned());
        let generated_numbers: HashSet<String> = HashSet::from_iter(generated_numbers);

        let mut score = 0;
        for n in generated_numbers {
            if winning_numbers.contains(&n) {
                score += 1;
            }
        }

        score
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut total = 0;
        for line in input.split('\n') {
            let score = Solution::number_of_matches(line);

            total += if score == 0 { 0 } else { 2_u32.pow(score - 1) };
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut total = 0;

        #[derive(Clone)]
        struct CardWithCopies<'a> {
            card: &'a str,
            copies: u32,
        }

        let cards = input.split('\n');
        let cards = cards.map(|x| CardWithCopies { card: x, copies: 1 });
        let cards = cards.collect::<Vec<_>>();
        let mut cards_with_copies = cards.clone();
        for (i, card) in cards.iter().enumerate() {
            let copies = cards_with_copies[i].copies;
            total += copies;

            let score = Solution::number_of_matches(card.card);
            for j in i..(i + score as usize) {
                if j + 1 == cards.len() {
                    break;
                }

                cards_with_copies[j + 1].copies += copies;
            }
        }

        total.to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "13");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "30");
    }
}

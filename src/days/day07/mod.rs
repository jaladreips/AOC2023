pub struct Solution;

const LABELS_NO_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const LABELS_JOKER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Hand {
    cards: [char; 5],
    bid: u64,
    implements_joker: bool,
}

impl Hand {
    fn get_type(&self) -> u8 {
        let card_set = std::collections::HashSet::from(self.cards);
        let mut local_cards = self.cards;
        if self.implements_joker {
            let mut repetitions = card_set
                .iter()
                .map(|&card| {
                    self.cards.iter().fold((0, card), |acc, &c| {
                        (acc.0 + if c == card && c != 'J' { 1 } else { 0 }, card)
                    })
                })
                .collect::<Vec<_>>();

            repetitions.sort();
            repetitions.reverse();
            local_cards = local_cards.map(|x| {
                if x == 'J' {
                    repetitions.first().expect("WTF repetitions").1
                } else {
                    x
                }
            });
        }

        let card_set = std::collections::HashSet::from(local_cards);
        let mut repetitions = card_set
            .iter()
            .map(|&card| {
                local_cards
                    .iter()
                    .fold(0, |acc, &c| acc + if c == card { 1 } else { 0 })
            })
            .collect::<Vec<_>>();

        repetitions.sort();
        repetitions.reverse();

        static KINDS: [&[u8]; 6] = [
            &[5],
            &[4, 1],
            &[3, 2],
            &[3, 1, 1],
            &[2, 2, 1],
            &[2, 1, 1, 1],
        ];

        let mut score = u8::MAX;
        let _ = &for &kind in KINDS.iter() {
            if kind == repetitions {
                return score;
            }
            score -= 1;
        };
        score
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_type() != other.get_type() {
            return self.get_type().cmp(&other.get_type());
        }

        for (card, other_card) in std::iter::zip(self.cards, other.cards) {
            if card == other_card {
                continue;
            }

            for label in if self.implements_joker {
                LABELS_JOKER
            } else {
                LABELS_NO_JOKER
            } {
                if label == card {
                    return std::cmp::Ordering::Greater;
                } else if label == other_card {
                    return std::cmp::Ordering::Less;
                }
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let pattern = regex::Regex::new(r"(\w+) (\d+)").expect("WTF regex");
        let mut hands = pattern
            .captures_iter(input)
            .map(|x| Hand {
                cards: x[1]
                    .chars()
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("WTF array"),
                bid: x[2].parse().expect("WTF bid"),
                implements_joker: false,
            })
            .collect::<Vec<_>>();

        hands.sort();

        let mut total = 0;
        for (i, &hand) in hands.iter().enumerate() {
            total += (i as u64 + 1) * hand.bid;
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let pattern = regex::Regex::new(r"(\w+) (\d+)").expect("WTF regex");
        let mut hands = pattern
            .captures_iter(input)
            .map(|x| Hand {
                cards: x[1]
                    .chars()
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("WTF array"),
                bid: x[2].parse().expect("WTF bid"),
                implements_joker: true,
            })
            .collect::<Vec<_>>();

        hands.sort();

        let mut total = 0;
        for (i, &hand) in hands.iter().enumerate() {
            total += (i as u64 + 1) * hand.bid;
        }

        total.to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star("22222 1"), "1");
        assert_eq!(<Solution as crate::Day>::first_star("22222 2"), "2");
        assert_eq!(
            <Solution as crate::Day>::first_star("22222 2 AAAAA 3"),
            (2 + 2 * 3).to_string()
        );
        assert_eq!(
            <Solution as crate::Day>::first_star("3333A 3 3333K 2"),
            (2 + 2 * 3).to_string()
        );
        assert_eq!(
            <Solution as crate::Day>::first_star("3333K 3 3333A 2"),
            (3 + 2 * 2).to_string()
        );
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "6440");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "5905");
    }
}

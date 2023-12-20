pub struct Solution;

impl Solution {
    fn hash(input: &str) -> usize {
        let mut total = 0;
        for c in input.chars() {
            total += c.to_ascii_lowercase() as usize;
            total *= 17;
            total %= 256;
        }
        total
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lens {
    label: String,
    focal: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Box {
    lenses: Vec<Lens>,
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        input
            .split(',')
            .map(|x| x.trim())
            .map(Solution::hash)
            .sum::<usize>()
            .to_string()
    }

    fn second_star(input: &str) -> String {
        let pattern = regex::Regex::new(r"(\w+)(\S)(\S?)").unwrap();

        let mut boxes = vec![Box { lenses: Vec::new() }; 256];
        for capture in regex::Regex::captures_iter(&pattern, input) {
            let lenses = &mut boxes.get_mut(Solution::hash(&capture[1])).unwrap().lenses;
            let index = lenses
                .iter()
                .enumerate()
                .filter_map(|(i, lens)| {
                    if lens.label == capture[1] {
                        Some(i)
                    } else {
                        None
                    }
                })
                .next();
            match capture[2].chars().next().unwrap() {
                '=' => {
                    if let Some(index) = index {
                        lenses[index] = Lens {
                            label: capture[1].to_string(),
                            focal: capture[3].parse::<u8>().unwrap(),
                        };
                    } else {
                        lenses.push(Lens {
                            label: capture[1].to_string(),
                            focal: capture[3].parse::<u8>().unwrap(),
                        });
                    }
                }
                '-' => {
                    if let Some(index) = index {
                        lenses.remove(index);
                    };
                }
                _ => {
                    panic!()
                }
            }
        }

        boxes
            .into_iter()
            .enumerate()
            .flat_map(|(i, b)| b.lenses.into_iter().enumerate().map(move |(slot, lens)| (i + 1) * (slot + 1) * lens.focal as usize))
            .sum::<usize>()
            .to_string()
    }
}

mod test {
    #[test]
    fn hash() {
        use super::*;
        let test_fn = Solution::hash;
        assert_eq!(test_fn("rn"), 0);
        assert_eq!(test_fn("qp"), 1);
    }

    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;
        assert_eq!(
            test_fn("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "1320"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
        assert_eq!(
            test_fn("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            "145"
        );
    }
}

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    time::Instant,
};

pub struct Solution;

impl Solution {}

#[derive(PartialEq, Debug, Clone)]
enum Group {
    Resolved(usize),
    Unresolved(String),
}

impl Group {
    fn from(g: &str) -> Self {
        match g.contains("?") {
            true => Group::Unresolved(g.into()),
            false => Group::Resolved(g.len()),
        }
    }

    fn is_resolved(&self) -> bool {
        match self {
            Group::Resolved(_) => true,
            Group::Unresolved(_) => false,
        }
    }

    fn resolve(self) -> Vec<Vec<Self>> {
        use Group::*;
        if let Unresolved(g) = self {
            [".", "#"]
                .map(|c| g.replacen('?', c, 1))
                .into_iter()
                .map(move |s| {
                    s.split('.')
                        .filter(|g| !g.is_empty())
                        .map(Group::from)
                        .collect::<Vec<_>>()
                })
                // .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        } else {
            [[self].to_vec()].to_vec()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Springs {
    groups: Vec<Group>,
    pattern: Vec<usize>,
}

impl Springs {
    fn from(input: &str, factor: usize, pattern: &str) -> Self {
        let input = [input]
            .into_iter()
            .cycle()
            .take(factor)
            .collect::<Vec<_>>()
            .join("?");
        let groups = input
            .split('.')
            .filter(|g| !g.is_empty())
            .map(Group::from)
            .collect();

        let pattern = pattern
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let len = pattern.len();
        let mut pattern = pattern
            .into_iter()
            .cycle()
            .take(len * factor)
            .collect::<Vec<_>>();

        Springs { groups, pattern }
    }

    fn count(&self) -> usize {
        let mut total = 0;
        self.clone().resolve(&mut total);
        total
    }

    fn resolve(mut self, total: &mut usize) {
        while let Some(p) = self.pattern.pop() {
            let group = self.groups.pop();
            if group.is_none() {
                return;
            }

            let group = group.unwrap();

            match group {
                Group::Resolved(g) => {
                    if g != p {
                        return;
                    }
                }
                Group::Unresolved(g) => {
                    for g in Group::from(&g).resolve().into_iter() {
                        let mut s = self.clone();
                        s.pattern.push(p);
                        s.groups.extend(g);
                        s.resolve(total)
                    }
                    return;
                }
            }
        }
    
        if self.pattern.is_empty() && self.groups.is_empty() {
            *total += 1;
        }
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let input = input
            .split('\n')
            .map(|x| x.trim())
            .map(|x| x.split(' ').collect::<Vec<_>>())
            .map(|x| Springs::from(x.first().unwrap(), 1, x.last().unwrap()))
            .collect::<Vec<_>>();

        input.iter().map(|s| s.count()).sum::<usize>().to_string()
    }

    fn second_star(input: &str) -> String {
        let input = input
            .split('\n')
            .map(|x| x.trim())
            .map(|x| x.split(' ').collect::<Vec<_>>())
            .map(|x| Springs::from(x.first().unwrap(), 5, x.last().unwrap()))
            .collect::<Vec<_>>();

        input.iter().map(|s| s.count()).sum::<usize>().to_string()
    }
}

mod test {
    use crate::days::day12::Group;

    #[test]
    fn groups() {
        assert_eq!(Group::from("#"), Group::Resolved(1));
        assert_eq!(Group::from("##"), Group::Resolved(2));
        assert_eq!(Group::from("?"), Group::Unresolved("?".to_string()));
        assert_eq!(Group::from("#").resolve(), [[Group::Resolved(1)]]);
        assert_eq!(
            Group::from("?").resolve(),
            [Vec::from([]), Vec::from([Group::Resolved(1)])]
        );
        assert_eq!(
            Group::from("?#").resolve(),
            [[Group::Resolved(1)], [Group::Resolved(2)]]
        );

        assert_eq!(
            Group::from("??").resolve(),
            [
                [Group::Unresolved("?".to_string())],
                [Group::Unresolved("#?".to_string())]
            ]
        );

        assert_eq!(
            Group::from("?#?").resolve(),
            [
                [Group::Unresolved("#?".to_string())],
                [Group::Unresolved("##?".to_string())]
            ]
        );

        assert_eq!(
            Group::from("#?#").resolve(),
            [
                Vec::from([Group::Resolved(1), Group::Resolved(1)]),
                Vec::from([Group::Resolved(3)])
            ]
        );
    }

    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;
        assert_eq!(test_fn("# 1"), "1");
        assert_eq!(test_fn("? 1"), "1");
        assert_eq!(test_fn("#? 2"), "1");
        assert_eq!(test_fn("#.# 1,1"), "1");
        assert_eq!(test_fn("#??? 1"), "1");

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
        assert_eq!(test_fn("???.### 1,1,3"), "1");
        assert_eq!(test_fn(".??..??...?##. 1,1,3"), "16384");
        assert_eq!(test_fn("?#?#?#?#?#?#?#? 1,3,1,6"), "1");
        assert_eq!(test_fn("????.#...#... 4,1,1"), "16");
        assert_eq!(test_fn("????.######..#####. 1,6,5"), "2500");
        assert_eq!(test_fn("?###???????? 3,2,1"), "506250");
        assert_eq!(
            test_fn(
                "???.### 1,1,3
                .??..??...?##. 1,1,3
                ?#?#?#?#?#?#?#? 1,3,1,6
                ????.#...#... 4,1,1
                ????.######..#####. 1,6,5
                ?###???????? 3,2,1"
            ),
            "525152"
        );
    }
}

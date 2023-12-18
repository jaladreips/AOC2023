use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Solution;

impl Solution {}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
enum Group {
    Resolved(usize),
    Unresolved(String),
}

impl Group {
    fn from(g: &str) -> Self {
        match g.contains('?') {
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

    fn is_potentially_empty(&self) -> bool {
        match self {
            Group::Resolved(_) => false,
            Group::Unresolved(g) => !g.contains('#'),
        }
    }

    fn resolve(self) -> Vec<Vec<Self>> {
        use Group::*;
        if let Unresolved(g) = self {
            ["#", "."]
                .map(|c| {
                    let g = g.chars().rev().collect::<String>();
                    let g = g.replacen('?', c, 1);
                    g.chars().rev().collect::<String>()
                })
                .into_iter()
                .map(move |s| {
                    s.split('.')
                        .filter(|g| !g.is_empty())
                        .map(Group::from)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        } else {
            [[self].to_vec()].to_vec()
        }
    }

    fn full_resolve(self) -> Vec<Vec<Self>> {
        let mut gathered_groups = vec![vec![self]];
        loop {
            let mut new_gathered_groups = vec![vec![]];
            let mut all_resolved = true;
            while let Some(mut groups) = gathered_groups.pop() {
                if groups.iter().all(Group::is_resolved) {
                    if !groups.is_empty() {
                        new_gathered_groups.push(groups)
                    }
                } else {
                    all_resolved = false;
                    let mut new_groups_vec = Vec::new();
                    groups.reverse();
                    while let Some(group) = groups.pop() {
                        if group.is_resolved() {
                            new_groups_vec.push(group);
                        } else {
                            groups.reverse();
                            new_gathered_groups.extend(
                                group
                                    .resolve()
                                    .into_iter()
                                    .map(|g| {
                                        let mut new_groups_vec = new_groups_vec.clone();
                                        new_groups_vec.extend(g);
                                        new_groups_vec.extend(groups.clone());
                                        new_groups_vec
                                    })
                                    .collect::<Vec<_>>(),
                            );
                            break;
                        }
                    }
                }
            }

            gathered_groups = new_gathered_groups;

            // let mut new_gathered_groups = vec![vec![]];
            // for groups in gathered_groups {
            //     if !new_gathered_groups.contains(&groups) {
            //         new_gathered_groups.push(groups);
            //     }
            // }

            // gathered_groups = new_gathered_groups;

            if all_resolved {
                break;
            }
        }

        gathered_groups
    }
}

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
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
        let pattern = pattern
            .into_iter()
            .cycle()
            .take(len * factor)
            .collect::<Vec<_>>();

        Springs { groups, pattern }
    }

    fn count(&self) -> usize {
        let mut total = 0;
        let mut cache: HashMap<Springs, usize> = HashMap::new();
        self.clone().resolve(&mut total, &mut cache);
        // let cache = Mutex::new(cache);
        // let cache = Arc::new(cache);
        // self.clone().resolve_par(&mut total);
        // println!("{cache:?}");
        total
    }

    fn resolve(mut self, total: &mut usize, cache: &mut HashMap<Self, usize>) {
        let &m = self.pattern.iter().max().unwrap_or(&0);
        if self.groups.iter().all(|g| match g {
            Group::Resolved(s) => *s < m,
            Group::Unresolved(g) => g.len() < m,
        }) {
            return;
        }

        if let Some(v) = cache.get(&self) {
            *total += v;
            return;
        } else if self
            .groups
            .iter()
            .fold(0, |acc, g| acc + if g.is_resolved() { 1 } else { 0 })
            > self.pattern.len()
        {
            return;
        }

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
                    if let Some(x) = g.split('?').next_back() {
                        if x.len() > p {
                            return;
                        }
                    }

                    for g in Group::from(&g).resolve().into_iter() {
                        let mut s = self.clone();
                        s.pattern.push(p);
                        s.groups.extend(g);
                        let mut val = 0;
                        s.clone().resolve(&mut val, cache);
                        cache.insert(s, val);
                        *total += val;
                    }

                    return;
                }
            }

            if self.pattern.is_empty() && self.groups.iter().all(Group::is_potentially_empty) {
                *total += 1;
            }
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
    #[allow(unused_imports)]
    use crate::days::day12::Group;

    #[test]
    fn groups() {
        assert_eq!(Group::from("#"), Group::Resolved(1));
        assert_eq!(Group::from("##"), Group::Resolved(2));
        assert_eq!(Group::from("?"), Group::Unresolved("?".to_string()));
        assert_eq!(Group::from("#").resolve(), [[Group::Resolved(1)]]);
        assert_eq!(
            Group::from("?").resolve(),
            [Vec::from([Group::Resolved(1)]), Vec::from([])]
        );
        assert_eq!(
            Group::from("?#").resolve(),
            [[Group::Resolved(2)], [Group::Resolved(1)]]
        );

        assert_eq!(
            Group::from("??").resolve(),
            [
                [Group::Unresolved("?#".to_string())],
                [Group::Unresolved("?".to_string())]
            ]
        );

        assert_eq!(
            Group::from("?#?").resolve(),
            [
                [Group::Unresolved("?##".to_string())],
                [Group::Unresolved("?#".to_string())]
            ]
        );

        assert_eq!(
            Group::from("#?#").resolve(),
            [
                Vec::from([Group::Resolved(3)]),
                Vec::from([Group::Resolved(1), Group::Resolved(1)]),
            ]
        );

        assert_eq!(
            Group::from("??#").resolve(),
            [
                Vec::from([Group::Unresolved("?##".to_string())]),
                Vec::from([Group::Unresolved("?".to_string()), Group::Resolved(1)]),
            ]
        );
    }

    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;

        assert_eq!(test_fn("#??? 1"), "1");
        assert_eq!(test_fn("??# 1"), "1");
        assert_eq!(test_fn("??? 1"), "3");
        assert_eq!(test_fn("?.######..#####. 1,6,5"), "1");
        assert_eq!(test_fn("??.######..#####. 1,6,5"), "2");
        assert_eq!(test_fn("????.######..#####. 1,6,5"), "4");
        assert_eq!(test_fn("# 1"), "1");
        assert_eq!(test_fn("? 1"), "1");
        assert_eq!(test_fn("#? 2"), "1");
        assert_eq!(test_fn("#.# 1,1"), "1");

        assert_eq!(test_fn("???.### 1,1,3"), "1");
        assert_eq!(test_fn(".??..??...?##. 1,1,3"), "4");
        assert_eq!(test_fn("?#?#?#?#?#?#?#? 1,3,1,6"), "1");
        assert_eq!(test_fn("????.#...#... 4,1,1"), "1");
        assert_eq!(test_fn("?###???????? 3,2,1"), "10");
        assert_eq!(test_fn(".??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"), "16384");

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
        assert_eq!(test_fn("?#?#?#?#?#?#?#? 1,3,1,6"), "1");
        println!("Done");
        assert_eq!(test_fn("???.### 1,1,3"), "1");
        assert_eq!(test_fn(".??..??...?##. 1,1,3"), "16384");
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

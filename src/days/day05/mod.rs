use regex::Regex;

use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct Solution;

#[derive(PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
struct Mapping {
    src: u64,
    dst: u64,
    range: u64,
}

impl Mapping {
    fn from_capture(capture: regex::Captures) -> Result<Mapping, std::num::ParseIntError> {
        Ok(Mapping {
            src: capture[2].parse::<u64>()?,
            dst: capture[1].parse::<u64>()?,
            range: capture[3].parse::<u64>()?,
        })
    }

    fn map(&self, src: u64) -> Option<u64> {
        if src >= self.src && src < self.src + self.range {
            Some(src - self.src + self.dst)
        } else {
            None
        }
    }
}

impl Solution {
    fn load_map(name: &str, input_as_iter: &mut std::str::Split<char>) -> Vec<Mapping> {
        assert_eq!(input_as_iter.next().unwrap(), format!("{name} map:"));

        let mut result = Vec::new();

        loop {
            let pattern = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
            let Some(captures) = Regex::captures(&pattern, input_as_iter.next().unwrap_or(""))
            else {
                break;
            };

            result.push(Mapping::from_capture(captures).expect("Should be valid"));
        }

        result.sort();

        result
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut seeds = Vec::new();

        let mut lines = input.split('\n');

        let pattern = Regex::new(r"(\d+)").unwrap();

        for seed in pattern.find_iter(lines.next().unwrap()) {
            seeds.push(seed.as_str().parse::<u64>().expect("Should be a number"));
        }

        assert_eq!(lines.next().unwrap(), "");

        let ops = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ]
        .map(|x| Solution::load_map(x, &mut lines));

        let mut best_location = u64::MAX;

        for seed in seeds {
            let mut tmp = seed;

            for op in &ops {
                tmp = *op
                    .iter()
                    .filter_map(|x| x.map(tmp))
                    .collect::<Vec<_>>()
                    .first()
                    .unwrap_or(&tmp);
            }

            best_location = if tmp < best_location {
                tmp
            } else {
                best_location
            };
        }

        best_location.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut lines = input.split('\n');

        let pattern = Regex::new(r"(\d+) (\d+)").unwrap();

        let seed_ranges = pattern.captures_iter(lines.next().unwrap()).map(|x| {
            (
                x[1].parse::<u64>().expect("Should be a number"),
                x[2].parse::<u64>().expect("Should be a number"),
            )
        });

        assert_eq!(lines.next().unwrap(), "");

        let ops = [
            "seed-to-soil",
            "soil-to-fertilizer",
            "fertilizer-to-water",
            "water-to-light",
            "light-to-temperature",
            "temperature-to-humidity",
            "humidity-to-location",
        ]
        .map(|x| Solution::load_map(x, &mut lines));

        let ops = Arc::new(ops);

        let mut handles = Vec::new();
        const STEP: usize = 1 << 16;

        let arc_proposed_locations = Arc::new(Mutex::new(Vec::new()));
        for seed_range in seed_ranges {
            for seed_subrange in (seed_range.0..seed_range.0 + seed_range.1).step_by(STEP) {
                let tmp_ops = ops.clone();
                let tmp_proposed_locations = arc_proposed_locations.clone();
                handles.push(thread::spawn(move || {
                    let mut best_location = u64::MAX;

                    for seed in seed_subrange
                        ..u64::min(seed_subrange + STEP as u64, seed_range.0 + seed_range.1)
                    {
                        let mut tmp = seed;

                        for op in &(*tmp_ops) {
                            tmp = *op
                                .iter()
                                .filter_map(|x| x.map(tmp))
                                .collect::<Vec<_>>()
                                .first()
                                .unwrap_or(&tmp);
                        }

                        best_location = if tmp < best_location {
                            tmp
                        } else {
                            best_location
                        };
                    }
                    tmp_proposed_locations.lock().unwrap().push(best_location);
                }));
            }
        }

        for handle in handles {
            let _ = handle.join();
        }

        let mut proposed_locations = Arc::into_inner(arc_proposed_locations)
            .expect("WTF")
            .into_inner()
            .expect("WTF");

        proposed_locations.sort();

        (*proposed_locations.first().expect("WTF")).to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "35");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "46");
    }
}

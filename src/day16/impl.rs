use std::collections::{HashMap, HashSet};

pub struct Solution;

#[derive(Debug, Clone)]
enum Field {
    MirrorLeftUp,
    MirrorLeftDown,
    Empty,
    SplitHorizontal,
    SplitVertical,
}

impl Field {
    fn from(c: &char) -> Self {
        use Field::*;
        match *c {
            '.' => Empty,
            '/' => MirrorLeftDown,
            '\\' => MirrorLeftUp,
            '|' => SplitHorizontal,
            '-' => SplitVertical,
            _ => {
                panic!()
            }
        }
    }

    fn interaction_helper(&self, dir: &Dir) -> (Dir, Point) {
        use Dir::*;
        match self {
            Field::MirrorLeftDown => match dir {
                Up => (Right, Point { x: 1, y: 0 }),
                Down => (Left, Point { x: -1, y: 0 }),
                Left => (Down, Point { x: 0, y: 1 }),
                Right => (Up, Point { x: 0, y: -1 }),
            },
            Field::MirrorLeftUp => match dir {
                Up => (Left, Point { x: -1, y: 0 }),
                Down => (Right, Point { x: 1, y: 0 }),
                Left => (Up, Point { x: 0, y: -1 }),
                Right => (Down, Point { x: 0, y: 1 }),
            },
            Field::Empty => match dir {
                Left => (Left, Point { x: -1, y: 0 }),
                Right => (Right, Point { x: 1, y: 0 }),
                Up => (Up, Point { x: 0, y: -1 }),
                Down => (Down, Point { x: 0, y: 1 }),
            },
            Field::SplitHorizontal => todo!(),
            Field::SplitVertical => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Down,
    Up,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    dir: Dir,
    pos: Point,
}

impl Beam {
    fn new() -> Self {
        Beam {
            dir: Dir::Right,
            pos: Point { x: 0, y: 0 },
        }
    }

    fn interact(self, f: &Field) -> Vec<Beam> {
        use Field::*;
        match f {
            MirrorLeftUp => [Beam {
                dir: f.interaction_helper(&self.dir).0,
                pos: self.pos + f.interaction_helper(&self.dir).1,
            }]
            .to_vec(),
            MirrorLeftDown => [Beam {
                dir: f.interaction_helper(&self.dir).0,
                pos: self.pos + f.interaction_helper(&self.dir).1,
            }]
            .to_vec(),
            Empty => [Beam {
                dir: f.interaction_helper(&self.dir).0,
                pos: self.pos + f.interaction_helper(&self.dir).1,
            }]
            .to_vec(),
            SplitHorizontal => match self.dir {
                Dir::Left | Dir::Right => [MirrorLeftDown, MirrorLeftUp]
                    .map(|f| Beam {
                        dir: f.interaction_helper(&self.dir).0,
                        pos: self.pos + f.interaction_helper(&self.dir).1,
                    })
                    .to_vec(),
                _ => {
                    let f = Empty;
                    [Beam {
                        dir: f.interaction_helper(&self.dir).0,
                        pos: self.pos + f.interaction_helper(&self.dir).1,
                    }]
                    .to_vec()
                }
            },
            SplitVertical => match self.dir {
                Dir::Down | Dir::Up => [MirrorLeftDown, MirrorLeftUp]
                    .map(|f| Beam {
                        dir: f.interaction_helper(&self.dir).0,
                        pos: self.pos + f.interaction_helper(&self.dir).1,
                    })
                    .to_vec(),
                _ => {
                    let f = Empty;
                    [Beam {
                        dir: f.interaction_helper(&self.dir).0,
                        pos: self.pos + f.interaction_helper(&self.dir).1,
                    }]
                    .to_vec()
                }
            },
        }
    }
}

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let map = input
            .split('\n')
            .map(|x| x.trim())
            .enumerate()
            .flat_map(|(i, x)| {
                x.chars().enumerate().map(move |(j, y)| {
                    (
                        Point {
                            x: j as i64,
                            y: i as i64,
                        },
                        Field::from(&y),
                    )
                })
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();

        let mut beams_history = HashSet::new();
        let mut current_beams = Vec::new();
        let mut energized_fields = HashSet::new();

        current_beams.push(Beam::new());

        while let Some(beam) = current_beams.pop() {
            if beam.pos.x < 0 || beam.pos.x > max_x {
                continue;
            }

            if beam.pos.y < 0 || beam.pos.y > max_y {
                continue;
            }

            if beams_history.contains(&beam) {
                continue;
            }

            beams_history.insert(beam);

            energized_fields.insert(beam.pos);

            let f = map.get(&beam.pos).unwrap();

            current_beams.extend(beam.interact(f));
        }

        energized_fields.len().to_string()
    }

    fn second_star(input: &str) -> String {
        let map = input
            .split('\n')
            .map(|x| x.trim())
            .enumerate()
            .flat_map(|(i, x)| {
                x.chars().enumerate().map(move |(j, y)| {
                    (
                        Point {
                            x: j as i64,
                            y: i as i64,
                        },
                        Field::from(&y),
                    )
                })
            })
            .collect::<HashMap<_, _>>();

        let max_x = map.keys().map(|p| p.x).max().unwrap();
        let max_y = map.keys().map(|p| p.y).max().unwrap();

        
        let run_for_beam = |beam: Beam| {
            let mut beams_history = HashSet::new();
            let mut current_beams = Vec::new();
            let mut energized_fields = HashSet::new();
            
            current_beams.push(beam);
            
            while let Some(beam) = current_beams.pop() {
                if beam.pos.x < 0 || beam.pos.x > max_x {
                    continue;
                }
                
                if beam.pos.y < 0 || beam.pos.y > max_y {
                    continue;
                }
                
                if beams_history.contains(&beam) {
                    continue;
                }
                
                beams_history.insert(beam);
                
                energized_fields.insert(beam.pos);
                
                let f = map.get(&beam.pos).unwrap();
                
                current_beams.extend(beam.interact(f));
            }
            
            energized_fields.len()
        };
        
        let mut max_total = 0;
        for i in 0..=std::cmp::max(max_x, max_y) {
            let v = [
                Beam{ dir: Dir::Right, pos: Point { x: 0, y : i } },
                Beam{ dir: Dir::Left, pos: Point { x: max_x, y : i } },
                Beam{ dir: Dir::Down, pos: Point { x: i, y : 0 } },
                Beam{ dir: Dir::Up, pos: Point { x: i, y : max_y } },
            ].map(run_for_beam).into_iter().max().unwrap();

            max_total = std::cmp::max(max_total, v);
        }

        max_total.to_string()

    }
}

mod test {
    #[test]
    fn first_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::first_star;
        assert_eq!(
            test_fn(
                r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|...."
            ),
            "46"
        );
    }

    #[test]
    fn second_star() {
        use super::*;
        let test_fn = <Solution as crate::Day>::second_star;
        assert_eq!(
            test_fn(
                r".|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|...."
            ),
            "51"
        );
    }
}

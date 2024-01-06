#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn from(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn to_dir(self) -> Dir {
        match (self.x, self.y) {
            (0, 1) => Dir::Down,
            (0, -1) => Dir::Up,
            (-1, 0) => Dir::Left,
            (1, 0) => Dir::Right,
            _ => panic!(),
        }
    }
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

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Point {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dir {
    Down,
    Up,
    Left,
    Right,
}

impl Dir {
    pub fn to_point(self) -> Point {
        match self {
            Dir::Down => Point::from(0, 1),
            Dir::Up => Point::from(0, -1),
            Dir::Left => Point::from(-1, 0),
            Dir::Right => Point::from(1, 0),
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn all_dirs() -> [Self; 4] {
        [Dir::Left, Dir::Right, Dir::Up, Dir::Down]
    }
}

pub fn input_to_2d_map<T, F: Fn(&char) -> T + Copy>(
    input: &str,
    f: F,
) -> std::collections::HashMap<Point, T> {
    input
        .split('\n')
        .map(|x| x.trim())
        .enumerate()
        .flat_map(|(i, x)| {
            x.chars()
                .enumerate()
                .map(move |(j, y)| (Point::from(j as i64, i as i64), f(&y)))
        })
        .collect()
}

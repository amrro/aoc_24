#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,  // Representing '<'
    Right, // Representing '>'
    Up,    // Representing '^'
    Down,  // Representing 'v'
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    loc: Location,
    dir: Direction,
}

impl Guard {
    fn new(loc: Location, dir: Direction) -> Self {
        Self { loc, dir }
    }
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn cordination_add(cor: usize, delta: i8) -> Option<usize> {
        if delta >= 0 {
            cor.checked_add(delta as usize)
        } else {
            cor.checked_sub((-delta) as usize)
        }
    }

    fn delta(&self, delta_x: i8, delta_y: i8) -> Option<Self> {
        if let Some(y) = Self::cordination_add(self.y, delta_y) {
            if let Some(x) = Self::cordination_add(self.x, delta_x) {
                return Some(Location::new(x, y));
            }
        }
        None
    }
}

/// Convert a char into a Direction
impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            _ => Err(format!("Invalid character for direction: {}", c)),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Left => '>',
            Direction::Right => '<',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn signum(&self) -> (i8, i8) {
        let mut row_step = 0;
        let mut col_step = 0;

        match self {
            Direction::Left => row_step = -1,
            Direction::Right => row_step = 1,
            Direction::Up => col_step = -1,
            Direction::Down => col_step = 1,
        }

        (col_step, row_step)
    }
}

// impl Into<char> for Direction {
//     fn into(self) -> char {
//         match self {
//             Direction::Left => '<',
//             Direction::Right => '>',
//             Direction::Up => '^',
//             Direction::Down => 'v',
//         }
//     }
// }

pub struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    guard: Option<Guard>,
}

impl Map {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        let height = data.len();
        let width = data[0].len();

        let mut map = Self {
            data,
            height,
            width,
            guard: None,
        };

        map.guard = map.find_guard();
        map
    }

    fn find_guard(&self) -> Option<Guard> {
        for col in 0..self.height {
            for row in 0..self.width {
                if let Ok(dir) = Direction::try_from(self.data[col][row]) {
                    return Some(Guard::new(Location::new(row, col), dir));
                }
            }
        }

        None
    }

    fn update_guard(&mut self, loc: Location, dir: Direction) {
        self.guard = Some(Guard::new(loc, dir));
        self.data[loc.y][loc.x] = dir.into();
    }

    pub fn walk(&mut self) {
        while let Some(guard) = self.guard {
            let (col_step, row_step) = guard.dir.signum();

            // The new location will never be less than zero, otherwise it will be null.
            if let Some(Location { x, y }) = guard.loc.delta(row_step, col_step) {
                if x < self.width && y < self.height {
                    if self.data[y][x] == '#' {
                        self.update_guard(guard.loc, guard.dir.rotate());
                    } else {
                        self.data[guard.loc.y][guard.loc.x] = 'X';
                        self.update_guard(Location::new(x, y), guard.dir);
                    }
                } else {
                    self.data[guard.loc.y][guard.loc.x] = 'X';
                    self.guard = None;
                }
            } else {
                self.data[guard.loc.y][guard.loc.x] = 'X';
                self.guard = None;
            }
        }
    }

    pub fn count_steps(&self) -> usize {
        let mut steps = 0;
        for col in 0..self.height {
            for row in 0..self.width {
                if self.data[col][row] == 'X' {
                    steps += 1;
                }
            }
        }

        steps
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in 0..self.height {
            for row in 0..self.width {
                write!(f, "{}", self.data[col][row])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_map_detect_guard() {
        let mut map = Map::new(SAMPLE.lines().map(|l| l.chars().collect()).collect());

        map.walk();
        println!("{map}");

        assert_eq!(map.count_steps(), 41);
    }
}

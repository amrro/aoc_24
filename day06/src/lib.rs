#![allow(dead_code)]

use std::{collections::HashSet, fmt};

/// Represents a 2D coordinate on the map.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    x: usize,
    y: usize,
}

/// Represents a direction the guard can take.
///
/// Directions include:
/// - Left (`<`)
/// - Right (`>`)
/// - Up (`^`)
/// - Down (`v`)
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// Represents a guard patrolling a map.
///
/// The [`Map`] struct uses this struct to track the guard's [`Location`] and [`Direction`]
/// during patrolling.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Left => '>',
            Direction::Right => '<',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

impl Direction {
    /// Rotates the direction 90 degrees clockwise.
    pub(crate) fn rotate(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    /// Returns the `(col_step, row_step)` delta for movement in the given direction.
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

/// Represents the lab map containing obstacles, the guard, and dimensions.
///
/// The map tracks:
/// - `data`: A 2D grid of characters representing the lab's layout.
/// - `height` and `width`: Dimensions of the grid.
/// - `guard`: The current position and direction of the [`Guard`].
#[derive(Clone)]
pub struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
    guard: Option<Guard>,
}

impl Map {
    /// Constructs a new map from the given 2D character grid.
    ///
    /// # Paraameters
    /// * `data` - A 2D grid of characters, where:
    ///   - `.` represents an open space.
    ///   - `#` represents an obstacle.
    ///   - `^`, `<`, `>`, or `v` represent the guard's position and direction. See [`Direction`]
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

    /// Calculates guard's initial position and direction on a map.
    ///
    /// Scans the map for a character representing the guard's direction (`^`, `<`, `>`, or `v`).
    /// See [`Direction`].
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

    /// Updates the guard's position and direction on the map.
    /// Marks the guard's new location with her directional character (`^`, `<`, `>`, or `v`).
    /// See [`Direction`].
    fn update_guard(&mut self, loc: Location, dir: Direction) {
        self.guard = Some(Guard::new(loc, dir));
        self.data[loc.y][loc.x] = dir.into();
    }

    /// Simulates the guard's movement across the map until it leaves the map or completes her patrol.
    ///
    /// The guard follows the patrol protocol:
    /// 1. If there is an obstacle directly ahead, the guard turns 90 degrees right. See
    ///    [`Direction::rotate`]
    /// 2. Otherwise, the guard continues forward in her current direction.
    ///
    /// Marks all positions visited by the guard with `X`.
    pub fn walk(&mut self) {
        while let Some(guard) = self.guard {
            let (col_step, row_step) = guard.dir.signum();

            // The new location will never be less than zero, otherwise it will be null.
            if let Some(next_loc) = guard.loc.delta(row_step, col_step) {
                if next_loc.x < self.width && next_loc.y < self.height {
                    if self.data[next_loc.y][next_loc.x] == '#' {
                        self.update_guard(guard.loc, guard.dir.rotate());
                    } else {
                        self.data[guard.loc.y][guard.loc.x] = 'X';
                        self.update_guard(next_loc, guard.dir);
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

    /// Counts the total number of positions visited by the guard (`X`).
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

    /// Tracks the guard's path and checks if it forms a cycle.
    ///
    /// Keeps a record of all locations visited by the guard. If a state repeats,
    /// it indicates a cycle.
    ///
    /// [`Guard`] is used to track locations. So that, if and only if the guard visits the same location with
    /// same direction, it's considered a cycle.
    fn track_guard(&mut self) -> Option<HashSet<Guard>> {
        let mut visited_locations = HashSet::new();
        while let Some(guard) = self.guard {
            if !visited_locations.insert(guard) {
                return Some(visited_locations);
            }

            let (delta_y, delta_x) = guard.dir.signum();
            if let Some(next_loc) = guard.loc.delta(delta_x, delta_y) {
                if next_loc.x < self.width && next_loc.y < self.height {
                    if self.data[next_loc.y][next_loc.x] == '#' {
                        self.guard = Some(Guard::new(guard.loc, guard.dir.rotate()));
                    } else {
                        // Keep the guard moving.
                        self.guard = Some(Guard::new(next_loc, guard.dir));
                        self.data[guard.loc.y][guard.loc.x] = '*';
                    }
                } else {
                    // That's it, if the guard left the map, it means we couldn't trap her.
                    return None;
                }
            } else {
                // That's it, if the gaurd left the map, it means we couldn't trap her.
                return None;
            }
        }

        Some(visited_locations)
    }

    /// Finds all possible trap positions where adding an obstacle would create a cycle.
    ///
    /// Simulates adding an obstacle (`#`) at every open position (`.`) on the map and checks if it traps the guard.
    pub fn find_traps(&self) -> usize {
        let mut traps = 0;
        for col in 0..self.height {
            for row in 0..self.width {
                if self.data[col][row] == '.' {
                    // keep the original map, and simulate on this map with a new obstacle.
                    let mut simulated_map = self.clone();
                    simulated_map.data[col][row] = '#';

                    if simulated_map.track_guard().is_some() {
                        traps += 1;
                    }
                }
            }
        }
        traps
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

    #[test]
    fn test_part_two() {
        let map = Map::new(SAMPLE.lines().map(|l| l.chars().collect()).collect());
        let output = map.find_traps();

        assert_eq!(output, 6);
    }
}

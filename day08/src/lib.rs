#![allow(dead_code)]

use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    ops,
};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq)]
struct Antenna {
    freq: char,
    loc: Location,
}

pub struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

#[derive(Debug)]
pub struct City {
    grid: Grid,
    antennas: HashMap<char, HashSet<Location>>,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn cordination_add(cor: usize, delta: isize) -> Option<usize> {
        if delta >= 0 {
            cor.checked_add(delta as usize)
        } else {
            cor.checked_sub((-delta) as usize)
        }
    }

    fn delta(&self, delta_x: isize, delta_y: isize) -> Option<Self> {
        if let Some(y) = Self::cordination_add(self.y, delta_y) {
            if let Some(x) = Self::cordination_add(self.x, delta_x) {
                return Some(Location::new(x, y));
            }
        }
        None
    }
}

impl ops::Sub for Location {
    type Output = (isize, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.x as isize - rhs.x as isize,
            self.y as isize - rhs.y as isize,
        )
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Antenna {
    fn new(freq: char, x: usize, y: usize) -> Self {
        Self {
            freq,
            loc: Location::new(x, y),
        }
    }
}

impl fmt::Debug for Antenna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "freq: {} at {:?}", self.freq, self.loc)
    }
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        let (height, width) = (data.len(), data[0].len());
        Self {
            grid: data,
            height,
            width,
        }
    }

    /// Modifies the grid at the given location.
    ///
    /// Returns:
    /// - `true`: if the location in bound of the grid.
    /// - `false`: if the location out of the grid.
    fn modify(&mut self, location: &Location, new_value: char) -> bool {
        if location.x >= self.height || location.y >= self.width {
            return false;
        }

        self.grid[location.x][location.y] = new_value;
        true
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in 0..self.height {
            writeln!(f, "{}", self.grid[row].iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl City {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let (height, width) = (grid.len(), grid[0].len());

        let mut antennas = HashMap::new();
        for row in 0..height {
            for col in 0..width {
                if grid[row][col] != '.' {
                    antennas
                        .entry(grid[row][col])
                        .or_insert_with(HashSet::new)
                        .insert(Location::new(row, col));
                }
            }
        }

        Self {
            grid: Grid::new(grid),
            antennas,
        }
    }

    pub fn from(data: &str) -> Self {
        let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        Self::new(grid)
    }

    fn cordination_add(cor: usize, delta: i8) -> Option<usize> {
        if delta >= 0 {
            cor.checked_add(delta as usize)
        } else {
            cor.checked_sub((-delta) as usize)
        }
    }

    pub fn find_antinodes(&self) -> Grid {
        let mut antinode_grid = Grid::new(vec![vec!['.'; self.grid.height]; self.grid.width]);

        for (_freq, locs) in self.antennas.iter() {
            let locs: Vec<Location> = locs.iter().cloned().collect();
            for i in 0..locs.len() {
                for j in i + 1..locs.len() {
                    if i == j {
                        continue;
                    }
                    let (ant, other) = (locs[i], locs[j]);
                    let (delta_x, delta_y) = ant - other;

                    let mut multiplier = 0;
                    while let Some(new_antinode) =
                        ant.delta(multiplier * delta_x, multiplier * delta_y)
                    {
                        if !antinode_grid.modify(&new_antinode, '#') {
                            break;
                        }

                        multiplier += 1;
                    }

                    let mut multiplier = 0;
                    while let Some(new_antinode) =
                        other.delta(multiplier * -delta_x, multiplier * -delta_y)
                    {
                        if !antinode_grid.modify(&new_antinode, '#') {
                            break;
                        }

                        multiplier += 1;
                    }
                }
            }
        }

        antinode_grid
    }

    pub fn get_unique_antinode_count(&self) -> usize {
        let antinode_grid = dbg!(self.find_antinodes());
        let mut antinode_count = 0;
        for row in 0..antinode_grid.height {
            for col in 0..antinode_grid.width {
                if antinode_grid.grid[row][col] == '#' {
                    antinode_count += 1;
                }
            }
        }

        antinode_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_city_from_str() {
        let city = City::from(SAMPLE);

        let count = city.get_unique_antinode_count();
        assert_eq!(count, 34);
    }
}

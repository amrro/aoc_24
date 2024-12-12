#![allow(dead_code)]

use std::{
    collections::{HashSet, VecDeque},
    convert, fmt, ops,
};

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn delta(&self) -> (i8, i8) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Region {
    plots: Vec<Location>,
    perimeter: usize,
}

pub struct Garden {
    plants: Vec<Vec<char>>,
    height: usize,
    width: usize,
    direction: [Direction; 4],
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Location {
    fn cord_add(cord: usize, delta: i8) -> Option<usize> {
        if delta >= 0 {
            cord.checked_add(delta as usize)
        } else {
            cord.checked_sub((-delta) as usize)
        }
    }

    fn add_delta(&self, detla_x: i8, delta_y: i8) -> Option<Self> {
        if let Some(x) = Self::cord_add(self.x, detla_x) {
            if let Some(y) = Self::cord_add(self.y, delta_y) {
                return Some(Location { x, y });
            }
        }

        None
    }
}

impl Region {
    #[inline]
    fn area(&self) -> usize {
        self.plots.len()
    }

    #[inline]
    fn price(&self) -> usize {
        self.perimeter * self.area()
    }
}

impl Garden {
    pub fn new(plots: Vec<Vec<char>>) -> Self {
        let (height, width) = (plots.len(), plots[0].len());
        Self {
            plants: plots,
            height,
            width,
            direction: [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
        }
    }

    fn in_bound(&self, loc: &Location) -> bool {
        loc.x < self.height && loc.y < self.width
    }

    fn adjacents_to(&self, plant: char, at: &Location) -> Vec<Location> {
        let mut adjacents = Vec::new();
        for dir in &self.direction {
            let (delta_x, delta_y) = dir.delta();
            if let Some(adj) = at.add_delta(delta_x, delta_y) {
                if self.in_bound(&adj) && self[&adj] == plant {
                    adjacents.push(adj);
                }
            }
        }

        adjacents
    }

    fn find_region(&self, start: Location, seen: &mut HashSet<Location>) -> Region {
        let mut queue = VecDeque::from([start]);
        let mut plots = Vec::new();
        let mut perimeter = 0;
        let plant = self[&start];

        while let Some(location) = queue.pop_front() {
            if seen.contains(&location) {
                continue;
            } else {
                // Mark this plant as seen.
                seen.insert(location);
            }

            plots.push(location);

            let adjacents = self.adjacents_to(plant, &location);

            perimeter += 4 - adjacents.len();
            queue.extend(adjacents);
        }

        Region { plots, perimeter }
    }

    fn regions(&self) -> Vec<Region> {
        let mut seen = HashSet::with_capacity(self.width * self.height);
        let mut regions = Vec::new();
        for x in 0..self.height {
            for y in 0..self.width {
                if !seen.contains(&Location { x, y }) {
                    regions.push(self.find_region(Location { x, y }, &mut seen));
                }
            }
        }
        regions
    }

    pub fn total_price(&self) -> usize {
        self.regions().into_iter().map(|r| r.price()).sum()
    }
}

impl convert::From<&str> for Garden {
    fn from(value: &str) -> Self {
        let plants = value
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect();
        Self::new(plants)
    }
}

impl ops::Index<&Location> for Garden {
    type Output = char;
    fn index(&self, loc: &Location) -> &Self::Output {
        if !self.in_bound(loc) {
            panic!(
                "Point {:?} out of bound, Map's dimentions: (height: {}, width: {})",
                loc, self.height, self.width
            );
        }
        &self.plants[loc.x][loc.y]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_garden_region() {
        let garden = Garden::from(SAMPLE);
        // dbg!(garden.regions());
        let total_price = garden.total_price();
        assert_eq!(total_price, 1930);
    }
}

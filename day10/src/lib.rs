#![allow(dead_code)]

use core::fmt;
use std::{
    collections::{HashSet, VecDeque},
    convert, ops,
};

const EMPTY: u8 = u8::MAX;

#[inline]
fn abs_diff(lhs: u8, rhs: u8) -> u8 {
    if lhs > rhs {
        lhs - rhs
    } else {
        rhs - lhs
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn delta(&self) -> (i8, i8) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, -1),
            Direction::West => (0, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Point {
    #[inline]
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
                return Some(Point { x, y });
            }
        }
        None
    }
}

pub struct TopoMap {
    contours: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl convert::From<&str> for TopoMap {
    fn from(value: &str) -> Self {
        let data = value
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or(EMPTY as u32) as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();
        Self::new(data)
    }
}

impl ops::Index<&Point> for TopoMap {
    type Output = u8;

    fn index(&self, point: &Point) -> &Self::Output {
        if !self.in_bound(point) {
            panic!(
                "Point {:?} out of bound, Map's dimentions: (height: {}, width: {})",
                point, self.height, self.width
            );
        }

        &self.contours[point.x][point.y]
    }
}

impl TopoMap {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        let (height, width) = (data.len(), data[0].len());
        Self {
            contours: data,
            width,
            height,
        }
    }

    #[inline]
    fn in_bound(&self, point: &Point) -> bool {
        point.x < self.height && point.y < self.width
    }

    /// Returns valid neighbors to some point.
    ///
    /// Neighbors only differ from the `point` in height by one.
    fn valid_neighbors(&self, point: &Point) -> Option<Vec<Point>> {
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        let mut neighbors = Vec::new();

        for dir in &directions {
            let (delta_x, delta_y) = dir.delta();

            if let Some(n) = point.delta(delta_x, delta_y) {
                if !self.in_bound(&n) || self[&n] == EMPTY {
                    continue;
                }

                let diff = self[&n].saturating_sub(self[point]);
                if diff == 1 {
                    neighbors.push(n);
                }
            }
        }

        if neighbors.is_empty() {
            None
        } else {
            Some(neighbors)
        }
    }

    pub fn total_score(&self) -> usize {
        let mut total_score = 0;
        for x in 0..self.height {
            for y in 0..self.width {
                if self.contours[x][y] == 0 {
                    let trailhead = Point { x, y };
                    total_score += self.unique_paths(trailhead);
                }
            }
        }

        total_score
    }

    pub fn unique_paths(&self, head: Point) -> usize {
        let mut stack = VecDeque::from([head]);
        let mut visited = HashSet::new();
        let mut score = 0;

        while let Some(current) = stack.pop_back() {
            // Skip already visited nodes
            if visited.contains(&current) {
                continue;
            }

            // Check if current point is height 9
            if self[&current] == 9 {
                score += 1;
            }

            // Get valid neighbors and add them to the stack
            if let Some(neighbors) = self.valid_neighbors(&current) {
                stack.extend(neighbors.into_iter());
            }

            // Mark as visited
            visited.insert(current);
        }

        score
    }

    pub fn total_rating(&self) -> usize {
        let mut total_score = 0;
        for x in 0..self.height {
            for y in 0..self.width {
                if self.contours[x][y] == 0 {
                    let trailhead = Point { x, y };
                    total_score += self.count_paths(trailhead);
                }
            }
        }

        total_score
    }

    fn count_paths(&self, current: Point) -> usize {
        let mut stack = VecDeque::from([current]);
        let mut score = 0;

        while let Some(current) = stack.pop_back() {
            // Check if current point is height 9
            if self[&current] == 9 {
                score += 1;
            }

            // Get valid neighbors and add them to the stack
            if let Some(neighbors) = self.valid_neighbors(&current) {
                stack.extend(neighbors.into_iter());
            }
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_map_total_score() {
        let map = TopoMap::from(SAMPLE);
        let score = map.total_score();
        assert_eq!(score, 36);
    }

    #[test]
    fn test_map_total_rating() {
        let map = TopoMap::from(SAMPLE);
        let score = map.total_rating();
        assert_eq!(score, 81);
    }
}

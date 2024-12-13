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

    pub fn delta_all() -> Vec<(i8, i8)> {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .map(|d| d.delta())
        .collect()
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
    sides: usize,
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

    /// Computes the number of contiguous sides (fence sections) in the region.
    fn compute_sides(&mut self) {
        let mut unique_sides = HashSet::new();

        // For each plot in the region
        for &Location { x, y } in &self.plots {
            // Check all four directions (North, East, South, West)
            for (dx, dy) in Direction::delta_all() {
                let neighbor_x = x.wrapping_add(dx as usize);
                let neighbor_y = y.wrapping_add(dy as usize);

                // If the neighbor is not part of the region, track this side
                if !self.plots.contains(&Location {
                    x: neighbor_x,
                    y: neighbor_y,
                }) {
                    let mut edge_x = x;
                    let mut edge_y = y;

                    // Traverse along the direction to find the end of the contiguous edge
                    while self.plots.contains(&Location {
                        x: edge_x.wrapping_add(dy as usize),
                        y: edge_y.wrapping_add(dx as usize),
                    }) && !self.plots.contains(&Location {
                        x: edge_x.wrapping_add(dx as usize),
                        y: edge_y.wrapping_add(dy as usize),
                    }) {
                        edge_x = edge_x.wrapping_add(dy as usize);
                        edge_y = edge_y.wrapping_add(dx as usize);
                    }

                    unique_sides.insert((edge_x, edge_y, dx, dy));
                }
            }
        }

        // The number of unique sides (fence sections)
        self.sides = unique_sides.len();
    }

    #[inline]
    fn price(&self, discount: bool) -> usize {
        if discount {
            self.sides * self.area()
        } else {
            self.perimeter * self.area()
        }
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
        let sides = 0;
        let target_plant = self[&start];

        while let Some(location) = queue.pop_back() {
            if seen.contains(&location) {
                continue;
            } else {
                // Mark this plant as seen.
                seen.insert(location);
            }

            plots.push(location);

            let adjacents = self.adjacents_to(target_plant, &location);

            perimeter += 4 - adjacents.len();
            queue.extend(adjacents);
        }

        let mut region = Region {
            plots,
            perimeter,
            sides,
        };
        region.compute_sides();

        region
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

    #[inline]
    pub fn total_price(&self, with_discount: bool) -> usize {
        self.regions()
            .into_iter()
            .map(|r| r.price(with_discount))
            .sum()
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
        let total_price = garden.total_price(false);
        assert_eq!(total_price, 1930);
    }

    #[test]
    fn test_garden_regions_with_discount() {
        let garden = Garden::from(SAMPLE);
        // dbg!(garden.regions());
        let total_price = garden.total_price(true);
        assert_eq!(total_price, 1206);
    }
}

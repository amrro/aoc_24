#![allow(dead_code)]

mod utils;

use std::fmt;

type Id = usize;
const SPACE: Option<Id> = None;

pub struct Disk {
    map: Vec<Option<Id>>,
}

impl Disk {
    pub fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut file_id: Id = 0;

        for (idx, c) in input
            .chars()
            .enumerate()
            .filter(|(_idx, c)| c.is_ascii_digit())
        {
            let length = c.to_digit(10).unwrap() as usize;
            let is_file = idx % 2 == 0;

            for _ in 0..length {
                if is_file {
                    blocks.push(Some(file_id));
                } else {
                    blocks.push(SPACE);
                }
            }

            if is_file {
                file_id += 1;
            }
        }

        Self { map: blocks }
    }

    pub fn fragment(&mut self) {
        let non_space_len = self.map.iter().filter(|&&b| b.is_some()).count();
        for idx in 0..non_space_len {
            // Skipe blocks.
            if self.map[idx].is_some() {
                continue;
            }
            let swap_idx = self.map.iter().rposition(|&b| b.is_some()).unwrap();
            self.map.swap(swap_idx, idx);
        }
    }

    pub fn checksum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(position, &block)| block.map(|v| v * position))
            .sum()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|b| if b.is_some() {
                    b.unwrap().to_string()
                } else {
                    '.'.to_string()
                })
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_map_parse() {
        let input = "2333133121414131402";
        let disk_map = Disk::parse(input);
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        assert_eq!(disk_map.to_string(), expected);
    }

    #[test]
    fn test_disk_map_fragment() {
        let input = "2333133121414131402";
        let mut disk_map = Disk::parse(input);
        disk_map.fragment();

        let expected = "0099811188827773336446555566..............";
        assert_eq!(disk_map.to_string(), expected);
    }

    #[test]
    fn test_disk_checksum() {
        let input = "2333133121414131402";
        let mut disk_map = Disk::parse(input);
        disk_map.fragment();
        let output = disk_map.checksum();

        let expected = 1928;
        assert_eq!(output, expected);
    }
}

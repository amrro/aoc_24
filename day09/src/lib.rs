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

    pub fn defragment(&mut self) {
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

struct File {
    id: Id,
    size: usize,
    // The original position in diska map.
    position: usize,
}

pub struct Files {
    list: Vec<File>,
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "position: {}, size: {}", self.position, self.size)
    }
}

impl Files {
    pub fn parse(input: &str) -> Self {
        let mut files = Vec::new();
        let mut file_id = 0;
        let mut position = 0;

        for (idx, ch) in input.trim().char_indices() {
            let size = ch.to_digit(10).unwrap() as usize;

            if idx % 2 == 0 {
                files.push(File {
                    id: file_id,
                    position,
                    size,
                });
                file_id += 1;
            }
            position += size;
        }

        Self { list: files }
    }

    pub fn defragment(&mut self) {
        let max_id = self.list.last().unwrap().id;
        for id in (0..=max_id).rev() {
            let file_idx = self.list.iter().position(|x| x.id == id).unwrap();
            let file = &self.list[file_idx];

            let mut new_pos = None;
            for window in self.list.windows(2) {
                if let [a, b] = window {
                    let free = (b.position) - (a.position + a.size);
                    let pos = a.position + a.size;

                    if pos > file.position {
                        break;
                    }

                    if free >= file.size {
                        new_pos = Some(pos);
                        break;
                    }
                }
            }

            if let Some(new_pos) = new_pos {
                self.list[file_idx].position = new_pos;
            }

            self.list.sort_by_key(|x| x.position);
        }
    }

    pub fn checksum(&self) -> usize {
        let mut sum = 0;
        for file in &self.list {
            for idx in file.position..(file.position + file.size) {
                sum += idx * file.id;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_checksum() {
        let input = "2333133121414131402";
        let mut disk_map = Disk::parse(input);
        disk_map.defragment();
        let output = disk_map.checksum();

        let expected = 1928;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_files_checksum() {
        let input = "2333133121414131402";
        let mut files = Files::parse(input);
        files.defragment();
        let output = files.checksum();

        assert_eq!(output, 2858);
    }
}

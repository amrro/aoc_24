use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn count_word(grid: Vec<Vec<char>>, word: &str) -> usize {
    let mut count = 0;

    let pivot_start = word.chars().next().unwrap();
    let pivot_end = word.chars().nth_back(0).unwrap();
    dbg!(pivot_end);
    let word_length = word.len();
    let height = grid.len();
    let width = grid[0].len();

    for col in 0..height {
        for row in 0..width {
            if grid[col][row] != pivot_start {
                continue;
            }

            // Reading `word` to the right.
            if row <= width - word_length {
                println!("width: {}, range: [{}..{}]", width, row, row + word_length);
                let expected = String::from_iter(&grid[col][row..row + word_length]);
                if expected == word {
                    count += 1;
                }
            }

            // Reading `word` to the left.
            if row >= word_length - 1 {
                let slice: String = (0..word_length).map(|idx| grid[col][row - idx]).collect();
                if slice == word {
                    count += 1;
                }
            }

            // Reading word upward.
            if col >= word_length - 1 {
                let slice = (0..word_length)
                    .map(|idx| grid[col - idx][row])
                    .collect::<String>();

                if slice == word {
                    count += 1;
                }
            }

            // Reading word downward
            if col <= height - word_length {
                let slice: String = (0..word_length).map(|idx| grid[col + idx][row]).collect();
                if slice == word {
                    count += 1;
                }
            }

            // Bishop movement to up-right.
            if row <= width - word_length && col >= word_length - 1 {
                let slice: String = (0..word_length)
                    .map(|idx| grid[col - idx][row + idx])
                    .collect();

                if slice == word {
                    count += 1;
                }
            }

            // Bishop movement down-right.
            if row <= width - word_length && col <= height - word_length {
                let slice: String = (0..word_length)
                    .map(|idx| grid[col + idx][row + idx])
                    .collect();

                if slice == word {
                    count += 1;
                }
            }

            // Bishop movement down-left
            if row >= word_length - 1 && col <= height - word_length {
                let slice: String = (0..word_length)
                    .map(|idx| grid[col + idx][row - idx])
                    .collect();

                if slice == word {
                    count += 1;
                }
            }

            // Bishop movement up-left.
            if row >= word_length - 1 && col >= word_length - 1 {
                let slice: String = (0..word_length)
                    .map(|idx| grid[col - idx][row - idx])
                    .collect();

                if slice == word {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_one() -> usize {
    let grid: Vec<Vec<char>> = read_file("src/input.txt")
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    count_word(grid, "XMAS")
}

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;

    const SAMPLE: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn it_works() {
        let grid: Vec<Vec<char>> = SAMPLE
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let count = count_word(grid, "XMAS");

        assert_eq!(count, 18);
    }

    #[test]
    fn test_part_one() {
        let output = part_one();
        assert_eq!(output, 2575);
    }
}

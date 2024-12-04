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

pub fn find_word(grid: Vec<Vec<char>>, word: &str) -> usize {
    let mut count = 0;

    let pivot_start = word.chars().next().unwrap();
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

    find_word(grid, "XMAS")
}

pub fn find_mas_x(grid: Vec<Vec<char>>) -> usize {
    let mut count = 0;

    let pivot_start = 'A';
    let height = grid.len();
    let width = grid[0].len();

    for col in 1..height - 1 {
        for row in 1..width - 1 {
            if grid[col][row] != pivot_start {
                continue;
            }

            let first_diagonal = String::from_iter([
                grid[col - 1][row - 1],
                grid[col][row],
                grid[col + 1][row + 1],
            ]);

            let second_diagonal = String::from_iter([
                grid[col - 1][row + 1],
                grid[col][row],
                grid[col + 1][row - 1],
            ]);

            if (first_diagonal == "MAS" || first_diagonal == "SAM")
                && (second_diagonal == "MAS" || second_diagonal == "SAM")
            {
                count += 1;
            }
        }
    }

    count
}

fn part_two() -> usize {
    let grid: Vec<Vec<char>> = read_file("src/input.txt")
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    find_mas_x(grid)
}

#[cfg(test)]
mod tests {

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
    fn part_one_sample() {
        let grid: Vec<Vec<char>> = SAMPLE
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let count = find_word(grid, "XMAS");

        assert_eq!(count, 18);
    }

    #[test]
    fn test_part_one() {
        let output = part_one();
        assert_eq!(output, 2575);
    }

    #[test]
    fn test_part_two_sample() {
        let grid: Vec<Vec<char>> = SAMPLE
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let count = find_mas_x(grid);

        assert_eq!(count, 9);
    }

    #[test]
    fn test_part_two() {
        let output = part_two();
        let expected = 2041_usize;
        assert_eq!(output, expected);
    }
}

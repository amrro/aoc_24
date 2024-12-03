use core::str;
use std::{
    fs::{self},
    io::{self, BufRead},
    path::Path,
    result::Result,
};

use regex::{Error, Regex};

const INPUT_PATH: &str = "src/input.txt";

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn extract(re: &regex::Regex, haystack: &str) -> Vec<(usize, usize)> {
    re.captures_iter(haystack)
        .map(|c| {
            let first = c.name("first").unwrap().as_str().parse::<usize>().unwrap();
            let second = c.name("second").unwrap().as_str().parse::<usize>().unwrap();

            (first, second)
        })
        .collect()
}

pub fn part_one() -> usize {
    let re = regex::Regex::new(r"mul\((?P<first>[0-9]{1,3}),(?P<second>[0-9]{1,3})\)").unwrap();
    read_file(INPUT_PATH)
        .lines()
        .map_while(Result::ok)
        .flat_map(|line| extract(&re, &line))
        .map(|(a, b)| a * b)
        .sum::<usize>()
}

/// Represents a parsed instruction from the corrupted memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    /// A multiplication instruction with two operands.
    Mul(usize, usize),
    /// - `Do` : Enables all subsequent `Mul` instructions.
    Do,
    /// - `Dont`: Disables all subsequent `Mul` instructions.
    Dont,
}

impl Instruction {
    const MUL_PATTERN: &str = r"mul\((?P<first>[0-9]{1,3}),(?P<second>[0-9]{1,3})\)";
    const DO_PATTERN: &str = r"do\(\)";
    const DONT_PATTERN: &str = r"don't\(\)";
}

impl Instruction {
    /// Parses a string into an `Instruction`.
    ///
    /// # Arguments
    /// - `input`: A string slice containing the raw instruction to parse.
    ///
    /// # Returns
    /// - `Ok(Instruction)`: If the input matches one of the patterns for `Mul`, `Do`, or `Dont`.
    /// - `Err(regex::Error)`: If the input does not match any known pattern.
    ///
    /// # Examples
    /// ```rust
    /// use day03::Instruction;
    ///
    /// let instr = Instruction::new("mul(2,4)").unwrap();
    /// assert_eq!(instr, Instruction::Mul(2, 4));
    ///
    /// let instr = Instruction::new("do()").unwrap();
    /// assert_eq!(instr, Instruction::Do);
    ///
    /// let instr = Instruction::new("don't()").unwrap();
    /// assert_eq!(instr, Instruction::Dont);
    /// ```
    pub fn parse(input: &str) -> Result<Self, regex::Error> {
        if regex::Regex::new(Self::MUL_PATTERN)?.is_match(input) {
            let re = regex::Regex::new(Self::MUL_PATTERN)?;
            if let Some(capture) = re.captures(input) {
                let first = capture
                    .name("first")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let second = capture
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                return Ok(Self::Mul(first, second));
            }
        } else if Regex::new(Self::DO_PATTERN)?.is_match(input) {
            return Ok(Self::Do);
        } else if Regex::new(Self::DONT_PATTERN)?.is_match(input) {
            return Ok(Self::Dont);
        }
        Err(Error::Syntax(format!(
            "Failed to find any matches: {}",
            input
        )))
    }

    /// Extracts all instructions from a given string.
    ///
    /// # Arguments
    /// - `haystack`: A string slice containing the corrupted memory dump.
    ///
    /// # Returns
    /// - `Vec<Result<Instruction, regex::Error>>`: A vector of parsed instructions or errors.
    ///
    /// # Examples
    /// ```rust
    /// use day03::Instruction;
    ///
    /// let input = "mul(2,4)_mul(3,7)&don't()_mul(5,5)_do()_mul(8,5)";
    /// let instructions = Instruction::extract(input);
    ///
    /// let parsed: Vec<Instruction> = instructions.into_iter().filter_map(Result::ok).collect();
    /// assert_eq!(parsed, vec![
    ///     Instruction::Mul(2, 4),
    ///     Instruction::Mul(3, 7),
    ///     Instruction::Dont,
    ///     Instruction::Mul(5, 5),
    ///     Instruction::Do,
    ///     Instruction::Mul(8, 5)
    /// ]);
    /// ```
    pub fn extract_all(haystack: &str) -> Vec<Result<Self, regex::Error>> {
        let re = Regex::new(&format!(
            "{}|{}|{}",
            Self::MUL_PATTERN,
            Self::DO_PATTERN,
            Self::DONT_PATTERN
        ))
        .unwrap();

        re.captures_iter(haystack)
            .map(|capture| Instruction::parse(capture.get(0).unwrap().as_str()))
            .collect()
    }

    /// Filters and processes instructions to only include enabled multiplications.
    ///
    /// # Arguments
    /// - `instructions`: A vector of `Instruction` values parsed from the memory dump.
    ///
    /// # Returns
    /// - `Vec<(usize, usize)>`: A vector of enabled multiplications represented as pairs of operands.
    ///
    /// # Behavior
    /// - At the start, `Mul` instructions are enabled.
    /// - A `Dont` instruction disables all subsequent `Mul` instructions.
    /// - A `Do` instruction re-enables all subsequent `Mul` instructions.
    ///
    /// # Examples
    /// ```rust
    /// use day03::Instruction;
    /// let instructions = vec![
    ///     Instruction::Mul(2, 4),
    ///     Instruction::Dont,
    ///     Instruction::Mul(5, 5),
    ///     Instruction::Do,
    ///     Instruction::Mul(8, 5)
    /// ];
    ///
    /// let result = Instruction::clean(instructions);
    /// assert_eq!(result, vec![(2, 4), (8, 5)]);
    /// ```
    pub fn filter_enabled(instructions: Vec<Instruction>) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut is_mul_enabled = false;

        for instr in instructions {
            match instr {
                Instruction::Mul(first, second) => {
                    if !is_mul_enabled {
                        result.push((first, second))
                    }
                }
                Instruction::Do => is_mul_enabled = false,
                Instruction::Dont => is_mul_enabled = true,
            }
        }

        result
    }
}

pub fn part_two() -> usize {
    let instructions: Vec<Instruction> = read_file(INPUT_PATH)
        .lines()
        .map_while(Result::ok)
        .flat_map(|line| Instruction::extract_all(&line))
        .map_while(|r| r.clone().ok())
        .collect();

    let mul_instructions = Instruction::filter_enabled(instructions);
    mul_instructions.iter().map(|(a, b)| a * b).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_op() {
        let result = part_one();
        let expected = 161289189;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let output = part_two();
        let expected = 83595109;
        assert_eq!(output, expected);
    }
}

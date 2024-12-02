use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

#[allow(dead_code)]
const SAMPLE: &str = "src/sample.txt";
const INPUT: &str = "src/input.txt";

#[derive(Debug, PartialEq)]
enum Trend {
    Ascending,
    Descending,
}

impl Trend {
    fn determine_trend(report: &[u8]) -> Self {
        if report[0] < report[1] {
            Self::Ascending
        } else {
            Self::Descending
        }
    }

    /// First Safety Check: The levels are either all increasing or all decreasing.
    fn matches(&self, first_level: u8, second_level: u8) -> bool {
        match self {
            Trend::Ascending => first_level < second_level,
            Trend::Descending => first_level > second_level,
        }
    }
}

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn read_reports() -> Vec<std::vec::Vec<u8>> {
    read_file(INPUT)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

/// Second Safety Check: Any two adjacent levels differ by at least 1 and at most 3
#[inline]
pub fn is_levels_safe(first_level: u8, second_level: u8) -> bool {
    (1..=3).contains(&first_level.abs_diff(second_level))
}

pub fn is_report_safe(report: &[u8]) -> bool {
    let trend = Trend::determine_trend(report);
    report.windows(2).all(|pair| {
        if let [first, second] = pair {
            trend.matches(*first, *second) && is_levels_safe(*first, *second)
        } else {
            false
        }
    })
}

pub fn part_one() -> usize {
    read_reports()
        .iter()
        .filter(|&report| is_report_safe(report))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_reports() {
        let safe_reports = part_one();

        assert_eq!(safe_reports, 287);
    }
}

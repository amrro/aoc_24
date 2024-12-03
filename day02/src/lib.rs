use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

#[allow(dead_code)]
const SAMPLE: &str = "src/sample.txt";
const INPUT: &str = "src/input.txt";

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

pub fn read_reports() -> Vec<std::vec::Vec<i32>> {
    read_file(INPUT)
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .inspect(|r| println!("{:?}", r))
        .collect::<Vec<Vec<i32>>>()
}

pub fn check_safety(report: &[i32]) -> bool {
    let mut diffs = vec![];
    for window in report.windows(2) {
        diffs.push(window[1] - window[0]);
    }

    // Safety Check:
    let valid_diffs = diffs.iter().all(|&d| d.abs() > 0 && d.abs() < 4);
    if !valid_diffs {
        return false;
    }

    let is_all_positives = diffs.iter().all(|&d| d > 0);
    let is_all_negatives = diffs.iter().all(|&d| d < 0);

    is_all_positives || is_all_negatives
}

pub fn dampen_check_safety(report: &[i32]) -> bool {
    // if the report itself is safe, no need to check.
    if check_safety(report) {
        return true;
    }

    for idx in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(idx);

        if check_safety(&new_report) {
            return true;
        }
    }

    false
}

pub fn part_one() -> usize {
    read_reports()
        .iter()
        .filter(|&report| check_safety(report))
        .count()
}

pub fn part_two() -> usize {
    read_reports()
        .iter()
        .filter(|&report| dampen_check_safety(report))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let safe_reports = part_one();

        assert_eq!(safe_reports, 287);
    }

    #[test]
    fn test_part_two() {
        let safe_reports = part_two();
        assert_eq!(safe_reports, 354);
    }
}

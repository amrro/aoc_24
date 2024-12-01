use core::panic;
use std::{
    collections::HashMap,
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

fn parse_location(line: &str) -> (usize, usize) {
    let mut locations = line.split_whitespace();
    let location_one = locations.next().unwrap().parse::<usize>().unwrap();
    let location_two = locations.next().unwrap().parse::<usize>().unwrap();

    (location_one, location_two)
}

pub fn read_locations() -> (Vec<usize>, Vec<usize>) {
    read_file(INPUT)
        .lines()
        .map_while(Result::ok)
        .map(|line| parse_location(&line))
        .unzip()
}

fn how_far_apart(first: usize, second: usize) -> usize {
    first.abs_diff(second)
}

pub fn part_one() -> usize {
    let (mut list_one, mut list_two) = read_locations();
    list_one.sort();
    list_two.sort();

    let mut sum = 0usize;
    for idx in 0..list_one.len() {
        sum += how_far_apart(list_one[idx], list_two[idx]);
    }

    sum
}

fn similarity_score(value: usize, locations: &[usize]) -> usize {
    let freq = locations.iter().filter(|l| **l == value).count();
    freq * value
}

pub fn part_two() -> usize {
    let (list_one, list_two) = read_locations();

    list_one.into_iter().fold(0_usize, |score, loc| {
        score + similarity_score(loc, &list_two)
    })
}

fn precompute_scores(locations: &[usize]) -> HashMap<usize, usize> {
    let mut scores = HashMap::new();
    for &loc in locations {
        *scores.entry(loc).or_insert(0) += 1;
    }
    scores
}

fn similarity_score_precomputed(loc: usize, freqs: &HashMap<usize, usize>) -> usize {
    let score = freqs.get(&loc).cloned().unwrap_or(0);
    loc * score
}

pub fn part_two_precomputed() -> usize {
    let (list_one, list_two) = read_locations();
    let scores_freq = precompute_scores(&list_two);

    list_one.into_iter().fold(0_usize, |scores, loc| {
        scores + similarity_score_precomputed(loc, &scores_freq)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample() {
        let result = part_one();
        let expected = 1110981_usize;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two();
        let expected = 24869388;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two_with_freqs() {
        let result = part_two_precomputed();
        let expected = 24869388;
        assert_eq!(result, expected);
    }
}

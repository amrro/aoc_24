use std::io::BufRead;

use day05::read::{get_rules, get_sequences, read_file};
use day05::Graph;

fn main() {
    let input = read_file("input/05.txt")
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");

    let (rule_pairs, seqs) = input.split_once("\n\n").unwrap();
    let rules = get_rules(rule_pairs);

    let graph = Graph::new(&rules);

    let output: usize = get_sequences(seqs)
        .iter()
        .filter(|s| graph.validate(s))
        .map(|s| s[s.len() / 2])
        .sum();

    println!("** Solution: {output} **");
}

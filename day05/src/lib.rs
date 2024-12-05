#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{self, BufRead},
    path::Path,
};

/// Represents a directed graph where each node has dependencies defined by rules.
///
/// The graph is implemented as an adjacency list, where the `rules` field maps
/// each node to the nodes of pages it depends on. This structure is used to model
/// page ordering rules for an elf's printing system.
struct Graph {
    /// Adjacency list where keys are nodes and values are sets of dependencies.
    /// For a rule `X|Y`, `rules[Y]` will include `X`, meaning `Y` depends on `X`.
    rules: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    /// Constructs a new `Graph` from a list of page ordering rules.
    ///
    /// Each rule `(a, b)` indicates that page `b` depends on page `a`,
    /// i.e., `a` must be printed before `b` if both are part of an update.
    fn new(rules: &[(usize, usize)]) -> Self {
        let mut graph = HashMap::new();

        for &(a, b) in rules {
            // This means that b depends on a.
            graph.entry(b).or_insert_with(HashSet::new).insert(a);
            // Ensure a is also a key in the graph (even if it has no dependencies).
            // This is will be useful for Kahn's Algorithm.
            graph.entry(a).or_insert_with(HashSet::new);
        }

        Self { rules: graph }
    }

    /// Shrinks the graph to only include nodes and dependencies relevant to a specific sequence.
    ///
    /// For a given sequence of pages, this method produces a subgraph containing only
    /// the nodes in the sequence and their dependencies (also restricted to the sequence).
    ///
    /// # Parameters
    /// - `sequence`: A slice of `usize` representing the pages in the sequence.
    ///
    /// # Returns
    /// A new `Graph` containing only the relevant nodes and dependencies.
    fn shrink(&self, sequence: &[usize]) -> Self {
        // Create a new adjacency list containing only the nodes in the provided sequence.
        let mut shrunk_graph = HashMap::new();

        for &page in sequence {
            if let Some(deps) = self.rules.get(&page) {
                let filtered_deps: HashSet<usize> = deps
                    .iter()
                    .filter(|&&dep| sequence.contains(&dep))
                    .copied()
                    .collect();
                shrunk_graph.insert(page, filtered_deps);
            }
        }

        Self {
            rules: shrunk_graph,
        }
    }

    /// Displays the adjacency list of the graph in a readable format.
    ///
    /// The output lists each node followed by its dependencies, sorted for readability.
    /// Example output:
    /// ```
    /// 13 -> []
    /// 29 -> [13]
    /// 47 -> [13, 29, 53, 61]
    /// ```
    fn dispaly(&self) {
        let mut keys = self.rules.keys().collect::<Vec<&usize>>();
        keys.sort();
        for key in keys {
            let mut sorted_key = self
                .rules
                .get(key)
                .unwrap()
                .iter()
                .cloned()
                .collect::<Vec<usize>>();

            sorted_key.sort();
            println!("{} -> {:?}", key, sorted_key);
        }
    }

    /// Validates whether a given sequence respects the page ordering rules.
    ///
    /// A sequence is valid if, for every page and its dependencies in the graph,
    /// the dependencies appear before the page in the sequence.
    ///
    /// # Parameters
    /// - `sequence`: A slice of `usize` representing the sequence to validate.
    ///
    /// # Returns
    /// `true` if the sequence respects all ordering rules; `false` otherwise.
    fn validate(&self, sequence: &[usize]) -> bool {
        let graph = self.shrink(sequence);

        // Store every item's position in sequnce.
        let mut seq_positions = HashMap::new();
        for (idx, page) in sequence.iter().enumerate() {
            seq_positions.insert(page, idx);
        }

        for (page, deps) in graph.rules.iter() {
            if let Some(page_pos) = seq_positions.get(page) {
                for dep in deps.iter() {
                    if let Some(dep_pos) = seq_positions.get(dep) {
                        if page_pos < dep_pos {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    /// Produces a topologically sorted sequence of pages based on the given sequence.
    ///
    /// This method uses Kahn's algorithm to compute a valid ordering of the pages
    /// while respecting the graph's dependencies.
    ///
    /// # Parameters
    /// - `sequence`: A slice of `usize` representing the pages to sort.
    ///
    /// # Returns
    /// A vector of `usize` representing the topologically sorted sequence.
    fn topological_sort(&self, sequence: &[usize]) -> Vec<usize> {
        // Shrink the universal graph into one per this sequence.
        let graph = self.shrink(sequence);

        // Computing the degrees for all nodes that exist in the adjacency list.
        let mut in_degree = HashMap::new();
        for &node in graph.rules.keys() {
            in_degree.entry(node).or_insert(0);
        }
        for dependencies in graph.rules.values() {
            for &dep in dependencies {
                *in_degree.entry(dep).or_insert(0) += 1;
            }
        }

        // Init queue with pages that has no deps.
        let mut queue = VecDeque::new();
        for (&page, &degree) in in_degree.iter() {
            if degree == 0 {
                queue.push_back(page);
            }
        }

        let mut sorted_seq = Vec::new();
        while let Some(page) = queue.pop_front() {
            if sequence.contains(&page) {
                sorted_seq.push(page);
            }

            // I want to update degrees based on removed item.
            for dep in &graph.rules[&page] {
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(*dep);
                    }
                }
            }
        }

        sorted_seq
    }
}

pub fn read_file(path: &str) -> io::BufReader<fs::File> {
    let file_path = Path::new(&path);
    let file = fs::File::open(file_path)
        .unwrap_or_else(|e| panic!("Failed to read file {}\n{}\n", path, e));

    io::BufReader::new(file)
}

fn get_rules(raw: &str) -> Vec<(usize, usize)> {
    raw.lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn get_sequences(raw: &str) -> Vec<Vec<usize>> {
    raw.lines()
        .map(|l| {
            l.split(",")
                .map(|p| p.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn part_one() -> usize {
    let input = read_file("src/input.txt")
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");

    let (rule_pairs, seqs) = input.split_once("\n\n").unwrap();
    let rules = get_rules(rule_pairs);

    let graph = Graph::new(&rules);

    get_sequences(seqs)
        .iter()
        .filter(|s| graph.validate(s))
        .map(|s| s[s.len() / 2])
        .sum()
}

fn part_two() -> usize {
    let input = read_file("src/input.txt")
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");

    let (rule_pairs, seqs) = input.split_once("\n\n").unwrap();
    let rules = get_rules(rule_pairs);

    let graph = Graph::new(&rules);

    get_sequences(seqs)
        .iter()
        .filter(|s| !graph.validate(s))
        .map(|seq| graph.topological_sort(seq))
        .map(|s| s[s.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_two() {
        let output = part_two();
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_part_one() {
        let output = part_one();
        let expected = 0;
        assert_eq!(output, expected);
    }
}

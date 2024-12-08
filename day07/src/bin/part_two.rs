use day07::Solver;
use std::io::BufRead;
use util::read_file;

fn main() {
    let solver = Solver::with_concat();
    let result: usize = read_file("input/07.txt")
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let (target, sequence) = line.split_once(": ").unwrap();
            (target.to_string(), sequence.to_string())
        })
        .map(|(target, seq)| {
            let target = target.parse::<usize>().unwrap();
            let seq: Vec<usize> = seq
                .split(" ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            (target, seq)
        })
        .filter(|(target, seq)| solver.check(*target, seq))
        .map(|(target, _seq)| target)
        .sum();

    println!("* Solution: {} *", result);
}

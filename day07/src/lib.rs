#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn evalute(&self, first: usize, second: usize) -> usize {
        match self {
            Op::Add => first + second,
            Op::Mul => first * second,
        }
    }
}

struct Permutations {
    op_count: usize,
    state: Vec<Op>,
    idx: usize,
    operators: Vec<Op>,
}

impl Permutations {
    fn new(op_count: usize) -> Self {
        let operators = vec![Op::Add, Op::Mul];
        Self {
            op_count,
            state: vec![Op::Add; op_count],
            idx: 0,
            operators,
        }
    }
    // fn is_valid(&self, target: usize, sequence: &[usize]) -> bool {
    //     let mut result = 0_usize;
    //     for (idx, operands) in sequence.windows(2).enumerate() {
    //         if let [first, second] = operands {
    //             result += self.state[idx].evalute(*first, *second);
    //         }
    //     }
    //
    //     result == target
    // }
}

impl Iterator for Permutations {
    type Item = Vec<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        let base = self.operators.len();

        // If idx exceeds the total number of permutations, terminate
        if self.idx >= base.pow(self.op_count as u32) {
            return None;
        }

        // Update the current state based on idx
        for i in 0..self.op_count {
            // Calculate the current operator index for position i
            let op_idx = (self.idx / base.pow(i as u32)) % base;
            self.state[i] = self.operators[op_idx];
        }

        self.idx += 1;
        Some(self.state.clone())
    }
}

pub struct Solver;

impl Solver {
    pub fn check(target: usize, sequence: &[usize]) -> bool {
        let permutations = Permutations::new(sequence.len() - 1);

        for perm in permutations {
            let mut result = sequence[0];
            for (&input, op) in sequence.iter().skip(1).zip(perm) {
                result = op.evalute(result, input);
            }
            if target == result {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_op() {
        let op = Op::Add;
        assert_eq!(op.evalute(81, 40), 121);

        let op = Op::Mul;
        assert_eq!(op.evalute(5, 4), 20);
    }

    #[test]
    fn test_part_one() {
        let input: usize = SAMPLE
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(target, seq)| {
                let target = target.parse::<usize>().unwrap();
                let seq: Vec<usize> = seq
                    .split(" ")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect();
                (target, seq)
            })
            .filter(|(target, seq)| Solver::check(*target, seq))
            .map(|(target, seq)| target)
            .sum();

        assert_eq!(input, 3749);
    }
}

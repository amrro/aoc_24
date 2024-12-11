use std::collections::HashMap;

pub struct StoneBlinker {
    transfomer: HashMap<usize, Vec<usize>>,
}

impl StoneBlinker {
    fn new() -> Self {
        // seed the table with some frequent values.
        let dp = HashMap::from([
            (0, vec![1]),
            (1, vec![2024]),
            (2, vec![4048]),
            (4, vec![8096]),
            (6, vec![12144]),
            (8, vec![16192]),
            (9, vec![18216]),
            (2024, vec![20, 24]),
            (4048, vec![40, 48]),
            (8096, vec![80, 96]),
        ]);

        Self { transfomer: dp }
    }

    #[inline]
    /// Splits `usize` number with into two `usize` halves.
    fn split(value: String) -> (usize, usize) {
        let mid_idx = value.len() / 2;
        let (first, second) = (&value[..mid_idx], &value[mid_idx..]);
        let first = first.parse::<usize>().unwrap();
        let second = second.parse::<usize>().unwrap();
        (first, second)
    }

    /// Blinks at single stone.
    fn blink_at(stone: usize) -> Vec<usize> {
        // No need to calculate if it's equals to `0`, since it's already seeded
        // into the transformer `HashMap`.
        if stone.to_string().len() % 2 == 0 {
            let string = stone.to_string();
            let (first, second) = Self::split(string);
            vec![first, second]
        } else {
            vec![stone * 2024]
        }
    }

    fn get(&mut self, stone: usize) -> Vec<usize> {
        if let Some(stones) = self.transfomer.get(&stone) {
            return stones.clone();
        }

        let calc = Self::blink_at(stone);
        self.transfomer.insert(stone, calc.clone());

        calc
    }
}

pub struct Stones {
    freqs: HashMap<usize, usize>,
    blinker: StoneBlinker,
}

impl Stones {
    pub fn new(input: &str) -> Self {
        let stones: Vec<_> = input
            .trim()
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut freqs = HashMap::new();
        for stone in stones {
            *freqs.entry(stone).or_insert(usize::default()) += 1;
        }

        Self {
            freqs,
            blinker: StoneBlinker::new(),
        }
    }

    /// Simulates the transformation of all stones for one blink.
    pub fn blinks(&mut self) {
        let mut new_freq = HashMap::new();
        for (stone, count) in self.freqs.drain() {
            let stones_after_blink = self.blinker.get(stone);
            for s in stones_after_blink.into_iter() {
                *new_freq.entry(s).or_insert(0) += count;
            }
        }

        self.freqs.extend(new_freq);
    }

    /// Repeats the blink simulation for a given number of iterations.
    ///
    /// # Returns
    /// The total number of stones after the specified number of blinks.
    pub fn repeat(&mut self, blinks: usize) -> usize {
        (0..blinks).for_each(|_| self.blinks());
        self.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.freqs.values().sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

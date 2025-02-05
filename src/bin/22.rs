#![feature(int_roundings)]

use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

advent_of_code::solution!(22);

trait MixPrune {
    fn mix(&self, val: Self) -> Self;
    fn prune(&self) -> Self;
}

impl MixPrune for usize {
    fn mix(&self, val: Self) -> Self {
        *self ^ val
    }
    fn prune(&self) -> Self {
        *self % 16777216
    }
}
/// - Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
/// - Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
/// - Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
struct PseudoRandom {
    _secret: usize,
}

impl PseudoRandom {
    fn new(seed: usize) -> Self {
        Self { _secret: seed }
    }

    fn _derive(&mut self) {
        let mut state = self._secret;
        state = state.mul(64).mix(state).prune();
        state = state.div_floor(32).mix(state).prune();
        state = state.mul(2048).mix(state).prune();
        self._secret = state;
    }

    fn get(&mut self) -> usize {
        self._derive();
        self._secret
    }

    fn get_price(&mut self) -> u8 {
        let n = self.get();
        (n % 10) as u8
    }

    fn sequences_to_price(&mut self, n: usize) -> HashMap<[i8; 4], u8> {
        assert!(n >= 4);
        let mut max = (self._secret % 10) as u8;
        let mut previous = max;
        let mut curr_seq = [0, 0, 0, 0];
        for i in 0..4 {
            let p = self.get_price();
            curr_seq[i] = (p as i8).checked_sub(previous as i8).unwrap();
            previous = p;
            if p > max {
                max = p;
            }
        }
        let mut sequences = HashMap::new();
        sequences.insert(curr_seq, previous);
        for _ in 4..n {
            let p = self.get_price();
            curr_seq.copy_within(1..4, 0);
            curr_seq[3] = (p as i8).checked_sub(previous as i8).unwrap();
            if p > max {
                max = p;
            }
            sequences.entry(curr_seq).or_insert(p);
            previous = p;
        }
        sequences
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|&l| !l.is_empty())
            .map(|l| PseudoRandom::new(l.parse().unwrap()))
            .map(|mut pr| {
                let mut last = 0;
                for _ in 0..2000 {
                    last = pr.get();
                }
                last
            })
            .sum(),
    )
}

fn best(seq_to_price_maps: Vec<HashMap<[i8; 4], u8>>) -> usize {
    let mut tested_seq = HashSet::new();
    let mut max = 0;
    for seq_to_price_m in &seq_to_price_maps {
        for seq in seq_to_price_m.keys() {
            if !tested_seq.insert(seq) {
                // already tested this seq
                continue;
            }
            let mut total = 0;
            for map in &seq_to_price_maps {
                if let Some(p) = map.get(seq) {
                    total += *p as usize;
                }
            }
            if total > max {
                max = total;
            }
        }
    }
    max
}

pub fn part_two(input: &str) -> Option<usize> {
    let seq_to_price_maps = input
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| PseudoRandom::new(l.parse().unwrap()))
        .map(|mut pr| pr.sequences_to_price(2000))
        .collect();
    let best_price = best(seq_to_price_maps);
    Some(best_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_mix_prune_usize() {
        assert_eq!(42.mix(15), 37);
        assert_eq!(100_000_000.prune(), 16113920);
    }

    #[test]
    fn test_pseudo_random() {
        let mut pseudo_random = PseudoRandom::new(123);
        let expected_numbers = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        for expected in expected_numbers {
            assert_eq!(expected, pseudo_random.get());
        }
    }
}

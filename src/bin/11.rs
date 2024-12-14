use std::collections::BTreeMap;

advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Stone(usize);

impl Stone {
    fn rule1(&self) -> Option<Self> {
        if self.0 == 0 {
            Some(Self(1))
        } else {
            None
        }
    }

    fn rule2(&self) -> Option<(Self, Self)> {
        // dbg!(self.0);
        assert!(self.0 != 0);
        // dbg!(self.0.ilog10());
        let num_count = self.0.ilog10() + 1;
        if num_count % 2 != 0 {
            return None;
        }

        let half_num_count = num_count / 2;
        let right_stone = Stone(self.0 % 10usize.pow(half_num_count));
        let left_stone = Stone(self.0 / 10usize.pow(half_num_count));
        Some((left_stone, right_stone))
    }

    fn rule3(&self) -> Self {
        Self(self.0 * 2024)
    }
}

struct Puzzle(Vec<Stone>);

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let stones = value
            .trim()
            .split(" ")
            .map(|n| Stone(n.parse().unwrap()))
            .collect();
        Self(stones)
    }
}

impl Puzzle {
    // fn mut_blink(&mut self) {
    //     let mut new_stones = Vec::with_capacity(self.0.len());
    //     for stone in &self.0 {
    //         if let Some(n_stone) = stone.rule1() {
    //             new_stones.push(n_stone);
    //         } else if let Some((n_stone1, n_stone2)) = stone.rule2() {
    //             new_stones.push(n_stone1);
    //             new_stones.push(n_stone2);
    //         } else {
    //             new_stones.push(stone.rule3());
    //         }
    //     }
    //     self.0 = new_stones;
    // }

    // fn nblink_stone(stone: Stone, n: usize) -> Vec<Stone> {
    //     let mut stones = vec![stone];
    //     for _ in 0..n {
    //         let mut new_stones = Vec::with_capacity(stones.len());
    //         for stone in &stones {
    //             if let Some(n_stone) = stone.rule1() {
    //                 new_stones.push(n_stone);
    //             } else if let Some((n_stone1, n_stone2)) = stone.rule2() {
    //                 new_stones.push(n_stone1);
    //                 new_stones.push(n_stone2);
    //             } else {
    //                 new_stones.push(stone.rule3());
    //             }
    //         }
    //         stones = new_stones;
    //     }
    //     stones
    // }

    fn cached_blinks(&self, blinks: usize) -> usize {
        let mut cache: BTreeMap<Stone, usize> = BTreeMap::new();
        for &stone in &self.0 {
            cache.entry(stone).and_modify(|e| *e += 1).or_insert(1);
        }

        for _ in 0..blinks {
            let mut new_stone_cache: BTreeMap<Stone, usize> = BTreeMap::new();
            for (stone, c) in cache.into_iter() {
                if c == 0 {
                    continue;
                }
                if let Some(n_stone) = stone.rule1() {
                    new_stone_cache
                        .entry(n_stone)
                        .and_modify(|n_c| *n_c += c)
                        .or_insert(c);
                } else if let Some((n_stone1, n_stone2)) = stone.rule2() {
                    new_stone_cache
                        .entry(n_stone1)
                        .and_modify(|n_c| *n_c += c)
                        .or_insert(c);
                    new_stone_cache
                        .entry(n_stone2)
                        .and_modify(|n_c| *n_c += c)
                        .or_insert(c);
                } else {
                    let n_stone = stone.rule3();
                    new_stone_cache
                        .entry(n_stone)
                        .and_modify(|n_c| *n_c += c)
                        .or_insert(c);
                }
            }
            cache = new_stone_cache;
        }

        cache.into_iter().map(|(_s, c)| c).sum()
    }

    // fn cached_blinks_dirty(&self, blinks: usize, blinks_threshold: usize) -> usize {
    //     // make sure blinks_threshold is a multiplier of blinks
    //     assert!(blinks % blinks_threshold == 0);
    //     // cache that maps Stone -> N_blinked stones (N being the blinks_threshold)
    //     let mut cache_threshold: BTreeMap<Stone, Vec<Stone>> = BTreeMap::new();
    //     // compute first cache
    //     for &stone in &self.0 {
    //         cache_threshold.insert(stone, Self::nblink_stone(stone, blinks_threshold));
    //     }

    //     // full cache construction
    //     let steps = blinks / blinks_threshold;
    //     for _step in 0..steps {
    //         let cache_extension = Arc::new(Mutex::new(BTreeMap::new()));
    //         cache_threshold.par_iter().for_each(|(_, blinked_stones)| {
    //             for &stone in blinked_stones {
    //                 if !cache_threshold.contains_key(&stone)
    //                 /* && !cache_extension.contains_key(&stone) */
    //                 {
    //                     let nblink_stones = Self::nblink_stone(stone, blinks_threshold);
    //                     cache_extension.lock().unwrap().insert(stone, nblink_stones);
    //                 }
    //             }
    //         });

    //         cache_threshold.extend(
    //             Arc::into_inner(cache_extension)
    //                 .unwrap()
    //                 .into_inner()
    //                 .unwrap(),
    //         );
    //     }

    //     fn get_count_for_cache_depth(
    //         stone: &Stone,
    //         cache: &BTreeMap<Stone, Vec<Stone>>,
    //         depth: usize,
    //     ) -> usize {
    //         if depth == 1 {
    //             return cache.get(stone).unwrap().len(); /* [stone].len() */
    //         }
    //         cache
    //             .get(stone)
    //             .unwrap()
    //             .into_par_iter()
    //             .map(|s| get_count_for_cache_depth(s, cache, depth - 1))
    //             .sum()
    //     }
    //     let mut total_count = 0;
    //     for stone in &self.0 {
    //         total_count += get_count_for_cache_depth(stone, &cache_threshold, steps);
    //     }

    //     total_count
    // }

    // fn stone_amount(&self) -> usize {
    //     self.0.len()
    // }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from(input);
    Some(puzzle.cached_blinks(25))
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from(input);
    Some(puzzle.cached_blinks(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

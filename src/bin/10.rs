use std::collections::BTreeSet;

use advent_of_code::{Bound, Dir, Pos};

advent_of_code::solution!(10);

enum Scoring {
    Score,
    Rating,
}

struct Map {
    map: Vec<Vec<u8>>,
    bound: Bound,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect::<Vec<Vec<u8>>>();
        let bound = Bound::from(&map);
        Self { map, bound }
    }
}

impl Map {
    const PEAK: u8 = 9;

    fn get_height(&self, pos: &Pos) -> u8 {
        self.map[pos.y][pos.x]
    }
    fn get_next_tiles_up(&self, pos: &Pos) -> [Option<Pos>; 4] {
        let mut res = [None; 4];
        let mut res_idx = 0;
        let height = self.get_height(pos);
        for dir in Dir::all() {
            if let Some(n_pos) = dir.apply_bounded(&pos, &self.bound) {
                let n_height = self.get_height(&n_pos);
                if n_height == height + 1 {
                    res[res_idx] = Some(n_pos);
                    res_idx += 1;
                    // assert!(res_idx < res.len());
                }
            }
        }
        res
    }
    fn hiking_trail_up_score(&self, start_pos: Pos) -> usize {
        let mut dfs_pile = Vec::new();
        let mut peak_pos = BTreeSet::new();
        dfs_pile.push(start_pos);
        'pile_loop: while let Some(pos) = dfs_pile.pop() {
            if self.get_height(&pos) == Self::PEAK {
                peak_pos.insert(pos);
                continue;
            }

            let next_pos_arr = self.get_next_tiles_up(&pos);
            for next_pos in next_pos_arr {
                match next_pos {
                    // get_next_tiles_up is careful to have all `Some` contiguous
                    None => continue 'pile_loop,
                    Some(next_pos) => dfs_pile.push(next_pos),
                }
            }
        }

        peak_pos.len()
    }
    fn hiking_trail_up_rating(&self, start_pos: Pos) -> usize {
        let mut dfs_pile = Vec::new();
        let mut trailhead_rating = 0;
        dfs_pile.push(start_pos);
        'pile_loop: while let Some(pos) = dfs_pile.pop() {
            if self.get_height(&pos) == Self::PEAK {
                // via the DFS algorithm, if we arrive at pos of height PEAK, we arrived via a unique way
                // so for the `Scoring::Rating`, we don't care to verify the pos, we just want to count
                // how many pathways into the graph allowed us to reach PEAK
                trailhead_rating += 1;
                continue;
            }

            let next_pos_arr = self.get_next_tiles_up(&pos);
            for next_pos in next_pos_arr {
                match next_pos {
                    // get_next_tiles_up is careful to have all `Some` contiguous
                    None => continue 'pile_loop,
                    Some(next_pos) => dfs_pile.push(next_pos),
                }
            }
        }
        trailhead_rating
    }
    fn hiking_trail(&self, scoring: Scoring) -> usize {
        let mut score = 0;
        for (y, line) in self.map.iter().enumerate() {
            for (x, &h) in line.iter().enumerate() {
                // start point for hiking
                if h == 0 {
                    let start_pos = Pos { x, y };
                    match scoring {
                        Scoring::Score => score += self.hiking_trail_up_score(start_pos),
                        Scoring::Rating => score += self.hiking_trail_up_rating(start_pos),
                    }
                }
            }
        }
        score
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.hiking_trail(Scoring::Score))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.hiking_trail(Scoring::Rating))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

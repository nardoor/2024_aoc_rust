use std::collections::{BTreeMap, VecDeque};

use advent_of_code::{Bound, Dir, Pos};

advent_of_code::solution!(18);

#[cfg(test)]
const MEMORY_GRID_SIZE: usize = 6 + 1;

#[cfg(not(test))]
const MEMORY_GRID_SIZE: usize = 70 + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Corrupted,
}

struct ByteCascade {
    bytes: Vec<Pos>,
}

impl From<&str> for ByteCascade {
    fn from(value: &str) -> Self {
        Self {
            bytes: value.lines().map(Pos::from).collect(),
        }
    }
}

struct MemoryRegion {
    pos: Pos,
    byte_cascade: ByteCascade,
    region: [[Tile; MEMORY_GRID_SIZE]; MEMORY_GRID_SIZE],
}

impl MemoryRegion {
    fn shortest_after_n_bytes(&self, n_bytes: usize) -> usize {
        let mut sim_region = self.region.clone();
        for byte_pos in self.byte_cascade.bytes[0..n_bytes].into_iter() {
            sim_region[byte_pos.y][byte_pos.x] = Tile::Corrupted;
        }

        let dest = Pos {
            x: MEMORY_GRID_SIZE - 1,
            y: MEMORY_GRID_SIZE - 1,
        };
        let bound = Bound {
            x_bound: MEMORY_GRID_SIZE,
            y_bound: MEMORY_GRID_SIZE,
        };

        let mut state_to_explore = VecDeque::new();
        state_to_explore.push_front((self.pos.clone(), 0));
        let mut history = BTreeMap::new();
        history.insert(self.pos.clone(), 0);

        while let Some((pos, len)) = state_to_explore.pop_back() {
            if pos == dest {
                return len;
            }
            let n_len = len + 1;
            for dir in Dir::all() {
                let Some(n_pos) = dir.apply_bounded(&pos, &bound) else {
                    continue;
                };

                if sim_region[n_pos.y][n_pos.x] == Tile::Corrupted {
                    continue;
                }

                if let Some(&hist_len) = history.get(&n_pos) {
                    if hist_len <= n_len {
                        continue;
                    }
                }

                history.insert(n_pos, n_len);
                state_to_explore.push_front((n_pos, n_len));
            }
        }
        panic!("didn't reach dest!");
    }

    fn coordinates_that_block_exit(&self, n_bytes: usize) -> Pos {
        let mut sim_region = self.region.clone();
        for byte_pos in self.byte_cascade.bytes[0..n_bytes].into_iter() {
            sim_region[byte_pos.y][byte_pos.x] = Tile::Corrupted;
        }

        let dest = Pos {
            x: MEMORY_GRID_SIZE - 1,
            y: MEMORY_GRID_SIZE - 1,
        };
        let bound = Bound {
            x_bound: MEMORY_GRID_SIZE,
            y_bound: MEMORY_GRID_SIZE,
        };

        'cascade: for n_byte_pos in self.byte_cascade.bytes[n_bytes..].into_iter() {
            sim_region[n_byte_pos.y][n_byte_pos.x] = Tile::Corrupted;

            let mut state_to_explore = VecDeque::new();
            state_to_explore.push_front((self.pos.clone(), 0));
            let mut history = BTreeMap::new();
            history.insert(self.pos.clone(), 0);

            while let Some((pos, len)) = state_to_explore.pop_back() {
                if pos == dest {
                    continue 'cascade;
                }
                let n_len = len + 1;
                for dir in Dir::all() {
                    let Some(n_pos) = dir.apply_bounded(&pos, &bound) else {
                        continue;
                    };

                    if sim_region[n_pos.y][n_pos.x] == Tile::Corrupted {
                        continue;
                    }

                    if let Some(&hist_len) = history.get(&n_pos) {
                        if hist_len <= n_len {
                            continue;
                        }
                    }

                    history.insert(n_pos, n_len);
                    state_to_explore.push_front((n_pos, n_len));
                }
            }
            /* didn't reach exit */
            return n_byte_pos.clone();
        }
        panic!("didn't end in 'cascade: loop!");
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let byte_cascade = ByteCascade::from(input);
    let memory_region = MemoryRegion {
        byte_cascade,
        pos: Pos { x: 0, y: 0 },
        region: [[Tile::Empty; MEMORY_GRID_SIZE]; MEMORY_GRID_SIZE],
    };

    #[cfg(test)]
    let n_bytes = 12;
    #[cfg(not(test))]
    let n_bytes = 1024;

    Some(memory_region.shortest_after_n_bytes(n_bytes))
}

pub fn part_two(input: &str) -> Option<String> {
    let byte_cascade = ByteCascade::from(input);
    let memory_region = MemoryRegion {
        byte_cascade,
        pos: Pos { x: 0, y: 0 },
        region: [[Tile::Empty; MEMORY_GRID_SIZE]; MEMORY_GRID_SIZE],
    };

    #[cfg(test)]
    let n_bytes = 12;
    #[cfg(not(test))]
    let n_bytes = 1024;

    let blocking_coords = memory_region.coordinates_that_block_exit(n_bytes);
    Some(format!("{},{}", blocking_coords.x, blocking_coords.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}

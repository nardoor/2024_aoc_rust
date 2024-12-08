use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Antenna(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            c => Self::Antenna(c),
        }
    }
}

struct Map {
    // map: Vec<Vec<Tile>>,
    antennas: HashMap<char, BTreeSet<Pos>>,
    bounds: Bounds,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map::new_from_map(
            value
                .lines()
                .map(|l| l.chars().map(Tile::from).collect())
                .collect(),
        )
    }
}

impl Map {
    fn new_from_map(map: Vec<Vec<Tile>>) -> Self {
        let mut antennas = HashMap::new();
        map.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, &t)| match t {
                Tile::Antenna(c) => {
                    antennas
                        .entry(c)
                        .and_modify(|v: &mut BTreeSet<Pos>| {
                            v.insert(Pos { x, y });
                        })
                        .or_insert_with(|| {
                            let mut set = BTreeSet::new();
                            set.insert(Pos { x, y });
                            set
                        });
                }
                _ => (),
            })
        });
        let bounds = Bounds {
            x: map[0].len(),
            y: map.len(),
        };
        Self {
            // map,
            antennas,
            bounds,
        }
    }

    /// part 1
    fn count_antinode(&self) -> usize {
        let mut spots: BTreeSet<Pos> = BTreeSet::new();

        // go over each antenna
        for antenna in self.antennas.keys() {
            let set = self.antennas.get(antenna).unwrap();

            for pair in set.iter().combinations(2) {
                assert_eq!(pair.len(), 2);
                let antenna_a = *pair[0];
                let antenna_b = *pair[1];

                let a_to_b = DirVec::new(antenna_a, antenna_b);

                if let Some(antinode1) = a_to_b
                    .apply(antenna_b)
                    .filter(|&p| self.bounds.check(p).is_some())
                {
                    spots.insert(antinode1);
                }

                if let Some(antinode2) = a_to_b
                    .opposite()
                    .apply(antenna_a)
                    .filter(|&p| self.bounds.check(p).is_some())
                {
                    spots.insert(antinode2);
                }
            }
        }

        spots.len()
    }

    /// part 2
    fn count_antinode_new_model(&self) -> usize {
        let mut spots: BTreeSet<Pos> = BTreeSet::new();

        // go over each antenna
        for antenna in self.antennas.keys() {
            let set = self.antennas.get(antenna).unwrap();

            for pair in set.iter().combinations(2) {
                assert_eq!(pair.len(), 2);
                let antenna_a = *pair[0];
                let antenna_b = *pair[1];

                let a_to_b = DirVec::new(antenna_a, antenna_b);

                let mut current = antenna_a;

                while let Some(antinode1) = a_to_b
                    .apply(current)
                    .filter(|&p| self.bounds.check(p).is_some())
                {
                    current = antinode1;
                    spots.insert(antinode1);
                }

                current = antenna_b;
                let b_to_a = a_to_b.opposite();
                while let Some(antinode2) = b_to_a
                    .apply(current)
                    .filter(|&p| self.bounds.check(p).is_some())
                {
                    current = antinode2;
                    spots.insert(antinode2);
                }
            }
        }

        spots.len()
    }
}

struct Bounds {
    x: usize,
    y: usize,
}
impl Bounds {
    fn check(&self, pos: Pos) -> Option<Pos> {
        if pos.x >= self.x || pos.y >= self.y {
            None
        } else {
            Some(pos)
        }
    }
}

#[derive(Clone, Copy)]
struct DirVec {
    dx: isize,
    dy: isize,
}

impl DirVec {
    fn new(a: Pos, b: Pos) -> Self {
        let res = Self {
            dx: b.x as isize - a.x as isize,
            dy: b.y as isize - a.y as isize,
        };

        // debug
        assert_eq!(res.apply(a).unwrap(), b);
        res
    }

    fn opposite(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }

    /* wasn't needed */
    // fn minimize(self) -> Self {
    //     // ugly get divisors of dx and dy, then search commons:
    //     let dx_divisors: Vec<isize> = (0..self.dx)
    //         .filter(|&n| self.dx.rem_euclid(n) == 0)
    //         .collect();

    //     let dy_divisors = (0..self.dy).filter(|&n| self.dy.rem_euclid(n) == 0);

    //     // find common divisors
    //     let mut common_divisors: Vec<isize> =
    //         dy_divisors.filter(|n| dx_divisors.contains(n)).collect();
    //     common_divisors.sort();
    //     match common_divisors.last() {
    //         Some(&div) => Self {
    //             dx: self.dx.div_euclid(div),
    //             dy: self.dy.div_euclid(div),
    //         },
    //         None => self,
    //     }
    // }

    fn apply(&self, pos: Pos) -> Option<Pos> {
        Some(Pos {
            x: pos.x.checked_add_signed(self.dx)?,
            y: pos.y.checked_add_signed(self.dy)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            ord => ord,
        }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.count_antinode())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.count_antinode_new_model())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

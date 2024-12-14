use std::collections::{BTreeSet, VecDeque};

use advent_of_code::{Bound, Dir, Pos};
use itertools::Itertools;

advent_of_code::solution!(12);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range(usize, usize);

impl Range {
    fn range_dist(&self, val: usize) -> usize {
        if self.0 <= val && val <= self.1 {
            return 0;
        } else if self.0 > val {
            self.0 - val
        } else if val > self.1 {
            val - self.1
        } else {
            unreachable!()
        }
        // self.0.saturating_sub(val).min(val.saturating_sub(self.1))
    }

    fn extend(&mut self, val: usize) {
        self.0 = self.0.min(val);
        self.1 = self.1.max(val);
    }

    fn is_next_to(&self, other: &Self) -> bool {
        return self.range_dist(other.0) <= 1 || self.range_dist(other.1) <= 1;
    }

    fn merge(&self, other: &Self) -> Self {
        Self(self.0.min(other.0), self.1.max(other.1))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Side {
    dir: Dir,
    dir_axis_coord: usize,
    side_range: Range,
}

impl Side {
    fn new(pos: &Pos, dir: Dir) -> Self {
        match dir {
            Dir::Down | Dir::Up => Self {
                dir,
                dir_axis_coord: pos.y,
                side_range: Range(pos.x, pos.x),
            },
            Dir::Left | Dir::Right => Self {
                dir,
                dir_axis_coord: pos.x,
                side_range: Range(pos.y, pos.y),
            },
        }
    }

    fn extend(&mut self, pos: &Pos) {
        match self.dir {
            Dir::Down | Dir::Up => {
                self.side_range.extend(pos.x);
            }
            Dir::Left | Dir::Right => {
                self.side_range.extend(pos.y);
            }
        }
    }

    fn is_in_side(&self, pos: &Pos, dir: Dir) -> bool {
        if dir != self.dir {
            return false;
        }

        match dir {
            Dir::Down | Dir::Up => {
                if self.dir_axis_coord == pos.y && self.side_range.range_dist(pos.x) <= 1 {
                    true
                } else {
                    false
                }
            }
            Dir::Left | Dir::Right => {
                if self.dir_axis_coord == pos.x && self.side_range.range_dist(pos.y) <= 1 {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn should_be_merged(&self, side: &Side) -> bool {
        if self.dir != side.dir {
            return false;
        }
        if self.dir_axis_coord != side.dir_axis_coord {
            return false;
        }
        return self.side_range.is_next_to(&side.side_range);
    }

    fn merge(&self, side: &Side) -> Self {
        assert!(self.should_be_merged(side));
        return Self {
            dir: self.dir,
            dir_axis_coord: self.dir_axis_coord,
            side_range: self.side_range.merge(&side.side_range),
        };
    }
}

struct Map {
    map: Vec<Vec<char>>,
    bound: Bound,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map = value.lines().map(|l| l.chars().collect()).collect();
        let bound = Bound::from(&map);
        Map { map, bound }
    }
}

impl Map {
    fn get(&self, pos: Pos) -> char {
        self.map[pos.y][pos.x]
    }

    /// ## Returns
    /// (pos_scanned, area, perimeter)
    fn area_perimeter_at(&self, start_pos: Pos) -> (BTreeSet<Pos>, usize, usize) {
        let mut perimeter = 0;
        let mut area = 0;
        let plant = self.get(start_pos);
        let mut scanned_pos = BTreeSet::new();

        let mut to_scan_pos = vec![start_pos];

        while let Some(pos) = to_scan_pos.pop() {
            if !scanned_pos.insert(pos) {
                continue;
            }
            area += 1;
            let mut potential_perimeter = 4;
            for dir in Dir::all() {
                if let Some(side_pos) = dir.apply_bounded(&pos, &self.bound) {
                    let side_plant = self.get(side_pos);
                    if side_plant == plant {
                        if !scanned_pos.contains(&side_pos) {
                            to_scan_pos.push(side_pos);
                        }
                        potential_perimeter -= 1;
                    }
                }
            }
            perimeter += potential_perimeter;
        }
        (scanned_pos, area, perimeter)
    }

    fn fence_price(&self) -> usize {
        let mut scanned_pos = BTreeSet::new();
        let mut fence_price = 0;
        for y in 0..self.bound.y_bound {
            for x in 0..self.bound.x_bound {
                let pos = Pos { x, y };
                if scanned_pos.contains(&pos) {
                    continue;
                }
                let (mut newly_scanned_pos, area, perimeter) = self.area_perimeter_at(pos);
                scanned_pos.append(&mut newly_scanned_pos);
                fence_price += area * perimeter;
            }
        }
        fence_price
    }

    fn merge_sides(sides: Vec<Side>) -> Vec<Side> {
        let mut current_sides;
        let mut new_sides = sides;
        loop {
            let mut merged_some = false;

            current_sides = new_sides.clone();
            for sides_combinations in current_sides.iter().combinations(2) {
                let side_a = sides_combinations[0];
                let side_b = sides_combinations[1];

                if side_a.should_be_merged(side_b) {
                    let merged = side_a.merge(side_b);
                    new_sides.remove(new_sides.iter().find_position(|&s| s == side_a).unwrap().0);
                    new_sides.remove(new_sides.iter().find_position(|&s| s == side_b).unwrap().0);
                    new_sides.push(merged);
                    merged_some = true;
                }
            }

            if !merged_some {
                break;
            }
        }
        new_sides
    }

    /// ## Returns
    /// (pos_scanned, area, sides)
    fn area_sides_at(&self, start_pos: Pos) -> (BTreeSet<Pos>, usize, usize) {
        let mut area = 0;
        let plant = self.get(start_pos);
        let mut scanned_pos = BTreeSet::new();

        let mut to_scan_pos = VecDeque::new();
        to_scan_pos.push_front(start_pos);
        let mut known_sides: Vec<Side> = Vec::new();
        while let Some(pos) = to_scan_pos.pop_back() {
            if !scanned_pos.insert(pos) {
                continue;
            }
            area += 1;

            for dir in Dir::all() {
                match dir.apply_bounded(&pos, &self.bound) {
                    Some(side_pos) if self.get(side_pos) == plant => {
                        if !scanned_pos.contains(&side_pos) {
                            to_scan_pos.push_front(side_pos);
                        }
                    }
                    None | Some(_) => {
                        // look for side and extend it
                        if let Some(known_side) = known_sides
                            .iter_mut()
                            .find(|side| side.dir == dir && side.is_in_side(&pos, dir))
                        {
                            known_side.extend(&pos);
                        } else {
                            // else create it
                            known_sides.push(Side::new(&pos, dir))
                        }
                    }
                }
            }
        }
        // dbg!(plant);
        // dbg!(area);
        // dbg!(&known_sides.len());
        if plant == 'R' {
            // dbg!(&known_sides);
        }
        let known_sides = Self::merge_sides(known_sides);
        let sides = known_sides.len();

        if sides % 2 != 0 {
            dbg!(start_pos);
            dbg!(plant);
            dbg!(&known_sides.len());
            dbg!(&known_sides);
        }
        assert!(sides % 2 == 0);

        (scanned_pos, area, sides)
    }

    fn fence_discount_price(&self) -> usize {
        let mut scanned_pos = BTreeSet::new();
        let mut fence_price = 0;
        for y in 0..self.bound.y_bound {
            for x in 0..self.bound.x_bound {
                let pos = Pos { x, y };
                if scanned_pos.contains(&pos) {
                    continue;
                }
                let (mut newly_scanned_pos, area, sides) = self.area_sides_at(pos);
                scanned_pos.append(&mut newly_scanned_pos);
                fence_price += area * sides;
            }
        }
        fence_price
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.fence_price())
}

// 839045 too high
pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from(input);
    Some(map.fence_discount_price())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let range = Range(2, 10);
        assert_eq!(range.range_dist(2), 0);
        assert_eq!(range.range_dist(4), 0);
        assert_eq!(range.range_dist(10), 0);
        assert_eq!(range.range_dist(1), 1);
        assert_eq!(range.range_dist(11), 1);
        assert_eq!(range.range_dist(15), 5);
        assert_eq!(range.range_dist(0), 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_b() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"#;
        let result = part_two(input);
        assert_eq!(result, Some(368));

        let e_input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;
        let result = part_two(e_input);
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_c() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(5402));
    }
}

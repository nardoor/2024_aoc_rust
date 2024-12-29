use advent_of_code::{Bound, Dir, Pos};

advent_of_code::solution!(20);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Start,
    End,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!(),
        }
    }
}

struct Map {
    map: Vec<Vec<Tile>>,
    bound: Bound,
    start: Pos,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut start = None;
        let map = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let t = Tile::from(c);
                        match t {
                            Tile::Start => start = Some(Pos { x, y }),
                            _ => (),
                        }
                        t
                    })
                    .collect()
            })
            .collect();
        let bound = Bound::from(&map);
        Self {
            map,
            bound,
            start: start.unwrap(),
        }
    }
}

impl Map {
    fn get(&self, pos: &Pos) -> Tile {
        self.map[pos.y][pos.x]
    }
    fn get_path(&self) -> Vec<Pos> {
        let mut path = Vec::new();
        let mut last = None;
        let mut curr = self.start;
        path.push(curr);
        'main_loop: while self.get(&curr) != Tile::End {
            for dir in Dir::all() {
                let Some(n_pos) = dir.apply_bounded(&curr, &self.bound) else {
                    continue;
                };
                match self.get(&n_pos) {
                    Tile::Empty => match last {
                        None => {
                            last = Some(curr);
                            path.push(n_pos);
                            curr = n_pos;
                        }
                        Some(inner_last) => {
                            if inner_last == n_pos {
                                /* do not turn around */
                                continue;
                            } else {
                                last = Some(curr);
                                path.push(n_pos);
                                curr = n_pos;
                            }
                        }
                    },
                    Tile::End => {
                        path.push(n_pos);
                        break 'main_loop;
                    }
                    _ => continue,
                }
            }
        }
        path
    }
}

mod shortcut {
    use advent_of_code::{DirVec, Pos};

    pub fn shortcut_between_two_pos(pos1: &Pos, pos2: &Pos) -> bool {
        let dir_vec = DirVec::new(*pos1, *pos2);
        if dir_vec.dx != 0 && dir_vec.dy != 0 {
            /* shortcut can only happen in a horizontal or vertical line */
            return false;
        }
        if dir_vec.dx.abs() == 2 || dir_vec.dy.abs() == 2 {
            return true;
        }
        return false;
    }

    pub fn long_shortcut_between_two_pos(pos1: &Pos, pos2: &Pos) -> Option<usize> {
        let dir_vec = DirVec::new(*pos1, *pos2);
        if dir_vec.dx.abs() + dir_vec.dy.abs() > 20 {
            return None;
        } else {
            return Some(dir_vec.dx.abs() as usize + dir_vec.dy.abs() as usize);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::from(input);
    let path = map.get_path();
    // dbg!(path);
    let mut shortcut_count = 0;
    for (i, pos) in path.iter().enumerate() {
        for (ni, n_pos) in path.iter().enumerate().skip(i + 3) {
            if shortcut::shortcut_between_two_pos(pos, n_pos) {
                let won_time = ni - i - 2;
                if won_time < 100 {
                    continue;
                }
                // println!("Shortcut: {pos:?} -> {n_pos:?} (skip of {won_time})");
                shortcut_count += 1;
            }
        }
    }
    Some(shortcut_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::from(input);
    let path = map.get_path();
    // dbg!(path);
    let mut shortcut_count = 0;
    for (i, pos) in path.iter().enumerate() {
        for (ni, n_pos) in path.iter().enumerate().skip(i + 3) {
            if let Some(shortcut_duration) = shortcut::long_shortcut_between_two_pos(pos, n_pos) {
                let won_time = ni - i - shortcut_duration;
                if won_time < 100 {
                    continue;
                }
                // println!("Shortcut: {pos:?} -> {n_pos:?} (skip of {won_time})");
                shortcut_count += 1;
            }
        }
    }
    Some(shortcut_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0)); /* 0 because of > 100 ps condition for full data */
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0)); /* 0 because of > 100 ps condition for full data */
    }
}

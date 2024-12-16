use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

use advent_of_code::{Dir, FromChar, Pos};

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Empty,
    BigBoxL,
    BigBoxR,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Box => f.write_char('O'),
            Tile::Empty => f.write_char('.'),
            Tile::Wall => f.write_char('#'),
            Tile::BigBoxL => f.write_char('['),
            Tile::BigBoxR => f.write_char(']'),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct BigPoxPos {
    left_pos: Pos,
}

impl BigPoxPos {
    fn from_left(left_pos: Pos) -> Self {
        Self { left_pos }
    }

    fn from_right(right_pos: Pos) -> Self {
        Self {
            left_pos: Dir::Left.apply(&right_pos).unwrap(),
        }
    }

    fn get_left_pos(&self) -> Pos {
        self.left_pos
    }

    fn get_right_pos(&self) -> Pos {
        Dir::Right.apply(&self.left_pos).unwrap()
    }

    fn next_pos(&self, dir: Dir, out_pos: &mut [Pos; 2]) -> usize {
        match dir {
            Dir::Left => {
                let Some(n_pos) = dir.apply(&self.get_left_pos()) else {
                    return 0;
                };
                out_pos[0] = n_pos;
                return 1;
            }
            Dir::Right => {
                let Some(n_pos) = dir.apply(&self.get_right_pos()) else {
                    return 0;
                };
                out_pos[0] = n_pos;
                return 1;
            }
            Dir::Up | Dir::Down => {
                let Some(n_left_pos) = dir.apply(&self.get_left_pos()) else {
                    return 0;
                };
                let n_right_pos = dir.apply(&self.get_right_pos()).unwrap();
                out_pos[0] = n_left_pos;
                out_pos[1] = n_right_pos;
                return 2;
            }
        }
    }
}

struct Puzzle {
    map: Vec<Vec<Tile>>,
    robot: Pos,
}

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let mut robot = None;
        let map = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '.' => Tile::Empty,
                        '@' => {
                            robot = Some(Pos { x, y });
                            Tile::Empty
                        }
                        other => panic!("unexpected {other}"),
                    })
                    .collect()
            })
            .collect();
        Self {
            map,
            robot: robot.unwrap(),
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, tiles) in self.map.iter().enumerate() {
            for (x, t) in tiles.into_iter().enumerate() {
                if self.robot.x == x && self.robot.y == y {
                    f.write_char('@')?;
                } else {
                    t.fmt(f)?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

struct MoveError {}

impl Puzzle {
    fn from_expanded(value: &str) -> Self {
        let mut robot = None;
        let map = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => [Tile::Wall, Tile::Wall],
                        'O' => [Tile::BigBoxL, Tile::BigBoxR],
                        '.' => [Tile::Empty, Tile::Empty],
                        '@' => {
                            robot = Some(Pos { x: 2 * x, y });
                            [Tile::Empty, Tile::Empty]
                        }
                        other => panic!("unexpected {other}"),
                    })
                    .flatten()
                    .collect()
            })
            .collect();
        Self {
            map,
            robot: robot.unwrap(),
        }
    }

    fn get(&self, pos: Pos) -> Tile {
        self.map[pos.y][pos.x]
    }

    fn get_mut(&mut self, pos: Pos) -> &mut Tile {
        &mut self.map[pos.y][pos.x]
    }

    fn move_box(&mut self, box_pos: Pos, n_pos: Pos) {
        assert!(self.get(box_pos) == Tile::Box);
        assert!(self.get(n_pos) == Tile::Empty);

        let r#box = self.get_mut(box_pos) as *mut Tile;
        let n_box = self.get_mut(n_pos) as *mut Tile;

        unsafe {
            std::ptr::swap(r#box, n_box);
        }
    }

    fn move_big_box(&mut self, big_box_pos: BigPoxPos, dir: Dir) {
        assert!(self.get(big_box_pos.get_left_pos()) == Tile::BigBoxL);
        assert!(self.get(big_box_pos.get_right_pos()) == Tile::BigBoxR);

        match dir {
            Dir::Down | Dir::Up => {
                let n_left_pos = dir.apply(&big_box_pos.get_left_pos()).unwrap();
                let n_right_pos = dir.apply(&big_box_pos.get_right_pos()).unwrap();
                assert!(self.get(n_left_pos) == Tile::Empty);
                assert!(self.get(n_right_pos) == Tile::Empty);

                let left = self.get_mut(big_box_pos.get_left_pos()) as *mut Tile;
                let right = self.get_mut(big_box_pos.get_right_pos()) as *mut Tile;
                let n_left = self.get_mut(n_left_pos) as *mut Tile;
                let n_right = self.get_mut(n_right_pos) as *mut Tile;
                unsafe {
                    std::ptr::swap(left, n_left);
                    std::ptr::swap(right, n_right);
                }
            }
            Dir::Left => {
                let n_left_pos = dir.apply(&big_box_pos.get_left_pos()).unwrap();
                assert!(self.get(n_left_pos) == Tile::Empty);
                let left = self.get_mut(big_box_pos.get_left_pos()) as *mut Tile;
                let right = self.get_mut(big_box_pos.get_right_pos()) as *mut Tile;
                let n_left = self.get_mut(n_left_pos) as *mut Tile;
                unsafe {
                    std::ptr::swap(n_left, left);
                    std::ptr::swap(left, right);
                }
            }
            Dir::Right => {
                let n_right_pos = dir.apply(&big_box_pos.get_right_pos()).unwrap();
                assert!(self.get(n_right_pos) == Tile::Empty);
                let right = self.get_mut(big_box_pos.get_right_pos()) as *mut Tile;
                let left = self.get_mut(big_box_pos.get_left_pos()) as *mut Tile;
                let n_right = self.get_mut(n_right_pos) as *mut Tile;
                unsafe {
                    std::ptr::swap(n_right, right);
                    std::ptr::swap(right, left);
                }
            }
        }
    }

    fn rec_move_big_box(&mut self, big_box_pos: BigPoxPos, dir: Dir) -> Result<(), MoveError> {
        let mut box_to_move = Vec::new();
        // breadth first "search"
        let mut box_to_check = VecDeque::new();
        box_to_check.push_front(big_box_pos);
        while let Some(box_pos) = box_to_check.pop_back() {
            if box_to_move.contains(&box_pos) {
                continue;
            }
            box_to_move.push(box_pos);
            let mut out_pos: [Pos; 2] = [Pos { x: 0, y: 0 }; 2];
            let n = box_pos.next_pos(dir, &mut out_pos);
            for i in 0..n {
                let n_pos = out_pos[i];
                match self.get(n_pos) {
                    Tile::Box => panic!("unsupported"),
                    Tile::Empty => (),
                    Tile::Wall => return Err(MoveError {}), // can't move
                    Tile::BigBoxL => {
                        let n_big_box = BigPoxPos::from_left(n_pos);
                        if !box_to_check.contains(&n_big_box) {
                            box_to_check.push_front(n_big_box);
                        }
                    }
                    Tile::BigBoxR => {
                        let n_big_box = BigPoxPos::from_right(n_pos);
                        if !box_to_check.contains(&n_big_box) {
                            box_to_check.push_front(n_big_box);
                        }
                    }
                }
            }
        }
        // all boxes check, now we move them
        for box_pos in box_to_move.into_iter().rev() {
            self.move_big_box(box_pos, dir);
        }
        Ok(())
    }

    fn rec_move_box(&mut self, box_pos: Pos, dir: Dir) -> Result<(), MoveError> {
        assert!(self.get(box_pos) == Tile::Box);

        let n_pos = dir.apply(&box_pos).unwrap();
        match self.get(n_pos) {
            Tile::Empty => {
                // move the box and return ok
                self.move_box(box_pos, n_pos);
                Ok(())
            }
            Tile::Box => {
                // try move the n_box before, if ok, move the box and return ok
                self.rec_move_box(n_pos, dir)?;
                self.move_box(box_pos, n_pos);
                Ok(())
            }
            Tile::Wall => {
                // can't move
                return Err(MoveError {});
            }
            _ => panic!("unsupported"),
        }
    }

    fn move_robot(&mut self, dir: Dir) {
        let n_pos = dir.apply(&self.robot).unwrap();
        match self.get(n_pos) {
            Tile::Empty => self.robot = n_pos,
            Tile::Wall => (), /* skip */
            Tile::Box => {
                match self.rec_move_box(n_pos, dir) {
                    Ok(()) => self.robot = n_pos, // box moved
                    Err(_e) => (),                // box didn't move */
                }
            }
            Tile::BigBoxR => {
                let big_box_pos = BigPoxPos::from_right(n_pos);
                match self.rec_move_big_box(big_box_pos, dir) {
                    Ok(()) => self.robot = n_pos,
                    Err(_e) => (),
                }
            }
            Tile::BigBoxL => {
                let big_box_pos = BigPoxPos::from_left(n_pos);
                match self.rec_move_big_box(big_box_pos, dir) {
                    Ok(()) => self.robot = n_pos,
                    Err(_e) => (),
                }
            }
        }
    }

    fn box_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.into_iter()
                    .enumerate()
                    .filter(|(_x, tile)| **tile == Tile::Box || **tile == Tile::BigBoxL)
                    .map(|(x, _tile)| 100 * y + x)
                    .sum::<usize>()
            })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let moves: Vec<Dir> = moves
        .replace('\n', "")
        .chars()
        .map(Dir::from_char)
        .collect();
    let mut puzzle = Puzzle::from(map);

    // println!("{puzzle}");
    for r#move in moves {
        puzzle.move_robot(r#move);
        // println!("Move {:?}:\n{puzzle}", r#move);
    }

    Some(puzzle.box_gps())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let moves: Vec<Dir> = moves
        .replace('\n', "")
        .chars()
        .map(Dir::from_char)
        .collect();
    let mut puzzle = Puzzle::from_expanded(map);

    // println!("{puzzle}");
    for r#move in moves {
        puzzle.move_robot(r#move);
        // println!("Move {:?}:\n{puzzle}", r#move);
    }

    Some(puzzle.box_gps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small_ex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_one_bigger_ex() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9021));
    }
}

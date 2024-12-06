use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::{collections::HashSet, ops::Add};

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapTile {
    Empty,
    Obstacle,
}

impl TryFrom<char> for MapTile {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(MapTile::Obstacle),
            '.' => Ok(MapTile::Empty),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!(),
        }
    }
}

impl Dir {
    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn all() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
    fn aligned(self, pos1: BoundedPos, pos2: BoundedPos) -> bool {
        match self {
            Self::Up => pos1.x == pos2.x,
            Self::Right => pos1.y == pos2.y,
            Self::Down => pos1.x == pos2.x,
            Self::Left => pos1.y == pos2.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BoundedPos {
    x: usize,
    y: usize,
    // first x value to be invalid
    x_bound: usize,
    // first y value to be invalid
    y_bound: usize,
}

impl PartialOrd for BoundedPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(self.x_bound == other.x_bound);
        assert!(self.y_bound == other.y_bound);

        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            ord => ord,
        }
    }
}

impl Ord for BoundedPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
struct Puzzle {
    guard_pos: BoundedPos,
    guard_dir: Dir,
    map: Vec<Vec<MapTile>>,
}

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let mut guard_pos: Option<(usize, usize)> = None;
        let mut guard_dir: Option<Dir> = None;
        let map: Vec<Vec<MapTile>> = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match MapTile::try_from(c) {
                        Ok(t) => t,
                        Err(c) => {
                            guard_pos = Some((x, y));
                            guard_dir = Some(Dir::from(c));
                            MapTile::Empty
                        }
                    })
                    .collect()
            })
            .collect();

        let y_bound = map.len();
        let x_bound = map[0].len();
        let Some((x, y)) = guard_pos else { panic!() };
        let guard_pos = BoundedPos {
            x,
            y,
            x_bound,
            y_bound,
        };
        Self {
            guard_dir: guard_dir.unwrap(),
            guard_pos: guard_pos,
            map,
        }
    }
}

impl Add<Dir> for BoundedPos {
    type Output = Option<Self>;
    fn add(mut self, rhs: Dir) -> Self::Output {
        let potential_pos = match rhs {
            Dir::Up => (
                self.x.checked_add_signed(0)?,
                self.y.checked_add_signed(-1)?,
            ),
            Dir::Right => (self.x.checked_add_signed(1)?, self.y.checked_add_signed(0)?),
            Dir::Down => (self.x.checked_add_signed(0)?, self.y.checked_add_signed(1)?),
            Dir::Left => (
                self.x.checked_add_signed(-1)?,
                self.y.checked_add_signed(0)?,
            ),
        };

        if potential_pos.0 >= self.x_bound {
            return None;
        }
        if potential_pos.1 >= self.y_bound {
            return None;
        }
        self.x = potential_pos.0;
        self.y = potential_pos.1;
        Some(self)
    }
}

impl Puzzle {
    fn get(&self, pos: BoundedPos) -> MapTile {
        self.map[pos.y][pos.x]
    }
    fn get_mut(&mut self, pos: BoundedPos) -> &mut MapTile {
        &mut self.map[pos.y][pos.x]
    }
    fn next_pos_dir(&self, pos: BoundedPos, dir: Dir) -> Option<(BoundedPos, Dir)> {
        let Some(new_pos) = pos + dir else {
            // pos got out of bound
            return None;
        };

        match self.get(new_pos) {
            MapTile::Empty => Some((new_pos, dir)),
            MapTile::Obstacle => Some((pos, dir.rotate_right())),
        }
    }

    // fn next_guard_pos_dir(&self) -> Option<(BoundedPos, Dir)> {
    //     self.next_pos_dir(self.guard_pos, self.guard_dir)
    // }

    // return amount of steps
    fn progress_until_out(&self) -> usize {
        // seek only unique pos (and not unique (pos, dir))
        self.path_until_out()
            .into_iter()
            .map(|(p, _d)| p)
            .collect::<BTreeSet<BoundedPos>>()
            .len()
    }

    fn path_until_out(&self) -> Vec<(BoundedPos, Dir)> {
        let mut covered_pos = Vec::new();

        let mut current_pos = self.guard_pos;
        let mut current_dir = self.guard_dir;
        covered_pos.push((current_pos, current_dir));

        while let Some((new_pos, new_dir)) = self.next_pos_dir(current_pos, current_dir) {
            current_pos = new_pos;
            current_dir = new_dir;
            covered_pos.push((current_pos, current_dir));
        }

        covered_pos
    }

    /// List **potential** loop-creating obstacle positions.
    // fn list_potential_loops(&self) -> Vec<(BoundedPos, Dir)> {
    //     let covered_pos = self.path_until_out();
    //     let mut potential_pos = Vec::new();
    //     let mut used_obs = HashSet::new();

    //     let y_bound = self.map.len();
    //     let x_bound = self.map[0].len();
    //     for y in 0..y_bound {
    //         for x in 0..x_bound {
    //             let obstacle1 = BoundedPos {
    //                 x,
    //                 y,
    //                 x_bound,
    //                 y_bound,
    //             };

    //             // iterate over all `#`
    //             if self.get(obstacle1) != MapTile::Obstacle {
    //                 continue;
    //             }

    //             // for each dir, suppose guard is going this direction when encountering this obstacle1
    //             for dir in Dir::all() {
    //                 let start_obs = (obstacle1, dir);
    //                 // if used_obs.contains(&(start_obs, 0)) {
    //                 //     continue;
    //                 // }
    //                 used_obs.insert((start_obs, 0));
    //                 let Some(mut current_pos) = obstacle1 + dir.opposite() else {
    //                     // "backward" movement out of bound
    //                     continue;
    //                 };
    //                 let mut current_dir = dir.rotate_right();

    //                 // do not check this obstacle if we won't hit it
    //                 if !covered_pos.contains(&(current_pos, current_dir)) {
    //                     continue;
    //                 }

    //                 // -- DEBUG --
    //                 let Some((new_pos, new_dir)) = self.next_pos_dir(current_pos, dir) else {
    //                     // next_pos shouldn't be out of bound (we shouldn't move, just rotate there)
    //                     unreachable!()
    //                 };
    //                 assert!(new_pos == current_pos);
    //                 assert!(new_dir == dir.rotate_right());
    //                 // -- END DEBUG --

    //                 let mut n_obstacle = Vec::new();
    //                 n_obstacle.push(start_obs);
    //                 // look for (N-1) more obstacles -> total of N -> then deduce potential (N+1)th obstacle for loop
    //                 while let Some((new_current_pos, new_current_dir)) =
    //                     self.next_pos_dir(current_pos, current_dir)
    //                 {
    //                     // rotated => obstacle; note its pos
    //                     if new_current_dir == current_dir.rotate_right() {
    //                         let new_obs = ((current_pos + current_dir).unwrap(), current_dir);
    //                         if n_obstacle.contains(&new_obs) {
    //                             // already in loop
    //                             break;
    //                         }
    //                         // if used_obs.contains(&(new_obs, n_obstacle.len() % 4)) {
    //                         //     // already used
    //                         //     break;
    //                         // }
    //                         n_obstacle.push(((current_pos + current_dir).unwrap(), current_dir));

    //                         // need mini of 3 consecutive obstacles
    //                         // need that N+1 is a multiple of 4 (4 rotations of 90° => loop)
    //                         if n_obstacle.len() >= 3 && (n_obstacle.len() + 1) % 4 == 0 {
    //                             let &before_obs = n_obstacle.last().unwrap();

    //                             let looping_obs = n_obstacle
    //                                 .iter()
    //                                 .filter(|&(_after_pos, after_dir)| {
    //                                     *after_dir == before_obs.1.opposite()
    //                                 })
    //                                 .filter_map(|&after_obs| {
    //                                     Self::compute_nth(before_obs, after_obs)
    //                                 })
    //                                 .for_each(|looping_ops| {
    //                                     if !potential_pos.contains(&looping_ops) {
    //                                         potential_pos.push(looping_ops);
    //                                     }
    //                                 });
    //                             // if let Some(looping_ops) = looping_obs {
    //                             //     potential_pos.push(looping_ops);
    //                             //     for (idx, used) in n_obstacle.iter().enumerate().skip(1) {
    //                             //         used_obs.insert((used.clone(), idx % 4));
    //                             //     }
    //                             //     // break;
    //                             // }
    //                         }
    //                     }
    //                     /* keep going */
    //                     current_pos = new_current_pos;
    //                     current_dir = new_current_dir;
    //                 }
    //                 // None -> got out of bounds
    //             }
    //         }
    //     }

    //     potential_pos
    // }

    /// ```txt
    /// . . . A . . . . .  
    /// . . . . . . . . #  
    /// . . . . . . . . .  
    /// . . . . . . . . .  
    /// . . N . . . . v .  
    /// . . . . . . . B .  
    /// ```
    ///
    /// - We must compute obs `N`.
    /// - We know `B` (obstacle hit before `N`) and `A` (obstacle hit after `N`).
    /// - We also know that we encounter `B` with `dir` (then we encounter `N` with `dir.rotate_right()`)
    ///
    /// ## Return
    ///
    /// - None if couldn't loop
    // fn compute_nth(
    //     before_obs: (BoundedPos, Dir),
    //     after_obs: (BoundedPos, Dir),
    // ) -> Option<(BoundedPos, Dir)> {
    //     let to_N_dir = before_obs.1.rotate_right();
    //     let mut pos = (before_obs.0 + before_obs.1.opposite()).unwrap();

    //     while let Some(new_pos) = pos + to_N_dir {
    //         if before_obs.1.aligned(new_pos, after_obs.0) {
    //             return Some(((new_pos + to_N_dir)?, to_N_dir));
    //         }
    //         pos = new_pos;
    //     }
    //     None
    // }

    /// Returns `true` if loops.
    fn walk_detect_loop(
        &self,
        start_pos: BoundedPos,
        start_dir: Dir,
        // history: &HashSet<(BoundedPos, Dir)>,
    ) -> bool {
        // // clone precedent history for faster detection
        // dbg!(start_pos, start_dir);
        let mut pos_dir_history = HashSet::new();
        pos_dir_history.insert((start_pos, start_dir));

        let mut current_pos = start_pos;
        let mut current_dir = start_dir;

        while let Some((new_pos, new_dir)) = self.next_pos_dir(current_pos, current_dir) {
            if pos_dir_history.contains(&(new_pos, new_dir)) {
                // LOOP!
                return true;
            }
            pos_dir_history.insert((new_pos, new_dir));
            current_pos = new_pos;
            current_dir = new_dir;
        }
        // out of bounds
        false
    }

    // fn effective_loop_possibilities(&mut self) -> usize {
    //     let mut reachable_loops = 0;
    //     let mut potential_loop_obs = self.list_potential_loops();
    //     dbg!(potential_loop_obs.len());
    //     // dbg!(potential_loop_obs);
    //     let mut hash = DefaultHasher::new();
    //     potential_loop_obs.hash(&mut hash);
    //     dbg!(hash.finish());
    //     // let mut obs_set = Vec::new();

    //     // for i in 0..10 {
    //     //     dbg!("======", obs_set.len(), self.guard_pos, self.guard_dir);
    //     //     for (obs_pos, _obs_dir) in &potential_loop_obs {
    //     //         if obs_set.contains(&obs_pos) {
    //     //             continue;
    //     //         }
    //     //         *self.get_mut(*obs_pos) = MapTile::Obstacle;
    //     //         if self.walk_detect_loop(self.guard_pos, self.guard_dir) {
    //     //             // dbg!(&obs_pos);
    //     //             obs_set.push(obs_pos);
    //     //             // dbg!(obs_set.len());
    //     //         }
    //     //         *self.get_mut(*obs_pos) = MapTile::Empty;
    //     //     }
    //     //     // dbg!(obs_set.len());
    //     // }
    //     // return obs_set.len();

    //     let mut pos_history = HashSet::new();
    //     pos_history.insert(self.guard_pos);

    //     while let Some((new_pos, new_dir)) = self.next_guard_pos_dir() {
    //         // did move => no obstacle there && didn't need to get through there earlier
    //         if new_pos != self.guard_pos && !pos_history.contains(&new_pos) {
    //             assert!(self.get(new_pos) == MapTile::Empty);

    //             // check if potential_loop_obs is at new_pos for guard_dir
    //             if let Some(obs_idx) =
    //                 potential_loop_obs
    //                     .iter()
    //                     .enumerate()
    //                     .find_map(|(idx, &(obs_pos, obs_dir))| {
    //                         if obs_pos == new_pos && obs_dir == self.guard_dir {
    //                             Some(idx)
    //                         } else {
    //                             None
    //                         }
    //                     })
    //             {
    //                 // found a loop;
    //                 potential_loop_obs.remove(obs_idx);
    //                 reachable_loops += 1;
    //             }
    //         }

    //         self.guard_pos = new_pos;
    //         self.guard_dir = new_dir;
    //         pos_history.insert(new_pos);
    //     }
    //     // dbg!(potential_loop_obs);
    //     reachable_loops
    // }

    fn count_possible_loop_obs(&mut self) -> usize {
        let path = self.path_until_out();

        let mut validated_loop_pos = BTreeSet::new();
        let mut eliminated_loop_pos = BTreeSet::new();
        for (pos, dir) in path {
            if pos == self.guard_pos && dir == self.guard_dir {
                continue; // skip pos 0
            }
            if eliminated_loop_pos.contains(&pos) {
                continue; // skip
            }
            let before_pos = (pos + dir.opposite()).unwrap();
            *self.get_mut(pos) = MapTile::Obstacle;
            if self.walk_detect_loop(before_pos, dir) {
                validated_loop_pos.insert(pos);
            } else {
                eliminated_loop_pos.insert(pos);
            }
            *self.get_mut(pos) = MapTile::Empty;
        }
        validated_loop_pos.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let puzzle = Puzzle::from(input);
    Some(puzzle.progress_until_out())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::from(input);
    Some(puzzle.count_possible_loop_obs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn hash_set_obs() {
        let mut set = HashSet::new();
        let pos_dir1 = (
            BoundedPos {
                x: 0,
                y: 1,
                x_bound: 2,
                y_bound: 2,
            },
            Dir::Up,
        );

        set.insert(pos_dir1);
        assert!(set.contains(&pos_dir1));

        let pos_dir2 = (pos_dir1.0, pos_dir1.1.rotate_right());
        assert!(!set.contains(&pos_dir2));

        let mut pos3 = pos_dir1.0;
        pos3.x += 1;

        let pos_dir3 = (pos3, pos_dir1.1);
        assert!(!set.contains(&pos_dir3));
    }
}

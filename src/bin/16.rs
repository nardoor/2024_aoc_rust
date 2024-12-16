use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    hash::Hash,
};

use advent_of_code::{Dir, Pos};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ReindeerState {
    reindeer: Pos,
    reindeer_dir: Dir,
}

impl Hash for ReindeerState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.reindeer.x);
        state.write_usize(self.reindeer.y * 1_000);
        state.write_usize((self.reindeer_dir as usize) * 1_000_000);
    }
}

struct Maze {
    map: Vec<Vec<Tile>>,
    target: Pos,
    init_state: (ReindeerState, usize),
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut reindeer = None;
        let mut target = None;
        let map = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'S' => {
                            reindeer = Some(Pos { x, y });
                            Tile::Empty
                        }
                        'E' => {
                            target = Some(Pos { x, y });
                            Tile::Empty
                        }
                        _ => panic!("Unsupported char {c}"),
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            target: target.unwrap(),
            init_state: (
                ReindeerState {
                    reindeer: reindeer.unwrap(),
                    reindeer_dir: Dir::Right, /* East by default */
                },
                0,
            ),
        }
    }
}

impl ReindeerState {
    fn copy_go_forward(&self) -> Self {
        let mut copy = self.clone();
        copy.reindeer = copy.reindeer_dir.apply(&copy.reindeer).unwrap();
        copy
    }

    fn copy_rotate_right(&self) -> Self {
        let mut copy = self.clone();
        copy.reindeer_dir = copy.reindeer_dir.rotate_right();
        copy
    }
    fn copy_rotate_left(&self) -> Self {
        let mut copy = self.clone();
        copy.reindeer_dir = copy.reindeer_dir.rotate_left();
        copy
    }

    fn copy_rotate_to(&self, new_dir: Dir) -> Self {
        assert!(new_dir != self.reindeer_dir);

        if new_dir == self.reindeer_dir.rotate_left() {
            self.copy_rotate_left()
        } else if new_dir == self.reindeer_dir.rotate_right() {
            self.copy_rotate_right()
        } else {
            panic!("shouldn't happen");
        }
    }
}

impl Maze {
    fn get(&self, pos: Pos) -> Tile {
        self.map[pos.y][pos.x]
    }
    fn can_go_forward(&self, state: &ReindeerState) -> bool {
        let new_pos = state.reindeer_dir.apply(&state.reindeer).unwrap();
        self.get(new_pos) == Tile::Empty
    }

    fn should_rotate(&self, pos: Pos, dir: Dir, out_dirs: &mut [Dir; 2]) -> usize {
        let mut dir_idx = 0;
        for other_dir in Dir::all() {
            if dir == other_dir || dir.opposite() == other_dir
            /* skip turn around */
            {
                continue;
            }
            let n_pos = other_dir.apply(&pos).unwrap();
            if self.get(n_pos) == Tile::Empty {
                out_dirs[dir_idx] = other_dir;
                dir_idx += 1;
            }
        }
        dir_idx
    }

    fn print_with_path(&self, path: &Vec<Pos>) {
        let mut out = String::new();
        for (y, line) in self.map.iter().enumerate() {
            for (x, t) in line.into_iter().enumerate() {
                let pos = Pos { x, y };
                if path.contains(&pos) {
                    assert!(self.get(pos) == Tile::Empty);
                    out += "O"
                } else {
                    match t {
                        Tile::Empty => out += ".",
                        Tile::Wall => out += "#",
                    }
                }
            }
            out += "\n";
        }
        println!("{}", out);
    }

    fn shortest_score(&self) -> (usize, usize) {
        let (state, score) = self.init_state;
        let mut history = HashMap::new();
        let mut pile = VecDeque::new();
        let mut best_score = usize::MAX;
        let mut best_paths = Vec::new();

        history.insert(state, score);
        let positions = vec![state.reindeer];
        pile.push_front((state, score, positions));

        while let Some((curr_state, curr_score, positions)) = pile.pop_back() {
            if curr_state.reindeer == self.target {
                /* don't need to explore further */
                if curr_score < best_score {
                    best_score = curr_score;
                    best_paths.clear();
                    best_paths.push(positions.clone());
                }
                if curr_score == best_score {
                    best_paths.push(positions.clone());
                }
                continue;
            }
            if self.can_go_forward(&curr_state) {
                let new_state = curr_state.copy_go_forward();
                let new_score = curr_score + 1;
                let mut new_positions = positions.clone();
                new_positions.push(new_state.reindeer);
                if let Some(&old_score) = history.get(&new_state) {
                    if old_score >= new_score {
                        history.insert(new_state, new_score);
                        pile.push_front((new_state, new_score, new_positions));
                    }
                } else {
                    // insert (not preset)
                    history.insert(new_state, new_score);
                    pile.push_front((new_state, new_score, new_positions));
                }
            }

            let mut out_dirs = [Dir::Up, Dir::Up];
            let n_rotations =
                self.should_rotate(curr_state.reindeer, curr_state.reindeer_dir, &mut out_dirs);

            let new_score = curr_score + 1_000;
            for &new_dir in &out_dirs[..n_rotations] {
                let new_state = curr_state.copy_rotate_to(new_dir);
                if let Some(&old_score) = history.get(&new_state) {
                    if old_score >= new_score {
                        history.insert(new_state, new_score);
                        pile.push_front((new_state, new_score, positions.clone()));
                    }
                } else {
                    history.insert(new_state, new_score);
                    pile.push_front((new_state, new_score, positions.clone()));
                }
            }
        }
        let mut pos_set = BTreeSet::new();
        for path in best_paths {
            // self.print_with_path(&path);
            for pos in path {
                pos_set.insert(pos);
            }
        }
        (best_score, pos_set.len())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::from(input);
    Some(maze.shortest_score().0)
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::from(input);
    Some(maze.shortest_score().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_0() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two_0() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 0,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(64));
    }
}

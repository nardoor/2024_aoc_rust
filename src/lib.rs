use std::cmp::Ordering;

pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }
    pub fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
    pub fn aligned(self, pos1: Pos, pos2: Pos) -> bool {
        match self {
            Self::Up => pos1.x == pos2.x,
            Self::Right => pos1.y == pos2.y,
            Self::Down => pos1.x == pos2.x,
            Self::Left => pos1.y == pos2.y,
        }
    }
    pub fn apply(&self, pos: &Pos) -> Option<Pos> {
        let new_pos = match self {
            Dir::Up => Pos {
                x: pos.x.checked_add_signed(0)?,
                y: pos.y.checked_add_signed(-1)?,
            },
            Dir::Right => Pos {
                x: pos.x.checked_add_signed(1)?,
                y: pos.y.checked_add_signed(0)?,
            },
            Dir::Down => Pos {
                x: pos.x.checked_add_signed(0)?,
                y: pos.y.checked_add_signed(1)?,
            },
            Dir::Left => Pos {
                x: pos.x.checked_add_signed(-1)?,
                y: pos.y.checked_add_signed(0)?,
            },
        };

        Some(new_pos)
    }
    pub fn apply_bounded(&self, pos: &Pos, bound: &Bound) -> Option<Pos> {
        let new_pos = self.apply(pos)?;
        if new_pos.x >= bound.x_bound {
            return None;
        }
        if new_pos.y >= bound.y_bound {
            return None;
        }
        return Some(new_pos);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
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

#[derive(Debug)]
pub struct Bound {
    // first x value to be invalid
    pub x_bound: usize,
    // first y value to be invalid
    pub y_bound: usize,
}

impl<T> From<&Vec<Vec<T>>> for Bound {
    fn from(value: &Vec<Vec<T>>) -> Self {
        Self {
            x_bound: value[0].len(),
            y_bound: value.len(),
        }
    }
}

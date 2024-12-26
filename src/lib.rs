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
    pub fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
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

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

impl FromChar for Dir {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            other => panic!("unexpected '{other}'"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DirVec {
    pub dx: isize,
    pub dy: isize,
}

impl DirVec {
    pub fn new(a: Pos, b: Pos) -> Self {
        let res = Self {
            dx: b.x as isize - a.x as isize,
            dy: b.y as isize - a.y as isize,
        };

        // debug
        assert_eq!(res.apply(a).unwrap(), b);
        res
    }

    pub fn opposite(&self) -> Self {
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

    pub fn apply(&self, pos: Pos) -> Option<Pos> {
        Some(Pos {
            x: pos.x.checked_add_signed(self.dx)?,
            y: pos.y.checked_add_signed(self.dy)?,
        })
    }

    pub fn multiply(&mut self, f: isize) {
        self.dx *= f;
        self.dy *= f;
    }

    pub fn apply_wrap_bounded(&self, pos: &Pos, bound: &Bound) -> Pos {
        // dbg!(pos.x, self.dx, bound.x_bound);
        // dbg!((pos.x as isize + self.dx) % bound.x_bound as isize);
        // dbg!((pos.x as isize + self.dx).rem_euclid(bound.x_bound as isize));
        // dbg!(((pos.x as isize + self.dx) % bound.x_bound as isize) as usize);
        let x = ((pos.x as isize + self.dx).rem_euclid(bound.x_bound as isize)) as usize;
        let y = ((pos.y as isize + self.dy).rem_euclid(bound.y_bound as isize)) as usize;
        Pos { x, y }
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

impl From<&str> for Pos {
    /* assume X,Y */
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(",").unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Bound {
    // first x value to be invalid
    pub x_bound: usize,
    // first y value to be invalid
    pub y_bound: usize,
}

impl Bound {
    pub fn check(&self, pos: Pos) -> Option<Pos> {
        if pos.x >= self.x_bound || pos.y >= self.y_bound {
            None
        } else {
            Some(pos)
        }
    }
}

impl<T> From<&Vec<Vec<T>>> for Bound {
    fn from(value: &Vec<Vec<T>>) -> Self {
        Self {
            x_bound: value[0].len(),
            y_bound: value.len(),
        }
    }
}

use std::{
    cmp::Ordering,
    fmt::{Display, Write},
};

use advent_of_code::{Bound, DirVec, Pos};

advent_of_code::solution!(14);

#[derive(Debug)]
enum Quarter {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
struct Map {
    bound: Bound,
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    vel: DirVec,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(" ").unwrap();
        let (_, p_vals) = p.split_once("=").unwrap();
        let (p_x, p_y) = p_vals.split_once(",").unwrap();
        let p_x: usize = p_x.parse().unwrap();
        let p_y: usize = p_y.parse().unwrap();

        let (_, v_vals) = v.split_once("=").unwrap();
        let (v_x, v_y) = v_vals.split_once(",").unwrap();
        let v_x = v_x.parse().unwrap();
        let v_y = v_y.parse().unwrap();

        Self {
            pos: Pos { x: p_x, y: p_y },
            vel: DirVec { dx: v_x, dy: v_y },
        }
    }
}

struct Picture<'a>(&'a Map, &'a Vec<Robot>);

impl<'a> Display for Picture<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.0.bound.y_bound {
            for x in 0..self.0.bound.x_bound {
                let count = self
                    .1
                    .iter()
                    .filter(|&r| r.pos.x == x && r.pos.y == y)
                    .count();

                if count == 0 {
                    f.write_char('.')?;
                } else if count > 9 {
                    f.write_char('N')?;
                } else {
                    f.write_fmt(format_args!("{}", count))?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

trait ProximityScore {
    fn proximity_score(&self) -> usize;
}

impl ProximityScore for Vec<Robot> {
    fn proximity_score(&self) -> usize {
        let mut score = 0;
        for (i1, r1) in self.iter().enumerate() {
            for r2 in self[i1 + 1..].iter() {
                if r1.pos.x.abs_diff(r2.pos.x) <= 1 && r1.pos.y.abs_diff(r2.pos.y) <= 1 {
                    score += 1;
                }
            }
        }
        score
    }
}

impl Robot {
    fn move_n_times(&mut self, map: &Map, n: usize) {
        let mut n_vel = self.vel.clone();
        n_vel.multiply(n as isize);
        self.pos = n_vel.apply_wrap_bounded(&self.pos, &map.bound);
    }

    fn quarter(&self, map: &Map) -> Option<Quarter> {
        let hx = map.bound.x_bound.div_euclid(2);
        let hy = map.bound.y_bound.div_euclid(2);
        match (self.pos.x.cmp(&hx), self.pos.y.cmp(&hy)) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => None,
            (Ordering::Greater, Ordering::Greater) => Some(Quarter::BottomRight),
            (Ordering::Less, Ordering::Greater) => Some(Quarter::BottomLeft),
            (Ordering::Greater, Ordering::Less) => Some(Quarter::TopRight),
            (Ordering::Less, Ordering::Less) => Some(Quarter::TopLeft),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(debug_assertions)]
    let map = Map {
        bound: Bound {
            x_bound: 11,
            y_bound: 7,
        },
    };

    #[cfg(not(debug_assertions))]
    let map = Map {
        bound: Bound {
            x_bound: 101,
            y_bound: 103,
        },
    };

    let mut robots: Vec<Robot> = input.lines().map(Robot::from).collect();

    robots.iter_mut().for_each(|r| r.move_n_times(&map, 100));
    let mut top_right_f = 0;
    let mut top_left_f = 0;
    let mut bottom_right_f = 0;
    let mut bottom_left_f = 0;
    robots.iter().for_each(|r| match r.quarter(&map) {
        None => (),
        Some(q) => match q {
            Quarter::BottomLeft => bottom_left_f += 1,
            Quarter::BottomRight => bottom_right_f += 1,
            Quarter::TopLeft => top_left_f += 1,
            Quarter::TopRight => top_right_f += 1,
        },
    });
    Some(top_left_f * top_right_f * bottom_left_f * bottom_right_f)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map {
        bound: Bound {
            x_bound: 101,
            y_bound: 103,
        },
    };
    let mut robots: Vec<Robot> = input.lines().map(Robot::from).collect();
    let mut secs = 0;
    let mut max_proximity_score = 0;
    loop {
        let proximity_score = robots.proximity_score();
        if proximity_score > max_proximity_score {
            max_proximity_score = proximity_score;
            // let picture = Picture(&map, &robots);
            // println!("secs: {secs}; proximity: {max_proximity_score}:\n{picture}");
        }
        // known solution
        if max_proximity_score >= 925 {
            let picture = Picture(&map, &robots);
            println!("secs: {secs}; proximity: {max_proximity_score}:\n{picture}");
            break;
        }
        secs += 1;
        robots.iter_mut().for_each(|r| r.move_n_times(&map, 1));
    }
    Some(secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_movement() {
        let map = Map {
            bound: Bound {
                x_bound: 11,
                y_bound: 7,
            },
        };
        let mut robot = Robot::from("p=2,4 v=2,-3");
        assert_eq!(robot.pos.x, 2);
        assert_eq!(robot.pos.y, 4);

        robot.move_n_times(&map, 1);
        assert_eq!(robot.pos.x, 4);
        assert_eq!(robot.pos.y, 1);

        robot.move_n_times(&map, 1);
        assert_eq!(robot.pos.x, 6);
        assert_eq!(robot.pos.y, 5);

        robot.move_n_times(&map, 1);
        assert_eq!(robot.pos.x, 8);
        assert_eq!(robot.pos.y, 2);

        robot.move_n_times(&map, 1);
        assert_eq!(robot.pos.x, 10);
        assert_eq!(robot.pos.y, 6);

        robot.move_n_times(&map, 1);
        assert_eq!(robot.pos.x, 1);
        assert_eq!(robot.pos.y, 3);

        let mut robot = Robot::from("p=2,4 v=2,-3");
        robot.move_n_times(&map, 5);
        assert_eq!(robot.pos.x, 1);
        assert_eq!(robot.pos.y, 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}

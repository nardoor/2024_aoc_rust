use core::panic;
use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    hash::Hash,
};

use advent_of_code::{Dir, DirVec, Pos};

advent_of_code::solution!(21);

struct CodePad(&'static Pad<4, 3>);

impl CodePad {
    const CODE_PAD: Pad<4, 3> = Pad([
        [Some('7'), Some('8'), Some('9')],
        [Some('4'), Some('5'), Some('6')],
        [Some('1'), Some('2'), Some('3')],
        [None, Some('0'), Some('A')],
    ]);

    fn new() -> Self {
        CodePad(&Self::CODE_PAD)
    }

    fn moves_for(&self, target_out: &String) -> PadInputList {
        let pad = self.get_pad();
        let mut cur_pos = pad.get_pos_for('A').unwrap();
        let mut pad_input_l = PadInputList { inputs: Vec::new() };
        let to_avoid_pos = self.get_empty_pos();

        for target_key in target_out.chars() {
            let target_key_pos = pad.get_pos_for(target_key).unwrap();
            // dbg!(target_key, target_key_pos);
            let dir_vec = DirVec::new(cur_pos, target_key_pos);
            let next_pos = dir_vec.apply(cur_pos).unwrap();

            let mut priority = Priority::None;
            if dir_vec.dx != 0 && dir_vec.dy != 0 {
                if dir_vec.x_dir().unwrap().aligned(cur_pos, to_avoid_pos)
                    && cur_pos.x.checked_add_signed(dir_vec.dx).unwrap() == to_avoid_pos.x
                {
                    priority = Priority::Y;
                } else if dir_vec.y_dir().unwrap().aligned(cur_pos, to_avoid_pos)
                    && cur_pos.y.checked_add_signed(dir_vec.dy).unwrap() == to_avoid_pos.y
                {
                    priority = Priority::X
                } else {
                    priority = RobotCommandPad::apply_priority_rules(
                        dir_vec.x_dir().unwrap(),
                        dir_vec.y_dir().unwrap(),
                    )
                }
            }
            pad_input_l.inputs.push(PadInput { dir_vec, priority });
            cur_pos = next_pos;
        }

        pad_input_l
    }
}

struct RobotCommandPad(&'static Pad<2, 3>);

impl RobotCommandPad {
    const ROBOT_PAD: Pad<2, 3> = Pad([
        [None, Some('^'), Some('A')],
        [Some('<'), Some('v'), Some('>')],
    ]);

    fn new() -> Self {
        RobotCommandPad(&Self::ROBOT_PAD)
    }

    fn pad_input_one_key(
        key: char,
        n: usize,
        cur_pos: Pos,
        to_avoid_pos: Pos,
        pad_input_list: &mut PadInputList,
        pad: &Pad<2, 3>,
    ) -> Pos {
        assert!(n >= 1);
        let key_pos = pad.get_pos_for(key).unwrap();
        let dir_vec = DirVec::new(cur_pos, key_pos);
        let mut priority = Priority::None;
        if dir_vec.dx != 0 && dir_vec.dy != 0 {
            if dir_vec.x_dir().unwrap().aligned(cur_pos, to_avoid_pos)
                && cur_pos.x.checked_add_signed(dir_vec.dx).unwrap() == to_avoid_pos.x
            {
                // println!("Careful aligned X");
                priority = Priority::Y;
            } else if dir_vec.y_dir().unwrap().aligned(cur_pos, to_avoid_pos)
                && cur_pos.y.checked_add_signed(dir_vec.dy).unwrap() == to_avoid_pos.y
            {
                // println!("Careful aligned Y");
                priority = Priority::X
            } else {
                priority =
                    Self::apply_priority_rules(dir_vec.x_dir().unwrap(), dir_vec.y_dir().unwrap())
            }
        }
        pad_input_list.inputs.push(PadInput { dir_vec, priority });

        for _ in 1..n {
            pad_input_list.inputs.push(PadInput {
                dir_vec: DirVec { dx: 0, dy: 0 },
                priority: Priority::None,
            });
        }

        dir_vec.apply(cur_pos).unwrap()
    }

    fn apply_priority_rules(x_dir: Dir, y_dir: Dir) -> Priority {
        match (x_dir, y_dir) {
            (Dir::Left, _) => Priority::X,
            (Dir::Right, _) => Priority::Y,
            _ => panic!(),
        }
    }

    fn expand_pad_input(
        &self,
        target_pad_input: &PadInput,
        memory: &mut HashMap<PadInput, (PadInputList, Pos)>,
    ) -> PadInputList {
        let pad = self.get_pad();
        let mut cur_pos = pad.get_pos_for('A').unwrap();
        let to_avoid_pos = self.get_empty_pos();

        let mut curr_pad_input_list = PadInputList { inputs: Vec::new() };
        if memory.contains_key(target_pad_input) && false {
            let (cache_inputs, n_pos) = memory.get(target_pad_input).unwrap().clone();
            curr_pad_input_list = cache_inputs;
            cur_pos = n_pos;
        } else {
            assert!(cur_pos == pad.get_pos_for('A').unwrap());
            let key_x = target_pad_input.dir_vec.x_dir().map(|d| d.to_char());
            let key_y = target_pad_input.dir_vec.y_dir().map(|d| d.to_char());
            if key_x.is_none() && key_y.is_none() { /* skip */
            } else if key_y.is_none() && key_x.is_some() {
                let n_pos = Self::pad_input_one_key(
                    key_x.unwrap(),
                    target_pad_input.dir_vec.dx.abs() as usize,
                    cur_pos,
                    to_avoid_pos,
                    &mut curr_pad_input_list,
                    pad,
                );
                cur_pos = n_pos
            } else if key_x.is_none() && key_y.is_some() {
                let n_pos = Self::pad_input_one_key(
                    key_y.unwrap(),
                    target_pad_input.dir_vec.dy.abs() as usize,
                    cur_pos,
                    to_avoid_pos,
                    &mut curr_pad_input_list,
                    pad,
                );
                cur_pos = n_pos;
            } else {
                match target_pad_input.priority {
                    Priority::X => {
                        let key_x = key_x.unwrap();
                        let n_pos = Self::pad_input_one_key(
                            key_x,
                            target_pad_input.dir_vec.dx.abs() as usize,
                            cur_pos,
                            to_avoid_pos,
                            &mut curr_pad_input_list,
                            pad,
                        );

                        let key_y = key_y.unwrap();
                        let n_pos = Self::pad_input_one_key(
                            key_y,
                            target_pad_input.dir_vec.dy.abs() as usize,
                            n_pos,
                            to_avoid_pos,
                            &mut curr_pad_input_list,
                            pad,
                        );
                        cur_pos = n_pos;
                    }
                    Priority::Y => {
                        let key_y = key_y.unwrap();
                        let n_pos = Self::pad_input_one_key(
                            key_y,
                            target_pad_input.dir_vec.dy.abs() as usize,
                            cur_pos,
                            to_avoid_pos,
                            &mut curr_pad_input_list,
                            pad,
                        );

                        let key_x = key_x.unwrap();
                        let n_pos = Self::pad_input_one_key(
                            key_x,
                            target_pad_input.dir_vec.dx.abs() as usize,
                            n_pos,
                            to_avoid_pos,
                            &mut curr_pad_input_list,
                            pad,
                        );
                        cur_pos = n_pos;
                    }
                    Priority::None => {
                        /* gotta try both ways... */
                        let key_x = key_x.unwrap();
                        let key_y = key_y.unwrap();

                        match Self::apply_priority_rules(
                            target_pad_input.dir_vec.x_dir().unwrap(),
                            target_pad_input.dir_vec.y_dir().unwrap(),
                        ) {
                            Priority::X => {
                                let mut n_pos = Self::pad_input_one_key(
                                    key_x,
                                    target_pad_input.dir_vec.dx.abs() as usize,
                                    cur_pos,
                                    to_avoid_pos,
                                    &mut curr_pad_input_list,
                                    pad,
                                );
                                n_pos = Self::pad_input_one_key(
                                    key_y,
                                    target_pad_input.dir_vec.dy.abs() as usize,
                                    n_pos,
                                    to_avoid_pos,
                                    &mut curr_pad_input_list,
                                    pad,
                                );
                                cur_pos = n_pos;
                            }
                            Priority::Y => {
                                let mut n_pos = Self::pad_input_one_key(
                                    key_y,
                                    target_pad_input.dir_vec.dy.abs() as usize,
                                    cur_pos,
                                    to_avoid_pos,
                                    &mut curr_pad_input_list,
                                    pad,
                                );
                                n_pos = Self::pad_input_one_key(
                                    key_x,
                                    target_pad_input.dir_vec.dx.abs() as usize,
                                    n_pos,
                                    to_avoid_pos,
                                    &mut curr_pad_input_list,
                                    pad,
                                );
                                cur_pos = n_pos;
                            }
                            Priority::None => panic!(),
                        }
                    }
                }
            }
            match memory.get(target_pad_input) {
                Some(e) => {
                    assert!(e.0 == curr_pad_input_list)
                }
                None => {
                    memory.insert(
                        target_pad_input.clone(),
                        (curr_pad_input_list.clone(), cur_pos),
                    );
                }
            }
        }

        /* go to A and press */
        let key = 'A';
        cur_pos =
            Self::pad_input_one_key(key, 1, cur_pos, to_avoid_pos, &mut curr_pad_input_list, pad);
        assert_eq!(cur_pos, pad.get_pos_for('A').unwrap());

        curr_pad_input_list
    }

    fn moves_for(
        &self,
        target_out: &PadInputList,
        memory: &mut HashMap<PadInput, (PadInputList, Pos)>,
    ) -> PadInputList {
        let mut pad_input_l = PadInputList { inputs: Vec::new() };

        for inner_pad_input in &target_out.inputs {
            let mut curr_pad_input_list = self.expand_pad_input(inner_pad_input, memory);
            pad_input_l.inputs.append(&mut curr_pad_input_list.inputs);
        }

        pad_input_l
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Priority {
    X,
    Y,
    None,
}
trait PadCtrl<const H: usize, const W: usize> {
    fn get_empty_pos(&self) -> Pos;
    fn get_pad(&self) -> &Pad<H, W>;
}

#[derive(PartialEq, Eq, Clone)]
struct PadInput {
    dir_vec: DirVec,
    priority: Priority,
}

impl Hash for PadInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(self.dir_vec.dx);
        state.write_isize(self.dir_vec.dy);
        state.write_usize(self.priority as usize * 1_000_000);
    }
}

impl PadInput {
    fn input_len(&self) -> usize {
        self.dir_vec.dx.abs() as usize + self.dir_vec.dy.abs() as usize + 1 /* press follows every move */
    }
}

#[derive(Clone, PartialEq, Eq)]
struct PadInputList {
    inputs: Vec<PadInput>,
}

impl PadInputList {
    fn total_input_len(&self) -> usize {
        self.inputs.iter().map(PadInput::input_len).sum()
    }
}

impl Debug for PadInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut priority = self.priority;
        if priority == Priority::None {
            priority = RobotCommandPad::apply_priority_rules(
                self.dir_vec.x_dir().unwrap_or(Dir::Left),
                self.dir_vec.y_dir().unwrap_or(Dir::Up),
            )
        }
        match priority {
            Priority::None => {
                if let Some(dir) = self.dir_vec.x_dir() {
                    for _ in 0..self.dir_vec.dx.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
                if let Some(dir) = self.dir_vec.y_dir() {
                    for _ in 0..self.dir_vec.dy.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
            }
            Priority::X => {
                if let Some(dir) = self.dir_vec.x_dir() {
                    for _ in 0..self.dir_vec.dx.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
                if let Some(dir) = self.dir_vec.y_dir() {
                    for _ in 0..self.dir_vec.dy.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
            }
            Priority::Y => {
                if let Some(dir) = self.dir_vec.y_dir() {
                    for _ in 0..self.dir_vec.dy.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
                if let Some(dir) = self.dir_vec.x_dir() {
                    for _ in 0..self.dir_vec.dx.abs() {
                        f.write_char(dir.to_char())?;
                    }
                }
            }
        }
        f.write_char('A')
    }
}
impl Debug for PadInputList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for inp in &self.inputs {
            inp.fmt(f)?;
        }
        Ok(())
        // f.write_char('\n')
    }
}

impl PadCtrl<2, 3> for RobotCommandPad {
    fn get_empty_pos(&self) -> Pos {
        Pos { x: 0, y: 0 }
    }
    fn get_pad(&self) -> &Pad<2, 3> {
        self.0
    }
}
impl PadCtrl<4, 3> for CodePad {
    fn get_empty_pos(&self) -> Pos {
        Pos { x: 0, y: 3 }
    }
    fn get_pad(&self) -> &Pad<4, 3> {
        self.0
    }
}

struct Pad<const H: usize, const W: usize>([[Option<char>; W]; H]);
impl<const H: usize, const W: usize> Pad<H, W> {
    fn get_pos_for(&self, key: char) -> Option<Pos> {
        for (y, l) in self.0.iter().enumerate() {
            for (x, &c) in l.iter().enumerate() {
                if let Some(c) = c {
                    if c == key {
                        return Some(Pos { x, y });
                    }
                }
            }
        }
        return None;
    }

    // fn get(&self, pos: &Pos) -> Option<char> {
    //     self.0.get(pos.y)?.get(pos.x)?.clone()
    // }
}

pub fn part_one(input: &str) -> Option<usize> {
    let codes: Vec<String> = input
        .split("\n")
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect();
    let mut complexity = 0;
    let code_pad = CodePad::new();
    let robot_pad1 = RobotCommandPad::new();
    let robot_pad2 = RobotCommandPad::new();
    let mut memory = HashMap::new();
    for code in codes {
        let num_code: usize = code[..(code.len() - 1)].parse().unwrap();
        // println!("{}", &code);
        let first_inp = &code_pad.moves_for(&code);
        // println!("[{}]: {:?}", first_inp.total_input_len(), &first_inp);
        let second_inp = robot_pad1.moves_for(&first_inp, &mut memory);
        // println!("[{}]: {:?}", second_inp.total_input_len(), second_inp);
        let third_inp = robot_pad2.moves_for(&second_inp, &mut memory);
        // println!("[{}]: {:?}", third_inp.total_input_len(), &third_inp);
        // dbg!(num_code);
        // dbg!(num_code * third_inp.total_input_len());
        complexity += num_code * third_inp.total_input_len();
    }
    Some(complexity)
}

pub fn part_two(input: &str) -> Option<usize> {
    let codes: Vec<String> = input
        .split("\n")
        .map(|s| s.to_owned())
        .filter(|s| !s.is_empty())
        .collect();
    let mut complexity = 0;
    let code_pad = CodePad::new();
    let robot_pad = RobotCommandPad::new();
    let mut memory = HashMap::new();
    for code in codes {
        let num_code: usize = code[..(code.len() - 1)].parse().unwrap();
        let inp = code_pad.moves_for(&code);
        let mut code_inputs: HashMap<PadInput, usize> = HashMap::new();
        for input in &inp.inputs {
            code_inputs
                .entry(input.clone())
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
        for _i in 0..=25 {
            // println!("Code: {code} - robot_i: {i}...");
            let mut n_inp = HashMap::new();
            for (input, count) in code_inputs {
                let expanded_input = robot_pad.expand_pad_input(&input, &mut memory);
                for exp_inp in expanded_input.inputs {
                    n_inp
                        .entry(exp_inp)
                        .and_modify(|n| *n += count)
                        .or_insert(count);
                }
            }
            code_inputs = n_inp;
        }
        // 90921859153294 too low
        complexity += num_code * code_inputs.iter().map(|(_, &n)| n).sum::<usize>();
    }
    Some(complexity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}

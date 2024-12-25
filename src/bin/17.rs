use std::{
    fmt::{Display, Write},
    usize,
};

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug)]
enum OperandType {
    Literal,
    Combo,
    Legacy,
}

#[derive(Debug)]
struct Operand(u8);

impl Operand {
    fn read(&self, regs: &Registers, r#type: OperandType) -> usize {
        match r#type {
            OperandType::Literal => self.0 as usize,
            OperandType::Combo => match self.0 {
                0..=3 => self.0 as usize,
                4 => regs.a,
                5 => regs.b,
                6 => regs.c,
                _ => panic!(),
            },
            OperandType::Legacy => 0,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Self {
        match opcode {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!(),
        }
    }

    fn operand_type(&self) -> OperandType {
        match self {
            Self::ADV => OperandType::Combo,
            Self::BXL => OperandType::Literal,
            Self::BST => OperandType::Combo,
            Self::JNZ => OperandType::Literal,
            Self::BXC => OperandType::Legacy,
            Self::OUT => OperandType::Combo,
            Self::BDV => OperandType::Combo,
            Self::CDV => OperandType::Combo,
        }
    }

    fn exec(&self, operand: Operand, regs: &mut Registers, pc: &mut usize) -> Option<u8> {
        let mut next_pc = *pc + 2;
        let mut out = None;
        match self {
            Self::ADV => {
                let num = regs.a;
                let den = operand.read(regs, self.operand_type());
                regs.a = num >> den;
            }
            Self::BXL => regs.b = regs.b ^ operand.read(regs, self.operand_type()),
            Self::BST => {
                regs.b = operand.read(regs, self.operand_type()) % 8;
            }
            Self::JNZ if regs.a != 0 => next_pc = operand.read(regs, self.operand_type()) as usize,
            Self::JNZ => {
                assert_eq!(regs.a, 0) /* nothing because regs.a == 0 */
            }
            Self::BXC => regs.b = regs.b ^ regs.c,
            Self::OUT => {
                let val = operand.read(regs, self.operand_type()) % 8;
                let val = val as u8;
                out = Some(val)
            }
            Self::BDV => {
                let num = regs.a;
                let den = operand.read(regs, self.operand_type());
                regs.b = num >> den;
            }
            Self::CDV => {
                let num = regs.a;
                let den = operand.read(regs, self.operand_type());
                regs.c = num >> den;
            }
        }
        *pc = next_pc;
        assert!(next_pc % 2 == 0);
        out
    }
}

#[derive(Clone)]
struct Program {
    regs: Registers,
    pc: usize,
    prog: Vec<u8>,
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        let (regs, prog) = value.split_once("\n\n").unwrap();

        let mut regs_line = regs.lines();
        let (_, reg_a) = regs_line.next().unwrap().split_once(": ").unwrap();
        let (_, reg_b) = regs_line.next().unwrap().split_once(": ").unwrap();
        let (_, reg_c) = regs_line.next().unwrap().split_once(": ").unwrap();

        let regs = Registers {
            a: reg_a.parse().unwrap(),
            b: reg_b.parse().unwrap(),
            c: reg_c.parse().unwrap(),
        };

        let (_, prog) = prog.split_once(": ").unwrap();
        let prog = prog
            .trim()
            .split(",")
            .map(|tb| tb.parse().unwrap())
            .collect();

        Self { regs, pc: 0, prog }
    }
}

impl Program {
    fn step(&mut self) -> Result<Option<u8>, ()> {
        if self.pc >= self.prog.len() {
            return Err(());
        }

        let instr = Instruction::from_opcode(self.prog[self.pc]);
        let operand = Operand(self.prog[self.pc + 1]);

        let out = instr.exec(operand, &mut self.regs, &mut self.pc);
        Ok(out)
    }

    fn run(&mut self) -> Vec<u8> {
        let mut prog_out = Vec::new();
        while let Ok(out) = self.step() {
            if let Some(out) = out {
                prog_out.push(out);
            }
        }
        prog_out
    }

    /// Registers { a: 25358015, b: 0, c: 0 }  
    /// Program:  
    /// - `BST - Combo (4)`     // B <- A % 8
    /// - `BXL - Literal (1)`   // B <- B ^ 1
    /// - `CDV - Combo (5)`     // C <- A >> B  
    /// - `ADV - Combo (3)`     // A <- A >> 3  
    /// - `BXC - Legacy (7)`    // B2 <- B ^ C         
    /// - `BXL - Literal (6)`   // B_out <- B2 ^ 6  
    /// - `OUT - Combo (5)`     // OUT <- B_out  
    /// - `JNZ - Literal (0)`   // PC <- 0  
    fn fix_corrupted_reg_a(&self) -> usize {
        let target_output = self.prog.clone();
        let mut best_a = usize::MAX;
        // do it in reverse because output N depends on N+1 (and not the opposite)
        let mut to_check = vec![(target_output.len() - 1, 0)];
        while let Some((idx, a)) = to_check.pop() {
            let a = a << 3; /* shift 3 (opposite operation of ADV - Combo(3)) */
            for i in 0..8 {
                let new_a = a + i;
                let mut prog = self.clone();
                prog.regs.a = new_a;
                let out = prog.run();
                if *out.first().unwrap() == target_output[idx] {
                    match idx.checked_sub(1) {
                        None => {
                            best_a = best_a.min(new_a);
                        }
                        Some(idx) => {
                            to_check.push((idx, new_a));
                        }
                    }
                }
            }
        }
        best_a
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.regs))?;
        f.write_char('\n')?;
        f.write_str("Program:\n")?;

        for i in (0..self.prog.len()).step_by(2) {
            let instr = Instruction::from_opcode(self.prog[i]);
            let operand = Operand(self.prog[i + 1]);

            f.write_fmt(format_args!(
                "{:?} - {:?} ({})\n",
                instr,
                instr.operand_type(),
                operand.0
            ))?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut program = Program::from(input);
    let out = program.run();
    Some(out.into_iter().map(|tb| tb.to_string()).join(","))
}

pub fn part_two(input: &str) -> Option<usize> {
    let program = Program::from(input);
    let out = program.fix_corrupted_reg_a();
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}

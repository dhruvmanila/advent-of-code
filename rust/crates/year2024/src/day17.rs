use std::collections::HashSet;
use std::fmt;
use std::slice::Iter;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone)]
struct Register {
    /// The value of register A.
    a: u64,
    /// The value of register B.
    b: u64,
    /// The value of register C.
    c: u64,
}

impl Register {
    /// Creates a new register with the given value for register A and 0 for registers B and C.
    const fn a(value: u64) -> Self {
        Self {
            a: value,
            b: 0,
            c: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instruction> {
        Ok(match s {
            "0" => Instruction::Adv,
            "1" => Instruction::Bxl,
            "2" => Instruction::Bst,
            "3" => Instruction::Jnz,
            "4" => Instruction::Bxc,
            "5" => Instruction::Out,
            "6" => Instruction::Bdv,
            "7" => Instruction::Cdv,
            _ => bail!("Invalid instruction: {s}"),
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Instruction::Adv => "adv",
            Instruction::Bxl => "bxl",
            Instruction::Bst => "bst",
            Instruction::Jnz => "jnz",
            Instruction::Bxc => "bxc",
            Instruction::Out => "out",
            Instruction::Bdv => "bdv",
            Instruction::Cdv => "cdv",
        };
        f.write_str(s)
    }
}

#[derive(Debug)]
struct Program(Vec<Instruction>);

impl Program {
    /// Returns the instruction and operand at the given index or [`None`] if out of bounds.
    ///
    /// # Panics
    ///
    /// Panics if the instruction at the given index does not have an operand.
    fn get(&self, index: usize) -> Option<(Instruction, u8)> {
        Some((
            self.0.get(index).copied()?,
            self.0
                .get(index + 1)
                .copied()
                .expect("instruction should have an operand") as u8,
        ))
    }

    /// Returns an iterator over the values of the instructions in the program.
    fn iter_values(&self) -> ProgramValuesIter<'_> {
        ProgramValuesIter {
            iter: self.0.iter(),
        }
    }
}

/// An iterator over the [`Program`] values.
struct ProgramValuesIter<'a> {
    iter: Iter<'a, Instruction>,
}

impl Iterator for ProgramValuesIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        Some(*self.iter.next()? as u8)
    }
}

impl DoubleEndedIterator for ProgramValuesIter<'_> {
    fn next_back(&mut self) -> Option<u8> {
        Some(*self.iter.next_back()? as u8)
    }
}

/// A [`Computer`] runner to run the [`Program`].
struct ProgramRunner<'a> {
    register: Register,
    program: &'a Program,
}

impl<'a> ProgramRunner<'a> {
    fn new(program: &'a Program, register: Register) -> ProgramRunner<'a> {
        ProgramRunner { register, program }
    }

    /// Consumes the runner, runs the program and returns the output.
    fn run(mut self) -> Output {
        let mut index = 0;
        let mut output = Vec::new();

        loop {
            let Some((instruction, operand)) = self.program.get(index) else {
                break;
            };
            match instruction {
                Instruction::Adv => self.adv_instruction(operand),
                Instruction::Bxl => self.bxl_instruction(operand),
                Instruction::Bst => self.bst_instruction(operand),
                Instruction::Jnz => {
                    if let Some(next_index) = self.jnz_instruction(operand) {
                        index = next_index;
                        continue;
                    }
                }
                Instruction::Bxc => self.bxc_instruction(),
                Instruction::Out => output.push(self.out_instruction(operand)),
                Instruction::Bdv => self.bdv_instruction(operand),
                Instruction::Cdv => self.cdv_instruction(operand),
            }
            index += 2;
        }

        Output(output)
    }

    /// Perform the `adv` instruction with the given operand.
    ///
    /// # Panics
    ///
    /// If the values in any of the register cannot be converted to a `u32` for power operation.
    fn adv_instruction(&mut self, operand: u8) {
        self.register.a /= 2u64.pow(
            u32::try_from(self.combo_operand_value(operand))
                .expect("should be able to convert u64 to u32 for power operation"),
        );
    }

    /// Perform the `bxl` instruction with the given operand.
    fn bxl_instruction(&mut self, operand: u8) {
        self.register.b ^= u64::from(operand);
    }

    /// Perform the `bst` instruction with the given operand.
    fn bst_instruction(&mut self, operand: u8) {
        self.register.b = self.combo_operand_value(operand) % 8;
    }

    /// Perform the `jnz` instruction with the given operand.
    ///
    /// Returns [`Some`] containing the value to move the instruction pointer to, [`None`] if
    /// there's no jump required.
    fn jnz_instruction(&mut self, operand: u8) -> Option<usize> {
        if self.register.a == 0 {
            None
        } else {
            Some(usize::from(operand))
        }
    }

    /// Perform the `bxc` instruction.
    fn bxc_instruction(&mut self) {
        self.register.b ^= self.register.c;
    }

    /// Perform the `out` instruction with the given operand, returning the output value.
    fn out_instruction(&mut self, operand: u8) -> u8 {
        // SAFETY: The modulo 8 operation only keeps the last 3 bits of the number which is always
        // going to be in 0..=7 range which can fit in a `u8`.
        u8::try_from(self.combo_operand_value(operand) % 8).unwrap()
    }

    /// Perform the `bdv` instruction with the given operand.
    ///
    /// # Panics
    ///
    /// If the values in any of the register cannot be converted to a `u32` for power operation.
    fn bdv_instruction(&mut self, operand: u8) {
        self.register.b = self.register.a
            / 2u64.pow(
                u32::try_from(self.combo_operand_value(operand))
                    .expect("should be able to convert u64 to u32 for power operation"),
            );
    }

    /// Perform the `cdv` instruction with the given operand.
    ///
    /// # Panics
    ///
    /// If the values in any of the register cannot be converted to a `u32` for power operation.
    fn cdv_instruction(&mut self, operand: u8) {
        self.register.c = self.register.a
            / 2u64.pow(
                u32::try_from(self.combo_operand_value(operand))
                    .expect("should be able to convert u64 to u32 for power operation"),
            );
    }

    /// Resolve the value of the given combo operand.
    ///
    /// # Panics
    ///
    /// If the operand value is 7 as it's reserved and should not occur in valid programs.
    /// If the operand value is > 7 which are not possible on a 3-bit computer.
    fn combo_operand_value(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => u64::from(operand),
            4 => self.register.a,
            5 => self.register.b,
            6 => self.register.c,
            7 => panic!("invalid program: received combo operand 7"),
            _ => unreachable!("operand should be in range 0..=7"),
        }
    }
}

/// A 3-bit computer.
#[derive(Debug)]
struct Computer {
    register: Register,
    program: Program,
}

impl Computer {
    /// Runs the program without modifying the values in the register and returns the output.
    fn run_program(&self) -> Output {
        ProgramRunner::new(&self.program, self.register.clone()).run()
    }

    /// Reverse engineer the program to find the value of register A that produces the output
    /// that matches the program instructions.
    fn reverse_engineer(&self) -> u64 {
        let mut values = HashSet::new();
        values.insert(0);
        for expected in self.program.iter_values().rev() {
            let mut next_values = HashSet::new();
            for value in &values {
                let value = value << 3;
                for candidate in value..value + 8 {
                    let output = ProgramRunner::new(&self.program, Register::a(candidate)).run();
                    if output.first() == Some(expected) {
                        next_values.insert(candidate);
                    }
                }
            }
            values = next_values;
        }
        values
            .iter()
            .min()
            .copied()
            .expect("there should be at least one number")
    }
}

impl FromStr for Computer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Computer> {
        let mut numbers = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|word| !word.is_empty());

        let register = Register {
            a: numbers
                .next()
                .ok_or_else(|| anyhow!("Missing register A"))?
                .parse()?,
            b: numbers
                .next()
                .ok_or_else(|| anyhow!("Missing register B"))?
                .parse()?,
            c: numbers
                .next()
                .ok_or_else(|| anyhow!("Missing register C"))?
                .parse()?,
        };

        Ok(Computer {
            register,
            program: Program(numbers.map(str::parse).collect::<Result<Vec<_>, _>>()?),
        })
    }
}

struct Output(Vec<u8>);

impl Output {
    fn first(&self) -> Option<u8> {
        self.0.first().copied()
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for &value in &self.0 {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, "{value}")?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let computer = Computer::from_str(input)?;

    println!("Part 1: {}", computer.run_program());
    println!("Part 2: {}", computer.reverse_engineer());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const SAMPLE_INPUT2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn run_program() {
        let computer = Computer::from_str(SAMPLE_INPUT1).unwrap();
        assert_eq!(computer.run_program().to_string(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn reverse_engineer() {
        let computer = Computer::from_str(SAMPLE_INPUT2).unwrap();
        assert_eq!(computer.reverse_engineer(), 117_440);
    }
}

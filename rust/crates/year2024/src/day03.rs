use std::slice::Iter;

use anyhow::Result;

#[derive(Debug, Default)]
struct Instructions(Vec<Instruction>);

impl Instructions {
    /// Sums the results of all multiplications.
    fn sum_multiplications(&self) -> u64 {
        self.0
            .iter()
            .filter_map(|instruction| match instruction {
                Instruction::Multiplication(a, b) => Some(a * b),
                _ => None,
            })
            .sum()
    }

    /// Sums the results of all enabled multiplications.
    fn sum_enabled_multiplications(&self) -> u64 {
        EnabledMultiplications::new(self.0.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let mut instructions = Vec::new();
        let mut parser = InstructionParser::new(value);
        while let Some(instruction) = parser.next() {
            instructions.push(instruction);
        }
        Self(instructions)
    }
}

struct EnabledMultiplications<'a> {
    instructions: Iter<'a, Instruction>,
    enabled: bool,
}

impl<'a> EnabledMultiplications<'a> {
    fn new(instructions: Iter<'a, Instruction>) -> Self {
        Self {
            instructions,
            enabled: true,
        }
    }
}

impl Iterator for EnabledMultiplications<'_> {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.instructions.next()? {
                Instruction::Do => self.enabled = true,
                Instruction::Dont => self.enabled = false,
                Instruction::Multiplication(a, b) if self.enabled => return Some((*a, *b)),
                Instruction::Multiplication(..) => continue,
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    /// Enables future multiplication instructions.
    Do,
    /// Disables future multiplication instructions.
    Dont,
    /// Multiplication instruction.
    Multiplication(u64, u64),
}

/// A parser for the instructions.
struct InstructionParser<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> InstructionParser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            bytes: source.as_bytes(),
            index: 0,
        }
    }

    /// Returns the next instruction or `None` if the end of the input is reached.
    fn next(&mut self) -> Option<Instruction> {
        loop {
            let instruction = match self.bump()? {
                b'm' => self.parse_multiplication_instruction(),
                b'd' => self.parse_do_dont_instruction(),
                _ => continue,
            };
            if instruction.is_some() {
                return instruction;
            }
        }
    }

    /// Parses a multiplication instruction.
    fn parse_multiplication_instruction(&mut self) -> Option<Instruction> {
        if !self.eat_bytes(b"ul(") {
            return None;
        }
        let left = self.parse_number()?;
        if !self.eat_byte(b',') {
            return None;
        }
        let right = self.parse_number()?;
        if self.eat_byte(b')') {
            Some(Instruction::Multiplication(left, right))
        } else {
            None
        }
    }

    /// Parses a "do" or "don't" instruction.
    fn parse_do_dont_instruction(&mut self) -> Option<Instruction> {
        if !self.eat_byte(b'o') {
            return None;
        }
        if self.eat_bytes(b"()") {
            Some(Instruction::Do)
        } else if self.eat_bytes(b"n't()") {
            Some(Instruction::Dont)
        } else {
            None
        }
    }

    /// Parses a number that is at most 3 digits long. Returns `None` is the current byte is not a
    /// digit or end of input is reached.
    fn parse_number(&mut self) -> Option<u64> {
        let mut number = match self.peek_byte()? {
            digit @ b'0'..=b'9' => u64::from(digit - b'0'),
            _ => {
                // The first byte is not a digit, so we can't parse a number. Return early
                // to avoid returning a 0.
                return None;
            }
        };
        self.index += 1;
        for _ in 0..2 {
            match self.peek_byte() {
                Some(digit @ b'0'..=b'9') => {
                    number = number * 10 + u64::from(digit - b'0');
                    self.index += 1;
                }
                _ => break,
            }
        }
        Some(number)
    }

    /// Consumes the current byte and returns it.
    fn bump(&mut self) -> Option<u8> {
        let current = self.peek_byte();
        self.index += 1;
        current
    }

    /// Returns the current byte without consuming it.
    fn peek_byte(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    /// Consumes the current byte if it matches the given byte.
    fn eat_byte(&mut self, byte: u8) -> bool {
        if self.peek_byte() == Some(byte) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    /// Consumes the given bytes if they match the input.
    fn eat_bytes(&mut self, bytes: &[u8]) -> bool {
        if self
            .bytes
            .get(self.index..)
            .is_some_and(|remaining| remaining.starts_with(bytes))
        {
            self.index += bytes.len();
            true
        } else {
            false
        }
    }
}

pub fn solve(input: &str) -> Result<()> {
    let instructions = Instructions::from(input);

    println!("Part 1: {:?}", instructions.sum_multiplications());
    println!("Part 2: {:?}", instructions.sum_enabled_multiplications());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sample1() {
        let instructions = Instructions::from(SAMPLE_INPUT1);
        assert_eq!(instructions.sum_multiplications(), 161);
    }

    #[test]
    fn sample2() {
        let instructions = Instructions::from(SAMPLE_INPUT2);
        assert_eq!(instructions.sum_enabled_multiplications(), 48);
    }
}

use std::{fmt, str::FromStr};

use anyhow::{Error, Result, bail};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
}

impl From<u8> for Operator {
    fn from(value: u8) -> Operator {
        match value {
            b'+' => Operator::Add,
            b'*' => Operator::Multiply,
            _ => unreachable!("invalid operator byte: {}", value as char),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => f.write_str("+"),
            Operator::Multiply => f.write_str("*"),
        }
    }
}

struct Problem {
    numbers: Vec<Vec<u8>>,
    operator: Operator,
}

impl Problem {
    fn solve_human(&self) -> u64 {
        let numbers = self.numbers.iter().map(|bytes| {
            let mut number = 0;
            for &byte in bytes {
                match byte {
                    b' ' => {}
                    digit @ b'0'..=b'9' => number = number * 10 + u64::from(digit - b'0'),
                    _ => unreachable!("invalid byte in number: {}", byte as char),
                }
            }
            number
        });

        match self.operator {
            Operator::Add => numbers.sum(),
            Operator::Multiply => numbers.product(),
        }
    }

    fn solve_cephalopod(&self) -> u64 {
        let ndigits = self.numbers[0].len();
        let numbers = (0..ndigits).rev().map(|digit_index| {
            let mut number = 0;
            for bytes in &self.numbers {
                match bytes[digit_index] {
                    b' ' if number == 0 => {} // Leading spaces are ignored
                    b' ' => break,            // Trailing spaces end the number
                    digit @ b'0'..=b'9' => number = number * 10 + u64::from(digit - b'0'),
                    byte => unreachable!("invalid byte in number: {}", byte as char),
                }
            }
            number
        });

        match self.operator {
            Operator::Add => numbers.sum(),
            Operator::Multiply => numbers.product(),
        }
    }
}

impl fmt::Debug for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numbers_str: Vec<String> = self
            .numbers
            .iter()
            .map(|bytes| bytes.iter().map(|&b| b as char).collect())
            .collect();
        f.debug_struct("Problem")
            .field("numbers", &numbers_str)
            .field("operator", &self.operator)
            .finish()
    }
}

struct Worksheet {
    problems: Vec<Problem>,
}

impl Worksheet {
    fn grand_total_human(&self) -> u64 {
        self.problems.iter().map(Problem::solve_human).sum()
    }

    fn grand_total_cephalopod(&self) -> u64 {
        self.problems.iter().map(Problem::solve_cephalopod).sum()
    }
}

impl FromStr for Worksheet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Worksheet, Error> {
        // Read lines in reverse order to process the operator line first
        let mut lines = s.lines().rev();

        let mut ndigits = vec![];
        let mut problems = vec![];

        if let Some(operator_line) = lines.next() {
            let mut whitespace_count = 0u8;
            for byte in operator_line.bytes() {
                match byte {
                    b' ' => whitespace_count += 1,
                    b'+' | b'*' => {
                        problems.push(Problem {
                            numbers: vec![],
                            operator: Operator::from(byte),
                        });
                        if whitespace_count > 0 {
                            // Avoid counting the actual whitespace that separates the numbers
                            ndigits.push(whitespace_count);
                            whitespace_count = 0;
                        }
                    }
                    _ => bail!("invalid character in operator line: {}", byte as char),
                }
            }
            if whitespace_count > 0 {
                // Trailing whitespace after the last operator
                ndigits.push(whitespace_count + 1);
            }
        } else {
            bail!("received empty input");
        }

        if ndigits.len() != problems.len() {
            bail!(
                "number of operators ({}) does not match number of problems ({})",
                problems.len(),
                ndigits.len()
            );
        }

        for line in lines {
            let mut bytes = line.bytes();
            for (&ndigit, problem) in ndigits.iter().zip(problems.iter_mut()) {
                let mut number_bytes = Vec::with_capacity(usize::from(ndigit));
                for _ in 0..ndigit {
                    match bytes.next() {
                        Some(b' ') => number_bytes.push(b' '),
                        Some(digit @ b'0'..=b'9') => number_bytes.push(digit),
                        Some(other) => bail!("invalid character in number: {}", other as char),
                        None => bail!("unexpected end of line when reading number"),
                    }
                }
                // Skip the separating space
                match bytes.next() {
                    Some(b' ') | None => {} // None => end of line
                    Some(other) => bail!("invalid character after number: {}", other as char),
                }
                problem.numbers.insert(0, number_bytes);
            }
        }

        Ok(Worksheet { problems })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let worksheet = Worksheet::from_str(input)?;

    println!("Part 1: {}", worksheet.grand_total_human());
    println!("Part 2: {}", worksheet.grand_total_cephalopod());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn sample() {
        let worksheet = Worksheet::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(worksheet.grand_total_human(), 4_277_556);
        assert_eq!(worksheet.grand_total_cephalopod(), 3_263_827);
    }
}

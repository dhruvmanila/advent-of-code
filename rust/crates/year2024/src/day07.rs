use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operator {
    /// Add the two numbers e.g., 1 + 2 = 3.
    Add,
    /// Multiply the two numbers e.g., 2 * 3 = 6.
    Multiply,
    /// Concatenate the two numbers e.g., 12 || 34 = 1234.
    Concatenate,
}

impl Operator {
    /// Return a slice containing only the add and multiply operators.
    fn add_or_multiply() -> &'static [Operator] {
        &[Operator::Add, Operator::Multiply]
    }

    /// Return a slice containing all the operators.
    fn all() -> &'static [Operator] {
        &[Operator::Add, Operator::Multiply, Operator::Concatenate]
    }

    /// Apply the operator to the two numbers.
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => a * 10u64.pow(b.ilog10() + 1) + b,
        }
    }
}

#[derive(Debug)]
struct CalibrationEquation {
    test_value: u64,
    numbers: Vec<u32>,
}

impl CalibrationEquation {
    /// Check if the equation is solvable using the given operators.
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        fn inner(
            equation: &CalibrationEquation,
            operators: &[Operator],
            value: u64,
            index: usize,
        ) -> bool {
            let Some(number) = equation.numbers.get(index).copied() else {
                // We have reached the end of the numbers.
                return value == equation.test_value;
            };
            for operator in operators {
                let next_value = operator.apply(value, number.into());
                if next_value > equation.test_value {
                    // Short-circuit the search as the value is already larger than the test value.
                    continue;
                }
                if inner(equation, operators, next_value, index + 1) {
                    return true;
                }
            }
            false
        }

        let Some(value) = self.numbers.first().copied() else {
            // There are no numbers to work with.
            return false;
        };

        inner(self, operators, value.into(), 1)
    }
}

impl FromStr for CalibrationEquation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("Expected a colon in the input"))?;
        Ok(Self {
            test_value: left.parse::<u64>()?,
            numbers: right
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
struct CalibrationEquations(Vec<CalibrationEquation>);

impl CalibrationEquations {
    /// Calculate the sum of the test values for the equations that are solvable using the given
    /// operators.
    fn result(&self, operators: &[Operator]) -> u64 {
        self.0
            .iter()
            .filter_map(|equation| {
                equation
                    .is_solvable(operators)
                    .then_some(equation.test_value)
            })
            .sum()
    }
}

impl FromStr for CalibrationEquations {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let equations = CalibrationEquations::from_str(input)?;

    println!("Part 1: {}", equations.result(Operator::add_or_multiply()));
    println!("Part 2: {}", equations.result(Operator::all()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sample() {
        let equations = CalibrationEquations::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(equations.result(Operator::add_or_multiply()), 3749);
        assert_eq!(equations.result(Operator::all()), 11387);
    }
}

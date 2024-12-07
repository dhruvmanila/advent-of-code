use std::fmt;
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Add => f.write_str("+"),
            Operator::Multiply => f.write_str("*"),
            Operator::Concatenate => f.write_str("||"),
        }
    }
}

#[derive(Debug)]
struct SolvedEquation<'a> {
    /// The equation that was solved.
    equation: &'a CalibrationEquation,
    /// The solution to the equation which is a list of operators to apply to the numbers in the
    /// equation. The operators are applied in order from left to right between two consecutive
    /// numbers.
    solution: Vec<Operator>,
}

impl fmt::Display for SolvedEquation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:", self.equation.test_value)?;
        let mut numbers = self.equation.numbers.iter();
        if let Some(number) = numbers.next() {
            write!(f, " {number}")?;
        }
        for (number, operator) in numbers.zip(self.solution.iter()) {
            write!(f, " {operator} {number}")?;
        }
        Ok(())
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
        self.solve(operators).is_some()
    }

    /// Returns [`Some`] with the solved equation if it's solvable, otherwise [`None`].
    ///
    /// The return type implements [`fmt::Display`] to print the equation with the solution.
    fn to_solved_equation(&self, operators: &[Operator]) -> Option<SolvedEquation> {
        Some(SolvedEquation {
            equation: self,
            solution: self.solve(operators)?,
        })
    }

    /// Solve the equation using the given operators.
    fn solve(&self, operators: &[Operator]) -> Option<Vec<Operator>> {
        fn inner(
            equation: &CalibrationEquation,
            operators: &[Operator],
            value: u64,
            index: usize,
        ) -> Option<Vec<Operator>> {
            let Some(number) = equation.numbers.get(index).copied() else {
                // We have reached the end of the numbers.
                return if value == equation.test_value {
                    Some(Vec::with_capacity(equation.numbers.len().saturating_sub(1)))
                } else {
                    None
                };
            };
            for operator in operators {
                let next_value = operator.apply(value, number.into());
                if next_value > equation.test_value {
                    // Short-circuit the search as the value is already larger than the test value.
                    continue;
                }
                if let Some(mut solution) = inner(equation, operators, next_value, index + 1) {
                    solution.push(*operator);
                    return Some(solution);
                }
            }
            None
        }

        let Some(value) = self.numbers.first().copied() else {
            // There are no numbers to work with.
            return None;
        };

        inner(self, operators, value.into(), 1).map(|mut solution| {
            solution.reverse();
            solution
        })
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

    /// Display the solutions to the equations that are solvable using the given operators.
    #[allow(dead_code)]
    fn display_solutions(&self, operators: &[Operator]) {
        for equation in &self.0 {
            if let Some(solved_equation) = equation.to_solved_equation(operators) {
                println!("{solved_equation}");
            }
        }
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

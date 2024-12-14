use std::fmt;
use std::str::FromStr;

use anyhow::{anyhow, Result};

/// Returns the quotient and remainder of the division of `a` by `b`.
fn divmod(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

/// Represents a linear equation of the form `ax + by = rhs`.
struct LinearEquation {
    a: i64,
    b: i64,
    rhs: i64,
}

impl fmt::Debug for LinearEquation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x + {}y = {}", self.a, self.b, self.rhs)
    }
}

/// Represents a system of two linear equations.
#[derive(Debug)]
struct LinearEquationSystem([LinearEquation; 2]);

impl LinearEquationSystem {
    /// Solves the system of linear equations using [Cramer's rule].
    ///
    /// Returns `None` if the system has no solution or has infinitely many solutions.
    ///
    /// [Cramer's rule]: https://en.wikipedia.org/wiki/Cramer%27s_rule
    fn solve(&self, rhs_correction: i64) -> Option<(i64, i64)> {
        let [eq_x, eq_y] = &self.0;
        let (rhs_x, rhs_y) = (eq_x.rhs + rhs_correction, eq_y.rhs + rhs_correction);
        let determinant = eq_x.a * eq_y.b - eq_x.b * eq_y.a;
        // We're only interested in integer solutions, not any real solutions so use modulo to
        // check whether the final value is an integer or a real number.
        let (x, x_remainder) = divmod(eq_y.b * rhs_x - eq_x.b * rhs_y, determinant);
        let (y, y_remainder) = divmod(eq_x.a * rhs_y - eq_y.a * rhs_x, determinant);
        if x_remainder == 0 && y_remainder == 0 {
            Some((x, y))
        } else {
            None
        }
    }
}

/// A claw machine with two buttons, A and B, and a prize.
#[derive(Debug)]
struct ClawMachine(LinearEquationSystem);

impl ClawMachine {
    /// Returns the fewest number of tokens required to win the prize, if possible.
    fn fewest_tokens_to_win(&self, prize_correction: i64) -> Option<i64> {
        let (a, b) = self.0.solve(prize_correction)?;
        Some(3 * a + b)
    }
}

impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s
            .split(|ch: char| !ch.is_ascii_digit())
            .filter(|word| !word.is_empty())
            .map(str::parse::<i64>);

        let ax = numbers
            .next()
            .ok_or_else(|| anyhow!("missing X for button A"))??;
        let ay = numbers
            .next()
            .ok_or_else(|| anyhow!("missing Y for button A"))??;
        let bx = numbers
            .next()
            .ok_or_else(|| anyhow!("missing X for button B"))??;
        let by = numbers
            .next()
            .ok_or_else(|| anyhow!("missing Y for button B"))??;
        let px = numbers
            .next()
            .ok_or_else(|| anyhow!("missing X for prize"))??;
        let py = numbers
            .next()
            .ok_or_else(|| anyhow!("missing Y for prize"))??;

        let x = LinearEquation {
            a: ax,
            b: bx,
            rhs: px,
        };
        let y = LinearEquation {
            a: ay,
            b: by,
            rhs: py,
        };

        Ok(Self(LinearEquationSystem([x, y])))
    }
}

/// An arcade with multiple claw machines.
#[derive(Debug)]
struct Arcade(Vec<ClawMachine>);

impl Arcade {
    /// Returns the fewest number of tokens required to win prizes from as many claw machines as
    /// possible.
    fn fewest_tokens_to_win(&self, prize_correction: i64) -> i64 {
        self.0
            .iter()
            .filter_map(|machine| machine.fewest_tokens_to_win(prize_correction))
            .sum()
    }
}

impl FromStr for Arcade {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split("\n\n")
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let arcade = input.parse::<Arcade>()?;

    println!("Part 1: {:?}", arcade.fewest_tokens_to_win(0));
    println!(
        "Part 2: {:?}",
        arcade.fewest_tokens_to_win(10_000_000_000_000)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn sample() {
        let arcade: Arcade = SAMPLE_INPUT.parse().unwrap();
        assert_eq!(arcade.fewest_tokens_to_win(0), 480);
    }
}

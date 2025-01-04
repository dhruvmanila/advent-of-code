use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Either;

/// A stone that has a number engraved on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(u64);

impl Stone {
    /// Returns the outcome when someone looking at the stone blink once.
    fn blink(self) -> BlinkOutcome {
        if self.0 == 0 {
            BlinkOutcome::Single(Stone(1))
        } else {
            let ndigits = self.0.ilog10() + 1;
            if ndigits % 2 == 0 {
                let divisor = 10u64.pow(ndigits / 2);
                BlinkOutcome::Split(Stone(self.0 / divisor), Stone(self.0 % divisor))
            } else {
                BlinkOutcome::Single(Stone(self.0 * 2024))
            }
        }
    }
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Stone, ParseIntError> {
        Ok(Stone(s.parse()?))
    }
}

/// The outcome of when someone looking at the stone blink once.
#[derive(Debug)]
enum BlinkOutcome {
    /// A single stone.
    Single(Stone),
    /// Splits the stone into two stones.
    Split(Stone, Stone),
}

impl BlinkOutcome {
    /// Returns an iterator over the stones.
    fn iter(&self) -> impl Iterator<Item = Stone> + '_ {
        match self {
            BlinkOutcome::Single(stone) => Either::Left(std::iter::once(*stone)),
            BlinkOutcome::Split(left, right) => Either::Right([*left, *right].into_iter()),
        }
    }
}

/// A collection of stones.
#[derive(Debug)]
struct Stones(HashMap<Stone, usize>);

impl Stones {
    /// Makes all the stones blink once.
    fn blink(&mut self) {
        let mut stones = HashMap::with_capacity(self.0.len());
        for (stone, count) in &self.0 {
            for new_stone in stone.blink().iter() {
                stones
                    .entry(new_stone)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        self.0 = stones;
    }

    /// Makes all the stones blink `n` times.
    fn blinks(&mut self, n: usize) {
        for _ in 0..n {
            self.blink();
        }
    }

    /// Returns the total number of stones.
    fn total(&self) -> usize {
        self.0.values().sum()
    }
}

impl FromStr for Stones {
    type Err = Error;

    fn from_str(s: &str) -> Result<Stones> {
        let mut stones = HashMap::new();
        for stone in s.split_ascii_whitespace() {
            let stone = Stone::from_str(stone)?;
            *stones.entry(stone).or_insert(0) += 1;
        }
        Ok(Stones(stones))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let mut stones = Stones::from_str(input)?;

    stones.blinks(25);
    println!("Part 1: {}", stones.total());

    stones.blinks(50);
    println!("Part 2: {}", stones.total());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "0 1 10 99 999";
    const SAMPLE_INPUT2: &str = "125 17";

    #[test_case(SAMPLE_INPUT1, 1, 7)]
    #[test_case(SAMPLE_INPUT2, 6, 22)]
    #[test_case(SAMPLE_INPUT2, 25, 55312)]
    fn sample(input: &str, blinks: usize, expected: usize) {
        let mut stones = Stones::from_str(input).unwrap();
        stones.blinks(blinks);
        assert_eq!(stones.total(), expected);
    }
}

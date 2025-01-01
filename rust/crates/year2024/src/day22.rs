use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SecretNumber(u64);

impl SecretNumber {
    /// Compute the next secret number using the rules specified in the puzzle.
    fn next(mut self) -> SecretNumber {
        self.0 = (self.0 ^ (self.0 << 6)) & 0x00FF_FFFF;
        self.0 = (self.0 ^ (self.0 >> 5)) & 0x00FF_FFFF;
        SecretNumber((self.0 ^ (self.0 << 11)) & 0x00FF_FFFF)
    }

    /// Returns an iterator that generates the next `n` secret numbers.
    ///
    /// The iterator will start with the next secret number after `self`. In total, the iterator
    /// will generate `n` secret numbers.
    fn successors(self, n: usize) -> impl Iterator<Item = SecretNumber> {
        std::iter::successors(Some(self), |previous| Some(previous.next()))
            .skip(1)
            .take(n)
    }

    /// Returns the ones digit of the secret number.
    fn ones_digit(self) -> u8 {
        // SAFETY: The ones digit of a number is always less than 10.
        u8::try_from(self.0 % 10).unwrap()
    }

    /// Returns the ones digit of the secret number as a signed integer.
    fn ones_digit_signed(self) -> i32 {
        i32::from(self.ones_digit())
    }
}

#[derive(Debug)]
struct SecretNumbers(Vec<SecretNumber>);

impl SecretNumbers {
    fn solve(&self, n: usize) -> (u64, u64) {
        let mut sum = 0;
        let mut sequence_price = HashMap::new();

        for &secret_number in &self.0 {
            // Store the previous secret number as we don't have access to it while iterating over
            // the 4 successive secret numbers.
            let mut previous = secret_number;

            // Store the last secret number for part 1 of the problem. At the end of the loop, this
            // is going to be the `n`th secret number.
            let mut last = SecretNumber(0);

            // The change sequence that we've seen so far for the current secret number.
            let mut seen = HashSet::new();

            for (first, second, third, fourth) in secret_number.successors(n).tuple_windows() {
                let price = fourth.ones_digit();
                let change_sequence = [
                    first.ones_digit_signed() - previous.ones_digit_signed(),
                    second.ones_digit_signed() - first.ones_digit_signed(),
                    third.ones_digit_signed() - second.ones_digit_signed(),
                    i32::from(price) - third.ones_digit_signed(),
                ];
                if seen.insert(change_sequence) {
                    *sequence_price.entry(change_sequence).or_default() += u64::from(price);
                }
                previous = first;
                last = fourth;
            }

            sum += last.0;
        }

        (sum, *sequence_price.values().max().unwrap())
    }
}

impl FromStr for SecretNumbers {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<SecretNumbers, ParseIntError> {
        Ok(SecretNumbers(
            s.lines()
                .map(|line| Ok(SecretNumber(line.parse()?)))
                .collect::<Result<Vec<_>, ParseIntError>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let secret_numbers = SecretNumbers::from_str(input)?;
    let (sum, bananas) = secret_numbers.solve(2000);

    println!("Part 1: {sum}");
    println!("Part 2: {bananas}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = "\
1
10
100
2024
";

    const SAMPLE_INPUT2: &str = "\
1
2
3
2024
";

    #[test]
    fn sample1() {
        let numbers = SecretNumbers::from_str(SAMPLE_INPUT1).unwrap();
        let (sum, _) = numbers.solve(2000);
        assert_eq!(sum, 37_327_623);
    }

    #[test]
    fn sample2() {
        let numbers = SecretNumbers::from_str(SAMPLE_INPUT2).unwrap();
        let (_, bananas) = numbers.solve(2000);
        assert_eq!(bananas, 23);
    }

    #[test]
    fn next_secret_number() {
        let mut number = SecretNumber(123);
        for expected in [
            15_887_950, 16_495_136, 527_345, 704_524, 1_553_684, 12_683_156, 11_100_544,
            12_249_484, 7_753_432, 5_908_254,
        ] as [u64; 10]
        {
            number = number.next();
            assert_eq!(number.0, expected);
        }
    }

    #[test]
    fn debug() {
        let number = SecretNumber(123);
        let (sum, bananas) = SecretNumbers(vec![number]).solve(9);
        assert_eq!(sum, 7_753_432);
        assert_eq!(bananas, 6);
    }
}

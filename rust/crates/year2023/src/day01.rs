use std::iter::{Enumerate, FusedIterator};
use std::str::Bytes;

use anyhow::{anyhow, Result};

/// The recovery strategy to use when parsing the digits from the document.
#[derive(Debug, Copy, Clone)]
enum RecoveryStrategy {
    /// Consider only the digits in the line.
    Digits,
    /// Consider the digits and the words that represent them in the line.
    DigitsAndWords,
}

/// The words for the digits 1 through 9.
const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// An iterator over the digits in a line.
struct DigitIter<'a> {
    /// The line being iterated over.
    line: &'a str,
    /// An iterator over the bytes and their indices in the line.
    bytes: Enumerate<Bytes<'a>>,
    /// The recovery strategy to use when parsing the digits.
    strategy: RecoveryStrategy,
}

impl<'a> DigitIter<'a> {
    fn new(line: &'a str, strategy: RecoveryStrategy) -> DigitIter<'a> {
        DigitIter {
            line,
            bytes: line.bytes().enumerate(),
            strategy,
        }
    }
}

impl Iterator for DigitIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        loop {
            let (index, byte) = self.bytes.next()?;
            if byte.is_ascii_digit() {
                return Some(u32::from(byte - b'0'));
            }
            if matches!(self.strategy, RecoveryStrategy::Digits) {
                continue;
            }
            for (digit, word) in (1u32..).zip(DIGIT_WORDS.iter()) {
                // SAFETY: `index` is always valid because it comes from the `next` call above.
                if self.line[index..].starts_with(word) {
                    return Some(digit);
                }
            }
        }
    }
}

impl DoubleEndedIterator for DigitIter<'_> {
    fn next_back(&mut self) -> Option<u32> {
        loop {
            let (index, byte) = self.bytes.next_back()?;
            if byte.is_ascii_digit() {
                return Some(u32::from(byte - b'0'));
            }
            if matches!(self.strategy, RecoveryStrategy::Digits) {
                continue;
            }
            for (digit, word) in (1u32..).zip(DIGIT_WORDS.iter()) {
                // SAFETY: `index` is always valid because it comes from the `next_back` call
                // above.
                if self.line[..=index].ends_with(word) {
                    return Some(digit);
                }
            }
        }
    }
}

impl FusedIterator for DigitIter<'_> {}

#[derive(Debug)]
struct CalibrationDocument<'a>(Vec<&'a str>);

impl CalibrationDocument<'_> {
    /// Returns the sum of the values recovered from each line of the document using the given
    /// strategy.
    ///
    /// Returns an error if a line does not contain any digits.
    fn sum_recover(&self, strategy: RecoveryStrategy) -> Result<u32> {
        self.0
            .iter()
            .map(move |line| {
                let mut digits = DigitIter::new(line, strategy);
                let first = digits
                    .next()
                    .ok_or_else(|| anyhow!("line does not contain any digits: {line:?}"))?;
                let last = digits.next_back().unwrap_or(first);
                Ok(first * 10 + last)
            })
            .try_fold(0, |acc, result| result.map(|value| acc + value))
    }
}

impl<'a> From<&'a str> for CalibrationDocument<'a> {
    fn from(value: &'a str) -> CalibrationDocument<'a> {
        CalibrationDocument(value.lines().collect())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let document = CalibrationDocument::from(input);

    println!(
        "Part 1: {}",
        document.sum_recover(RecoveryStrategy::Digits)?
    );
    println!(
        "Part 2: {}",
        document.sum_recover(RecoveryStrategy::DigitsAndWords)?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SAMPLE_INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn recover_digits() {
        let document = CalibrationDocument::from(SAMPLE_INPUT1);
        assert_eq!(document.sum_recover(RecoveryStrategy::Digits).unwrap(), 142);
    }

    #[test]
    fn recover_digits_and_words() {
        let document = CalibrationDocument::from(SAMPLE_INPUT2);
        assert_eq!(
            document
                .sum_recover(RecoveryStrategy::DigitsAndWords)
                .unwrap(),
            281
        );
    }
}

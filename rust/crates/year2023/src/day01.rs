use anyhow::Result;

/// Returns an iterator over the calibration value for each line as computed
/// by the rules in mentioned part 1.
fn calibration_values1(input: &str) -> impl Iterator<Item = Result<u32>> + '_ {
    input.lines().enumerate().map(|(index, line)| {
        let mut digits = line.chars().filter_map(|ch| ch.to_digit(10));
        let first = digits
            .next()
            .ok_or_else(|| anyhow::anyhow!("Line {} has no digits: {}", index, line))?;
        let last = digits.next_back().unwrap_or(first);
        Ok(first * 10 + last)
    })
}

/// The words for the digits 1 through 9.
const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// An iterator over the digits in a line as computed by the rules in mentioned
/// in part 2.
struct DigitIter<'a> {
    /// The line being iterated over.
    line: &'a str,
    /// An iterator over the character and their indices in the line.
    chars: std::str::CharIndices<'a>,
}

impl<'a> DigitIter<'a> {
    fn new(line: &'a str) -> Self {
        Self {
            line,
            chars: line.char_indices(),
        }
    }
}

impl Iterator for DigitIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((index, ch)) = self.chars.next() else {
                break;
            };
            if let Some(digit) = ch.to_digit(10) {
                return Some(digit);
            }
            for (digit, word) in (1u32..).zip(DIGIT_WORDS.iter()) {
                // SAFETY: `index` is always valid because it comes from the
                // `next` call above.
                if self.line[index..].starts_with(word) {
                    return Some(digit + 1);
                }
            }
        }
        None
    }
}

/// Returns an iterator over the calibration value for each line as computed
/// by the rules in mentioned part 2.
fn calibration_values2(input: &str) -> impl Iterator<Item = Result<u32>> + '_ {
    input.lines().enumerate().map(|(index, line)| {
        let mut digits = DigitIter::new(line);
        let first = digits
            .next()
            .ok_or_else(|| anyhow::anyhow!("Line {} has no digits: {}", index, line))?;
        let last = digits.last().unwrap_or(first);
        Ok(first * 10 + last)
    })
}

pub fn solve(input: &str) -> Result<()> {
    println!(
        "Part 1: {:?}",
        calibration_values1(input).sum::<Result<u32>>()?
    );
    println!(
        "Part 2: {:?}",
        calibration_values2(input).sum::<Result<u32>>()?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT1: &str = r"\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const SAMPLE_INPUT2: &str = r"\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_sample_part_1() -> Result<()> {
        let values = calibration_values1(SAMPLE_INPUT1).collect::<Result<Vec<_>>>()?;

        assert_eq!(values, vec![12, 38, 15, 77]);
        assert_eq!(values.iter().sum::<u32>(), 142);

        Ok(())
    }

    #[test]
    fn test_sample_part_2() -> Result<()> {
        let values = calibration_values2(SAMPLE_INPUT2).collect::<Result<Vec<_>>>()?;

        assert_eq!(values, vec![29, 83, 13, 24, 42, 14, 76]);
        assert_eq!(values.iter().sum::<u32>(), 281);

        Ok(())
    }
}

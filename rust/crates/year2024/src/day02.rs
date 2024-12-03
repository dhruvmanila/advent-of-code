use std::cmp::Ordering;
use std::str::FromStr;

use aoc_lib::SkipNthExt;

use anyhow::{Error, Result};

#[derive(Debug, Default)]
struct UnusualData(Vec<Report>);

impl UnusualData {
    fn safe_report_count(&self) -> usize {
        self.0.iter().filter(|report| report.is_safe()).count()
    }

    fn safe_report_count_with_problem_dampener(&self) -> usize {
        self.0
            .iter()
            .filter(|report| report.is_safe_with_problem_dampener())
            .count()
    }
}

impl FromStr for UnusualData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(UnusualData(
            s.lines()
                .map(Report::from_str)
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

#[derive(Debug)]
struct Report(Vec<u32>);

impl Report {
    fn is_safe(&self) -> bool {
        Self::is_safe_impl(self.0.iter().copied())
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        for i in 0..self.0.len() {
            if Self::is_safe_impl(self.0.iter().copied().skip_nth(i)) {
                return true;
            }
        }
        false
    }

    fn is_safe_impl(mut levels: impl Iterator<Item = u32>) -> bool {
        let Some(first) = levels.next() else {
            // There are no levels to compare, so the report is safe.
            return true;
        };
        let Some(mut prev) = levels.next() else {
            // There is only one level to compare, so the report is safe.
            return true;
        };
        let order = first.cmp(&prev);
        if order == Ordering::Equal || first.abs_diff(prev) > 3 {
            // The difference between the two levels should be *at least* one.
            return false;
        }
        for level in levels {
            if prev.cmp(&level) != order || prev.abs_diff(level) > 3 {
                return false;
            }
            prev = level;
        }
        true
    }
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Report(
            s.split_ascii_whitespace()
                .map(|word| word.parse::<u32>().map_err(Error::from))
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let data = UnusualData::from_str(input)?;

    println!("Part 1: {:?}", data.safe_report_count());
    println!(
        "Part 2: {:?}",
        data.safe_report_count_with_problem_dampener()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn sample() {
        let data = UnusualData::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(data.safe_report_count(), 2);
        assert_eq!(data.safe_report_count_with_problem_dampener(), 4);
    }
}

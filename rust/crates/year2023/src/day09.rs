use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

/// Represents a history of values.
///
/// Invariant: The history must contain at least one value.
#[derive(Debug)]
struct History(Vec<i32>);

impl History {
    /// Creates a new history from the given values.
    ///
    /// # Panics
    ///
    /// If `values` is empty.
    fn new(values: Vec<i32>) -> History {
        assert!(!values.is_empty());
        History(values)
    }

    /// Returns `true` if all values in the history are zero.
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&value| value == 0)
    }

    /// Returns the first value in the history.
    fn first(&self) -> i32 {
        // SAFETY: The invariant guarantees that there is at least one value.
        *self.0.first().unwrap()
    }

    /// Returns the last value in the history.
    fn last(&self) -> i32 {
        // SAFETY: The invariant guarantees that there is at least one value.
        *self.0.last().unwrap()
    }

    /// Reduce the history by subtracting each value from the next.
    ///
    /// For example, the history `[1, 3, 6, 10]` would be reduced to `[2, 3, 4]`.
    fn reduce(&self) -> History {
        History::new(
            self.0
                .iter()
                .zip(self.0.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect(),
        )
    }

    /// Returns a vector of reduced histories excluding the current history.
    fn reduced_histories(&self) -> Vec<History> {
        std::iter::successors(Some(self.reduce()), |history| {
            (!history.is_zero()).then(|| history.reduce())
        })
        .collect()
    }

    /// Returns the previous value in the history.
    ///
    /// This is calculated by reducing the history until it is all zeroes, then extrapolating the
    /// previous value from the first value in each reduced history.
    fn prev(&self) -> i32 {
        self.first()
            - self
                .reduced_histories()
                .iter()
                .rfold(0, |value, history| history.first() - value)
    }

    /// Returns the next value in the history.
    ///
    /// This is calculated by reducing the history until it is all zeroes, then extrapolating the
    /// next value from the last value in each reduced history.
    fn next(&self) -> i32 {
        self.last()
            + self
                .reduced_histories()
                .iter()
                .rfold(0, |value, history| history.last() + value)
    }
}

impl FromStr for History {
    type Err = Error;

    fn from_str(s: &str) -> Result<History> {
        Ok(History::new(
            s.split_ascii_whitespace()
                .map(|s| {
                    s.parse::<i32>()
                        .with_context(|| anyhow!("Invalid number: {:?}", s))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

/// Represents the OASIS report which is a sequence of histories.
#[derive(Debug)]
struct OasisReport(Vec<History>);

impl OasisReport {
    /// Returns the sum of the previous values in each history.
    fn sum_prev(&self) -> i32 {
        self.0.iter().map(History::prev).sum()
    }

    /// Returns the sum of the next values in each history.
    fn sum_next(&self) -> i32 {
        self.0.iter().map(History::next).sum()
    }
}

impl FromStr for OasisReport {
    type Err = Error;

    fn from_str(s: &str) -> Result<OasisReport> {
        Ok(OasisReport(
            s.lines()
                .map(|line| {
                    History::from_str(line)
                        .with_context(|| anyhow!("Invalid history line: {:?}", line))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let report = OasisReport::from_str(input)?;

    println!("Part 1: {}", report.sum_next());
    println!("Part 2: {}", report.sum_prev());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_sample() {
        let report = OasisReport::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(report.sum_next(), 114);
        assert_eq!(report.sum_prev(), 2);
    }
}

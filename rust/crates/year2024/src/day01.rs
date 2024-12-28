use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};

/// Represents the two groups of location IDs.
#[derive(Debug, Default)]
struct LocationList {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl LocationList {
    /// Sorts the left and right sides of the location list.
    fn sort_unstable(&mut self) {
        self.left.sort_unstable();
        self.right.sort_unstable();
    }

    /// Returns the total distance between the left list and the right list.
    fn total_distance(&self) -> u32 {
        self.left
            .iter()
            .copied()
            .zip(self.right.iter().copied())
            .map(|(a, b)| a.abs_diff(b))
            .sum()
    }

    /// Returns the similarity score of the location list.
    fn similarity_score(&self) -> u32 {
        let mut counter = HashMap::new();
        for n in self.right.iter().copied() {
            *counter.entry(n).or_insert(0) += 1;
        }
        self.left
            .iter()
            .copied()
            .map(|n| n * counter.get(&n).copied().unwrap_or(0))
            .sum()
    }
}

impl FromStr for LocationList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut list = LocationList::default();
        for (index, line) in s.lines().enumerate() {
            let mut numbers = line
                .split_ascii_whitespace()
                .filter_map(|word| word.parse::<u32>().ok());
            list.left.push(
                numbers.next().ok_or_else(|| {
                    anyhow!("Failed to parse number on left side of line {index}")
                })?,
            );
            list.right.push(
                numbers.next().ok_or_else(|| {
                    anyhow!("Failed to parse number on right side of line {index}")
                })?,
            );
        }
        list.sort_unstable();
        Ok(list)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let list = LocationList::from_str(input)?;

    println!("Part 1: {:?}", list.total_distance());
    println!("Part 2: {:?}", list.similarity_score());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn sample() {
        let list = LocationList::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(list.total_distance(), 11);
        assert_eq!(list.similarity_score(), 31);
    }
}

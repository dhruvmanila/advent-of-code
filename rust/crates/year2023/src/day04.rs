use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: HashSet<u32>,
    available: HashSet<u32>,
}

impl ScratchCard {
    /// Returns the number of winners on this scratch card i.e., the count of
    /// available numbers that are a winning number.
    fn winning_count(&self) -> usize {
        self.winning_numbers.intersection(&self.available).count()
    }

    /// Returns the number of points this scratch card is worth.
    fn points(&self) -> u32 {
        self.winning_count()
            .checked_sub(1)
            .map_or(0, |count| 2u32.pow(count as u32))
    }
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (winners, available) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("Expected a colon in the line"))?
            .1
            .split_once('|')
            .ok_or_else(|| anyhow!("Expected a pipe in the line"))?;

        Ok(Self {
            winning_numbers: winners
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?,
            available: available
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?,
        })
    }
}

#[derive(Debug, Default)]
struct ScratchCards(Vec<ScratchCard>);

impl ScratchCards {
    /// Returns the total number of points won on all scratch cards.
    fn points(&self) -> u32 {
        self.0.iter().map(ScratchCard::points).sum()
    }

    /// Returns the total number of cards available after processing all of them
    /// as per the rules in part 2 of the puzzle.
    fn total_cards(&self) -> u32 {
        let mut quantity = vec![1u32; self.0.len()];
        for (idx, card) in self.0.iter().enumerate() {
            let count = quantity[idx];
            if let Some(quantities) = quantity.get_mut(idx + 1..idx + 1 + card.winning_count()) {
                for quantity in quantities {
                    *quantity += count;
                }
            }
        }
        quantity.iter().sum()
    }
}

impl FromStr for ScratchCards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(
            s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let scratchcards = input.parse::<ScratchCards>()?;

    println!("Part 1: {}", scratchcards.points());
    println!("Part 2: {}", scratchcards.total_cards());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_sample() -> Result<()> {
        let scratchcards = SAMPLE_INPUT.parse::<ScratchCards>()?;
        let points = scratchcards
            .0
            .iter()
            .map(ScratchCard::points)
            .collect::<Vec<_>>();

        assert_eq!(points, &[8, 2, 2, 1, 0, 0]);
        assert_eq!(scratchcards.points(), 13);
        assert_eq!(scratchcards.total_cards(), 30);

        Ok(())
    }
}

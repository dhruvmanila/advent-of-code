use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
struct ScratchCard {
    winning_numbers: HashSet<u32>,
    available: HashSet<u32>,
}

impl ScratchCard {
    /// Creates a new scratch card with the given winning numbers and available numbers.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in `winning_numbers` or `available` is >= 32.
    fn new(winning_numbers: HashSet<u32>, available: HashSet<u32>) -> ScratchCard {
        // These assertions are necessary because the points calculation uses a u32 to store the
        // number of points, which can only represent numbers up to 2^32 - 1.
        assert!(winning_numbers.len() < 32);
        assert!(available.len() < 32);

        ScratchCard {
            winning_numbers,
            available,
        }
    }

    /// Returns the number of winners on this scratch card i.e., the count of available numbers
    /// that are a winning number.
    fn winning_count(&self) -> usize {
        self.winning_numbers.intersection(&self.available).count()
    }

    /// Returns the number of points this scratch card is worth.
    fn points(&self) -> u32 {
        self.winning_count().checked_sub(1).map_or(0, |count| {
            // SAFETY: `count` is always less than 32
            2u32.pow(u32::try_from(count).unwrap())
        })
    }
}

impl FromStr for ScratchCard {
    type Err = Error;

    fn from_str(s: &str) -> Result<ScratchCard> {
        let (winners, available) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("Expected a colon in the line"))?
            .1
            .split_once('|')
            .ok_or_else(|| anyhow!("Expected a pipe in the line"))?;

        Ok(ScratchCard::new(
            winners
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?,
            available
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<_>, _>>()?,
        ))
    }
}

#[derive(Debug, Default)]
struct ScratchCards(Vec<ScratchCard>);

impl ScratchCards {
    /// Returns the total number of points won on all scratch cards.
    fn points(&self) -> u32 {
        self.0.iter().map(ScratchCard::points).sum()
    }

    /// Returns the total number of cards available after processing all of them as per the rules
    /// in part 2 of the puzzle.
    fn total_cards(&self) -> u32 {
        let mut quantity = vec![1u32; self.0.len()];
        for (idx, card) in self.0.iter().enumerate() {
            // SAFETY: `quantity` was initialized with the length of `self.0`
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
    type Err = Error;

    fn from_str(s: &str) -> Result<ScratchCards> {
        Ok(ScratchCards(
            s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let scratch_cards = ScratchCards::from_str(input)?;

    println!("Part 1: {}", scratch_cards.points());
    println!("Part 2: {}", scratch_cards.total_cards());

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
    fn test_sample() {
        let scratchcards = ScratchCards::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(scratchcards.points(), 13);
        assert_eq!(scratchcards.total_cards(), 30);
    }
}

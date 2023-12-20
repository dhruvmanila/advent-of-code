use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

/// Card types.
///
/// Each card is aware of it's ordering, which means that they can be compared
/// directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    /// Construct a card from the given byte (`u8`).
    ///
    /// Valid bytes represent the following cards:
    /// - `A`: Ace
    /// - `K`: King
    /// - `Q`: Queen
    /// - `J`: Jack
    /// - `T`: 10
    /// - `0`..=`9`: 0..=9
    ///
    /// Returns `None` if the given byte does not represent a valid card.
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'A' => Some(Self::Ace),
            b'K' => Some(Self::King),
            b'Q' => Some(Self::Queen),
            b'J' => Some(Self::Jack),
            b'T' => Some(Self::Number(10)),
            digit @ b'0'..=b'9' => Some(Self::Number(digit - b'0')),
            _ => None,
        }
    }
}

/// Hand types.
///
/// Each type is aware of it's ordering, which means that they can be compared
/// directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// Represents a hand of five cards.
#[derive(Debug)]
struct Hand([Card; 5]);

impl Hand {
    /// Returns the cards in this hand.
    fn cards(&self) -> &[Card; 5] {
        &self.0
    }

    /// Returns the kind of this hand.
    fn kind(&self) -> HandType {
        let mut card_count = HashMap::new();
        for card in self.0.iter() {
            *card_count.entry(card).or_insert(0) += 1;
        }
        match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => unreachable!("Impossible hand: {:?}", self.0),
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [first, second, third, fourth, fifth] = s.as_bytes() else {
            bail!("Invalid hand: {:?} (expected 5 cards)", s);
        };

        Ok(Hand([
            Card::from_byte(*first).ok_or_else(|| anyhow!("Invalid card at 0: {:?}", first))?,
            Card::from_byte(*second).ok_or_else(|| anyhow!("Invalid card at 1: {:?}", second))?,
            Card::from_byte(*third).ok_or_else(|| anyhow!("Invalid card at 2: {:?}", third))?,
            Card::from_byte(*fourth).ok_or_else(|| anyhow!("Invalid card at 3: {:?}", fourth))?,
            Card::from_byte(*fifth).ok_or_else(|| anyhow!("Invalid card at 4: {:?}", fifth))?,
        ]))
    }
}

/// Represents a weighted hand. This is a hand with a bid value.
#[derive(Debug)]
struct WeightedHand {
    hand: Hand,
    bid: u32,
}

impl PartialEq for WeightedHand {
    /// Two weighted hands are equal if the type of their hands are equal.
    fn eq(&self, other: &Self) -> bool {
        self.hand.kind() == other.hand.kind()
    }
}

impl Eq for WeightedHand {}

impl PartialOrd for WeightedHand {
    /// Returns the ordering of two weighted hands. This is based on the
    /// ordering of their underlying hands.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WeightedHand {
    /// Returns the total ordering of two weighted hands.
    ///
    /// This is based on the ordering of their underlying hands. If the hands
    /// are of the same type, then a secondary ordering rule takes effect. This
    /// is done by comparing the cards in order.
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand.kind().cmp(&other.hand.kind()) {
            Ordering::Equal => self.hand.cards().cmp(other.hand.cards()),
            ordering => ordering,
        }
    }
}

impl FromStr for WeightedHand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid input: {:?}", s))?;

        Ok(Self {
            hand: hand.parse::<Hand>()?,
            bid: bid
                .parse::<u32>()
                .map_err(|e| anyhow!("Invalid bid value {:?}: {:?}", bid, e))?,
        })
    }
}

/// Represents a collection of weighted hands.
#[derive(Debug, Default)]
struct Hands(Vec<WeightedHand>);

impl Hands {
    /// Consumes this collection of hands and returns a new collection with the
    /// hands sorted by their [`HandType`].
    fn into_sorted(self) -> Self {
        let mut hands = self;
        hands.0.sort_unstable();
        hands
    }

    /// Returns the total winnings of this collection of hands.
    ///
    /// This is the sum of the bid values of each hand multiplied by the rank
    /// of the hand.
    ///
    /// This function assumes that the hands are sorted by their [`HandType`].
    /// If not, the result will be incorrect. Use [`Hands::into_sorted`] to
    /// sort the hands.
    fn total_winnings(&self) -> u32 {
        (1..)
            .zip(self.0.iter())
            .map(|(rank, hand)| rank * hand.bid)
            .sum()
    }
}

impl FromStr for Hands {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.parse::<WeightedHand>())
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let hands = input.parse::<Hands>()?.into_sorted();

    println!("Part 1: {}", hands.total_winnings());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    const SAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test_case("33333", HandType::FiveOfAKind)]
    #[test_case("33A33", HandType::FourOfAKind)]
    #[test_case("2K2K2", HandType::FullHouse)]
    #[test_case("5J55T", HandType::ThreeOfAKind)]
    #[test_case("2K2K3", HandType::TwoPair)]
    #[test_case("8KQJ8", HandType::OnePair)]
    #[test_case("AKQJ8", HandType::HighCard)]
    fn test_hand_kind(cards: &str, expected: HandType) -> Result<()> {
        assert_eq!(cards.parse::<Hand>()?.kind(), expected);
        Ok(())
    }

    #[test]
    fn test_sample() -> Result<()> {
        let hands = SAMPLE_INPUT.parse::<Hands>()?.into_sorted();
        assert_eq!(hands.total_winnings(), 6440);
        Ok(())
    }
}

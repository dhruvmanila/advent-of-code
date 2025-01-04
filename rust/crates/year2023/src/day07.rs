use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

/// Abstraction for a camel hand.
trait CamelHand: FromStr {
    type Card: Ord;
    /// Returns the cards in this hand.
    fn cards(&self) -> &[Self::Card; 5];
    /// Determines the kind of this hand.
    fn kind(&self) -> HandType;
}

/// Card types.
///
/// Each card is aware of it's ordering, which means that they can be compared
/// directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NormalHandCard {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl NormalHandCard {
    /// Construct a normal hand card from the given byte (`u8`).
    ///
    /// Returns `None` if the given byte does not represent a valid card.
    fn from_byte(byte: u8) -> Option<NormalHandCard> {
        match byte {
            b'A' => Some(NormalHandCard::Ace),
            b'K' => Some(NormalHandCard::King),
            b'Q' => Some(NormalHandCard::Queen),
            b'J' => Some(NormalHandCard::Jack),
            b'T' => Some(NormalHandCard::Number(10)),
            digit @ b'0'..=b'9' => Some(NormalHandCard::Number(digit - b'0')),
            _ => None,
        }
    }
}

/// Card types for a joker hand.
///
/// Each card is aware of it's ordering, which means that they can be compared
/// directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum JokerHandCard {
    Jack,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl JokerHandCard {
    /// Construct a joker hand card from the given byte (`u8`).
    ///
    /// Returns `None` if the given byte does not represent a valid card.
    fn from_byte(byte: u8) -> Option<JokerHandCard> {
        match byte {
            b'A' => Some(JokerHandCard::Ace),
            b'K' => Some(JokerHandCard::King),
            b'Q' => Some(JokerHandCard::Queen),
            b'J' => Some(JokerHandCard::Jack),
            b'T' => Some(JokerHandCard::Number(10)),
            digit @ b'0'..=b'9' => Some(JokerHandCard::Number(digit - b'0')),
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

/// Represents a normal hand of five cards.
#[derive(Debug)]
struct NormalHand([NormalHandCard; 5]);

impl CamelHand for NormalHand {
    type Card = NormalHandCard;

    /// Returns the cards in this hand.
    fn cards(&self) -> &[NormalHandCard; 5] {
        &self.0
    }

    /// Returns the kind of this hand.
    fn kind(&self) -> HandType {
        let mut card_count = HashMap::new();
        for card in &self.0 {
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

impl FromStr for NormalHand {
    type Err = Error;

    fn from_str(s: &str) -> Result<NormalHand> {
        let [first, second, third, fourth, fifth] = s.as_bytes() else {
            bail!("Invalid hand: {:?} (expected 5 cards)", s);
        };

        Ok(NormalHand([
            NormalHandCard::from_byte(*first)
                .ok_or_else(|| anyhow!("Invalid card at 0: {:?}", first))?,
            NormalHandCard::from_byte(*second)
                .ok_or_else(|| anyhow!("Invalid card at 1: {:?}", second))?,
            NormalHandCard::from_byte(*third)
                .ok_or_else(|| anyhow!("Invalid card at 2: {:?}", third))?,
            NormalHandCard::from_byte(*fourth)
                .ok_or_else(|| anyhow!("Invalid card at 3: {:?}", fourth))?,
            NormalHandCard::from_byte(*fifth)
                .ok_or_else(|| anyhow!("Invalid card at 4: {:?}", fifth))?,
        ]))
    }
}

/// Represents a joker hand of five cards.
#[derive(Debug)]
struct JokerHand([JokerHandCard; 5]);

impl CamelHand for JokerHand {
    type Card = JokerHandCard;

    /// Returns the cards in this hand.
    fn cards(&self) -> &[JokerHandCard; 5] {
        &self.0
    }

    /// Returns the kind of this hand.
    fn kind(&self) -> HandType {
        let mut card_count = HashMap::new();
        for card in &self.0 {
            *card_count.entry(card).or_insert(0) += 1;
        }
        match card_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if card_count.values().any(|&count| count == 4) {
                    if card_count.contains_key(&JokerHandCard::Jack) {
                        // Four of a kind upgraded to five of a kind because
                        // of the joker.
                        HandType::FiveOfAKind
                    } else {
                        HandType::FourOfAKind
                    }
                } else if card_count.contains_key(&JokerHandCard::Jack) {
                    // Full house upgraded to five of a kind because of the
                    // joker.
                    HandType::FiveOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if card_count.values().any(|&count| count == 3) {
                    if card_count.contains_key(&JokerHandCard::Jack) {
                        // Three of a kind upgraded to four of a kind because
                        // of the joker.
                        HandType::FourOfAKind
                    } else {
                        HandType::ThreeOfAKind
                    }
                } else {
                    card_count
                        .get(&JokerHandCard::Jack)
                        .map_or(HandType::TwoPair, |&count| {
                            if count == 2 {
                                // Two pair upgraded to four of a kind because
                                // of the joker.
                                HandType::FourOfAKind
                            } else {
                                // Two pair upgraded to full house because of
                                // the joker.
                                HandType::FullHouse
                            }
                        })
                }
            }
            4 => {
                if card_count.contains_key(&JokerHandCard::Jack) {
                    // One pair upgraded to three of a kind because of the
                    // joker.
                    HandType::ThreeOfAKind
                } else {
                    HandType::OnePair
                }
            }
            5 => {
                if card_count.contains_key(&JokerHandCard::Jack) {
                    // High card upgraded to one pair because of the joker.
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
            _ => unreachable!("Impossible hand: {:?}", self.0),
        }
    }
}

impl FromStr for JokerHand {
    type Err = Error;

    fn from_str(s: &str) -> Result<JokerHand> {
        let [first, second, third, fourth, fifth] = s.as_bytes() else {
            bail!("Invalid hand: {:?} (expected 5 cards)", s);
        };

        Ok(JokerHand([
            JokerHandCard::from_byte(*first)
                .ok_or_else(|| anyhow!("Invalid card at 0: {:?}", first))?,
            JokerHandCard::from_byte(*second)
                .ok_or_else(|| anyhow!("Invalid card at 1: {:?}", second))?,
            JokerHandCard::from_byte(*third)
                .ok_or_else(|| anyhow!("Invalid card at 2: {:?}", third))?,
            JokerHandCard::from_byte(*fourth)
                .ok_or_else(|| anyhow!("Invalid card at 3: {:?}", fourth))?,
            JokerHandCard::from_byte(*fifth)
                .ok_or_else(|| anyhow!("Invalid card at 4: {:?}", fifth))?,
        ]))
    }
}

/// Represents a weighted hand. This is a hand with a bid value.
#[derive(Debug)]
struct WeightedHand<H: CamelHand> {
    hand: H,
    bid: u32,
}

impl<H: CamelHand> PartialEq for WeightedHand<H> {
    /// Two weighted hands are equal if the type of their hands are equal.
    fn eq(&self, other: &WeightedHand<H>) -> bool {
        self.hand.kind() == other.hand.kind()
    }
}

impl<H: CamelHand> Eq for WeightedHand<H> {}

impl<H: CamelHand> PartialOrd for WeightedHand<H> {
    /// Returns the ordering of two weighted hands. This is based on the
    /// ordering of their underlying hands.
    fn partial_cmp(&self, other: &WeightedHand<H>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<H: CamelHand> Ord for WeightedHand<H> {
    /// Returns the total ordering of two weighted hands.
    ///
    /// This is based on the ordering of their underlying hands. If the hands
    /// are of the same type, then a secondary ordering rule takes effect. This
    /// is done by comparing the cards in order.
    fn cmp(&self, other: &WeightedHand<H>) -> Ordering {
        match self.hand.kind().cmp(&other.hand.kind()) {
            Ordering::Equal => self.hand.cards().cmp(other.hand.cards()),
            ordering => ordering,
        }
    }
}

impl<H: CamelHand> FromStr for WeightedHand<H> {
    type Err = Error;

    fn from_str(s: &str) -> Result<WeightedHand<H>> {
        let (hand, bid) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid input: {:?}", s))?;

        Ok(WeightedHand {
            hand: H::from_str(hand).map_err(|_| anyhow!("Invalid hand {:?}", hand))?,
            bid: bid
                .parse::<u32>()
                .map_err(|e| anyhow!("Invalid bid value {:?}: {:?}", bid, e))?,
        })
    }
}

/// Represents a collection of weighted hands.
#[derive(Debug, Default)]
struct Hands<H: CamelHand>(Vec<WeightedHand<H>>);

impl<H: CamelHand> Hands<H> {
    /// Consumes this collection of hands and returns a new collection with the hands sorted by
    /// their [`HandType`].
    fn into_sorted(self) -> Self {
        let mut hands = self;
        hands.0.sort_unstable();
        hands
    }

    /// Returns the total winnings of this collection of hands.
    ///
    /// This is the sum of the bid values of each hand multiplied by the rank of the hand.
    ///
    /// This function assumes that the hands are sorted by their [`HandType`]. If not, the result
    /// will be incorrect. Use [`Hands::into_sorted`] to sort the hands.
    fn total_winnings(&self) -> u32 {
        (1..)
            .zip(self.0.iter())
            .map(|(rank, hand)| rank * hand.bid)
            .sum()
    }
}

impl<H: CamelHand> FromStr for Hands<H> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Hands<H>> {
        Ok(Hands(
            s.lines().map(str::parse).collect::<Result<Vec<_>>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let normal_hands = Hands::<NormalHand>::from_str(input)?.into_sorted();
    println!("Part 1: {}", normal_hands.total_winnings());

    let joker_hands = Hands::<JokerHand>::from_str(input)?.into_sorted();
    println!("Part 2: {}", joker_hands.total_winnings());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test_case(NormalHandCard::Number(2), NormalHandCard::Number(3), Ordering::Less)]
    #[test_case(NormalHandCard::Number(2), NormalHandCard::Number(2), Ordering::Equal)]
    #[test_case(
        NormalHandCard::Number(2),
        NormalHandCard::Number(1),
        Ordering::Greater
    )]
    #[test_case(NormalHandCard::Number(2), NormalHandCard::Jack, Ordering::Less)]
    #[test_case(NormalHandCard::Jack, NormalHandCard::Queen, Ordering::Less)]
    #[test_case(NormalHandCard::Queen, NormalHandCard::King, Ordering::Less)]
    #[test_case(NormalHandCard::King, NormalHandCard::Ace, Ordering::Less)]
    fn normal_hand_card_ordering(card1: NormalHandCard, card2: NormalHandCard, expected: Ordering) {
        assert_eq!(card1.cmp(&card2), expected);
    }

    #[test_case(JokerHandCard::Jack, JokerHandCard::Number(2), Ordering::Less)]
    #[test_case(JokerHandCard::Jack, JokerHandCard::Jack, Ordering::Equal)]
    #[test_case(JokerHandCard::Jack, JokerHandCard::Queen, Ordering::Less)]
    fn joker_hand_card_ordering(card1: JokerHandCard, card2: JokerHandCard, expected: Ordering) {
        assert_eq!(card1.cmp(&card2), expected);
    }

    #[test_case("33333", HandType::FiveOfAKind)]
    #[test_case("33A33", HandType::FourOfAKind)]
    #[test_case("2K2K2", HandType::FullHouse)]
    #[test_case("5J55T", HandType::ThreeOfAKind)]
    #[test_case("2K2K3", HandType::TwoPair)]
    #[test_case("8KQJ8", HandType::OnePair)]
    #[test_case("AKQJ8", HandType::HighCard)]
    fn normal_hand_kind(cards: &str, expected: HandType) -> Result<()> {
        assert_eq!(cards.parse::<NormalHand>()?.kind(), expected);
        Ok(())
    }

    #[test_case("33333", HandType::FiveOfAKind)]
    #[test_case("JJJJJ", HandType::FiveOfAKind)]
    #[test_case("33J33", HandType::FiveOfAKind)]
    #[test_case("JJ3JJ", HandType::FiveOfAKind)]
    #[test_case("33JJJ", HandType::FiveOfAKind)]
    #[test_case("JJ333", HandType::FiveOfAKind)]
    #[test_case("33A33", HandType::FourOfAKind)]
    #[test_case("222JK", HandType::FourOfAKind)]
    #[test_case("JJJ2K", HandType::FourOfAKind)]
    #[test_case("22JJK", HandType::FourOfAKind)]
    #[test_case("2K2K2", HandType::FullHouse)]
    #[test_case("2244J", HandType::FullHouse)]
    #[test_case("5552T", HandType::ThreeOfAKind)]
    #[test_case("JKT55", HandType::ThreeOfAKind)]
    #[test_case("2K2K3", HandType::TwoPair)]
    #[test_case("8KQ68", HandType::OnePair)]
    #[test_case("AKQJ8", HandType::OnePair)]
    #[test_case("AKQ48", HandType::HighCard)]
    fn joker_hand_kind(cards: &str, expected: HandType) -> Result<()> {
        assert_eq!(cards.parse::<JokerHand>()?.kind(), expected);
        Ok(())
    }

    #[test]
    fn sample() -> Result<()> {
        let normal_hands = SAMPLE_INPUT.parse::<Hands<NormalHand>>()?.into_sorted();
        assert_eq!(normal_hands.total_winnings(), 6440);

        let joker_hands = SAMPLE_INPUT.parse::<Hands<JokerHand>>()?.into_sorted();
        assert_eq!(joker_hands.total_winnings(), 5905);

        Ok(())
    }
}

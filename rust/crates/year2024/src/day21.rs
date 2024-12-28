use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{anyhow, Error, Result};
use aoc_lib::matrix::{CardinalDirection, Position};
use itertools::Itertools;

/// Returns an iterator of direction tiles that represent the shortest path between two positions.
///
/// The `gap` position is used to make sure that the path doesn't go through it.
///
/// Reference: <https://observablehq.com/@jwolondon/advent-of-code-2024-day-21>
fn shortest_path(
    start: Position,
    end: Position,
    gap: Position,
) -> impl Iterator<Item = DirectionTile> {
    let vertical = if end.row() > start.row() {
        std::iter::repeat(DirectionTile::Direction(CardinalDirection::Down))
            .take(end.row() - start.row())
    } else {
        std::iter::repeat(DirectionTile::Direction(CardinalDirection::Up))
            .take(start.row() - end.row())
    };

    let horizontal = if end.col() > start.col() {
        std::iter::repeat(DirectionTile::Direction(CardinalDirection::Right))
            .take(end.col() - start.col())
    } else {
        std::iter::repeat(DirectionTile::Direction(CardinalDirection::Left))
            .take(start.col() - end.col())
    };

    if end.col() > start.col() && Position::new(end.row(), start.col()) != gap {
        vertical.chain(horizontal)
    } else if Position::new(start.row(), end.col()) != gap {
        horizontal.chain(vertical)
    } else {
        vertical.chain(horizontal)
    }
    .chain(std::iter::once(DirectionTile::Activate))
}

/// A tile that represents a digit or the activation tile on a numeric keypad.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum NumericTile {
    /// A digit tile from 0 to 9.
    Digit(u8),
    /// The activation tile.
    Activate,
}

impl NumericTile {
    /// Returns the digit value of the tile if it's a digit, [`None`] otherwise.
    const fn as_digit(self) -> Option<u8> {
        match self {
            NumericTile::Digit(digit) => Some(digit),
            NumericTile::Activate => None,
        }
    }
}

impl TryFrom<u8> for NumericTile {
    type Error = Error;

    fn try_from(value: u8) -> Result<NumericTile, Error> {
        match value {
            b'0'..=b'9' => Ok(NumericTile::Digit(value - b'0')),
            b'A' => Ok(NumericTile::Activate),
            _ => Err(anyhow!("Invalid numeric tile character: {}", value as char)),
        }
    }
}

impl fmt::Debug for NumericTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumericTile::Digit(digit) => write!(f, "{digit}"),
            NumericTile::Activate => f.write_char('A'),
        }
    }
}

/// A tile that represents a cardinal direction or the activation tile on a direction keypad.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum DirectionTile {
    /// A cardinal direction tile (up, down, left, or right).
    Direction(CardinalDirection),
    /// The activation tile.
    Activate,
}

impl DirectionTile {
    /// All possible direction tiles.
    const ALL: [DirectionTile; 5] = [
        DirectionTile::Direction(CardinalDirection::Up),
        DirectionTile::Direction(CardinalDirection::Down),
        DirectionTile::Direction(CardinalDirection::Left),
        DirectionTile::Direction(CardinalDirection::Right),
        DirectionTile::Activate,
    ];

    /// Returns the position of the tile on the directional keypad.
    #[must_use]
    const fn position(self) -> Position {
        match self {
            DirectionTile::Direction(CardinalDirection::Up) => Position::new(0, 1),
            DirectionTile::Activate => Position::new(0, 2),
            DirectionTile::Direction(CardinalDirection::Left) => Position::new(1, 0),
            DirectionTile::Direction(CardinalDirection::Down) => Position::new(1, 1),
            DirectionTile::Direction(CardinalDirection::Right) => Position::new(1, 2),
        }
    }
}

impl fmt::Debug for DirectionTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectionTile::Direction(CardinalDirection::Up) => f.write_char('^'),
            DirectionTile::Direction(CardinalDirection::Down) => f.write_char('v'),
            DirectionTile::Direction(CardinalDirection::Left) => f.write_char('<'),
            DirectionTile::Direction(CardinalDirection::Right) => f.write_char('>'),
            DirectionTile::Activate => f.write_char('A'),
        }
    }
}

#[derive(Debug)]
struct NumericKeypad {
    /// The positions of the numeric tiles on the keypad.
    positions: HashMap<NumericTile, Position>,
    /// The position of the gap on the keypad.
    gap: Position,
}

impl NumericKeypad {
    fn new() -> NumericKeypad {
        NumericKeypad {
            positions: [
                (NumericTile::Digit(7), Position::new(0, 0)),
                (NumericTile::Digit(8), Position::new(0, 1)),
                (NumericTile::Digit(9), Position::new(0, 2)),
                (NumericTile::Digit(4), Position::new(1, 0)),
                (NumericTile::Digit(5), Position::new(1, 1)),
                (NumericTile::Digit(6), Position::new(1, 2)),
                (NumericTile::Digit(1), Position::new(2, 0)),
                (NumericTile::Digit(2), Position::new(2, 1)),
                (NumericTile::Digit(3), Position::new(2, 2)),
                (NumericTile::Digit(0), Position::new(3, 1)),
                (NumericTile::Activate, Position::new(3, 2)),
            ]
            .into_iter()
            .collect(),
            gap: Position::new(3, 0),
        }
    }

    /// Returns an iterator of [`DirectionalInstruction`] for each numeric `tiles` on the keypad.
    ///
    /// The first instruction starts from the activation tile.
    fn instructions<'a, 'b>(
        &'a self,
        tiles: &'b [NumericTile],
    ) -> NumericKeypadInstructions<'a, 'b> {
        NumericKeypadInstructions {
            keypad: self,
            tiles: tiles.iter(),
            previous: *self.positions.get(&NumericTile::Activate).unwrap(),
        }
    }
}

/// An iterator of directional instructions for each numeric tile on the keypad.
struct NumericKeypadInstructions<'a, 'b> {
    keypad: &'a NumericKeypad,
    tiles: std::slice::Iter<'b, NumericTile>,
    previous: Position,
}

impl<'a> Iterator for NumericKeypadInstructions<'a, '_> {
    type Item = DirectionalInstruction<'a>;

    fn next(&mut self) -> Option<DirectionalInstruction<'a>> {
        let current = self.keypad.positions.get(self.tiles.next()?).unwrap();
        let path = shortest_path(self.previous, *current, self.keypad.gap);
        self.previous = *current;
        Some(DirectionalInstruction(path.collect()))
    }
}

impl FusedIterator for NumericKeypadInstructions<'_, '_> {}

struct DirectionalKeypad {
    /// A lookup table of the shortest path between each pair of direction tiles.
    lookup: HashMap<(DirectionTile, DirectionTile), Vec<DirectionTile>>,
}

impl DirectionalKeypad {
    fn new() -> DirectionalKeypad {
        DirectionalKeypad {
            lookup: DirectionTile::ALL
                .into_iter()
                .permutations(2)
                .map(|pair| {
                    let &[source, target] = &*pair else {
                        panic!("Expected a pair of direction tiles, got: {pair:?}");
                    };
                    (
                        (source, target),
                        shortest_path(source.position(), target.position(), Position::zero())
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    /// Returns the shortest path between two direction tiles.
    fn shortest_path(&self, start: DirectionTile, end: DirectionTile) -> &[DirectionTile] {
        static SAME_TILE: [DirectionTile; 1] = [DirectionTile::Activate];

        // Short-circuit if the start and end tiles are the same because the lookup table doesn't
        // contain such pairs.
        if start == end {
            return &SAME_TILE;
        }

        // SAFETY: The lookup table is initialized with all possible pairs of direction tiles.
        self.lookup.get(&(start, end)).unwrap()
    }

    /// Returns an iterator of [`DirectionalInstruction`] for each direction `tiles` on the keypad.
    ///
    /// The first instruction starts from the activation tile.
    fn instructions<'a, 'b>(
        &'a self,
        tiles: &'b [DirectionTile],
    ) -> DirectionKeypadInstructions<'a, 'b> {
        DirectionKeypadInstructions {
            keypad: self,
            tiles: tiles.iter(),
            previous: DirectionTile::Activate,
        }
    }
}

/// An iterator of directional instructions for each direction tile on the keypad.
struct DirectionKeypadInstructions<'a, 'b> {
    keypad: &'a DirectionalKeypad,
    tiles: std::slice::Iter<'b, DirectionTile>,
    previous: DirectionTile,
}

impl<'a> Iterator for DirectionKeypadInstructions<'a, '_> {
    type Item = DirectionalInstruction<'a>;

    fn next(&mut self) -> Option<DirectionalInstruction<'a>> {
        let current = self.tiles.next()?;
        let path = self.keypad.shortest_path(self.previous, *current);
        self.previous = *current;
        Some(DirectionalInstruction(Cow::Borrowed(path)))
    }
}

impl FusedIterator for DirectionKeypadInstructions<'_, '_> {}

/// A directional instruction that represents a sequence of direction tiles.
#[derive(PartialEq, Eq, Hash)]
struct DirectionalInstruction<'a>(Cow<'a, [DirectionTile]>);

impl Deref for DirectionalInstruction<'_> {
    type Target = [DirectionTile];

    fn deref(&self) -> &[DirectionTile] {
        &self.0
    }
}

impl fmt::Debug for DirectionalInstruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tile in self.0.iter() {
            write!(f, "{tile:?}")?;
        }
        Ok(())
    }
}

/// A numeric keypad of the following layout:
///
/// ```text
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
static NUMERIC_KEYPAD: LazyLock<NumericKeypad> = LazyLock::new(NumericKeypad::new);

/// A directional keypad of the following layout:
///
/// ```text
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
static DIRECTION_KEYPAD: LazyLock<DirectionalKeypad> = LazyLock::new(DirectionalKeypad::new);

/// A door code that's made up of 4 numeric tiles.
struct DoorCode([NumericTile; 4]);

impl DoorCode {
    /// Returns the numeric part of the code (ignoring leading zeroes).
    fn number(&self) -> u32 {
        self.iter()
            .map_while(|tile| tile.as_digit())
            .fold(0, |number, digit| number * 10 + u32::from(digit))
    }

    /// Returns the complexity of the code with `n` directional robots.
    ///
    /// The configuration of the keypads is as follows:
    /// 1. One numeric keypad (on a door) where this code needs to be typed by a robot
    /// 2. `n` directional keypads that the robots are using
    /// 3. One directional keypad that **you** are using
    fn complexity(&self, n: u32) -> usize {
        let mut frequencies: HashMap<DirectionalInstruction<'_>, usize> = HashMap::new();
        for instruction in NUMERIC_KEYPAD.instructions(self) {
            *frequencies.entry(instruction).or_default() += 1;
        }
        for _ in 0..n {
            let mut new_frequencies: HashMap<DirectionalInstruction<'_>, usize> = HashMap::new();
            for (instruction, count) in &frequencies {
                for next_instruction in DIRECTION_KEYPAD.instructions(instruction) {
                    *new_frequencies.entry(next_instruction).or_default() += count;
                }
            }
            frequencies = new_frequencies;
        }
        let number = self.number() as usize;
        frequencies
            .iter()
            .fold(0, |complexity, (instruction, count)| {
                complexity + number * count * instruction.len()
            })
    }
}

impl Deref for DoorCode {
    type Target = [NumericTile];

    fn deref(&self) -> &[NumericTile] {
        &self.0
    }
}

impl FromStr for DoorCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<DoorCode, Error> {
        let &[first, second, third, fourth] = s.as_bytes() else {
            return Err(anyhow!("Invalid door code: {} (expected 4 characters)", s));
        };
        Ok(DoorCode([
            NumericTile::try_from(first)?,
            NumericTile::try_from(second)?,
            NumericTile::try_from(third)?,
            NumericTile::try_from(fourth)?,
        ]))
    }
}

impl fmt::Debug for DoorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tile in self.iter() {
            write!(f, "{tile:?}")?;
        }
        Ok(())
    }
}

/// A collection of door codes.
struct DoorCodes(Vec<DoorCode>);

impl DoorCodes {
    /// Returns the total complexity of all door codes with `n` directional robots.
    fn sum_complexity(&self, n: u32) -> usize {
        self.iter().map(|code| code.complexity(n)).sum()
    }
}

impl Deref for DoorCodes {
    type Target = [DoorCode];

    fn deref(&self) -> &[DoorCode] {
        &self.0
    }
}

impl FromStr for DoorCodes {
    type Err = Error;

    fn from_str(s: &str) -> Result<DoorCodes, Error> {
        let codes = s
            .lines()
            .map(DoorCode::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(DoorCodes(codes))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let codes = DoorCodes::from_str(input)?;

    println!("Part 1: {}", codes.sum_complexity(2));
    println!("Part 2: {}", codes.sum_complexity(25));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    fn sample() {
        let codes = DoorCodes::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(codes.sum_complexity(2), 126_384);
    }
}

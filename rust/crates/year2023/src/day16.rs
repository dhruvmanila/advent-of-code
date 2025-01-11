use std::collections::HashSet;
use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use aoc_lib::matrix::{CardinalDirection, Position, SquareMatrix};

/// The kind of splitter tile.
#[derive(Debug, Copy, Clone)]
enum SplitterTile {
    /// A horizontal splitter tile (`-`).
    Horizontal,
    /// A vertical splitter tile (`|`).
    Vertical,
}

impl fmt::Display for SplitterTile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SplitterTile::Horizontal => f.write_char('-'),
            SplitterTile::Vertical => f.write_char('|'),
        }
    }
}

/// The kind of mirror tile.
#[derive(Debug, Copy, Clone)]
enum MirrorTile {
    /// A forward slash mirror tile (`/`).
    Forward,
    /// A backward slash mirror tile (`\`).
    Backward,
}

impl fmt::Display for MirrorTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MirrorTile::Forward => f.write_char('/'),
            MirrorTile::Backward => f.write_char('\\'),
        }
    }
}

/// A tile in the contraption.
#[derive(Debug, Copy, Clone)]
enum Tile {
    /// An empty tile (`.`).
    Empty,
    /// A mirror tile (`/` or `\`).
    Mirror(MirrorTile),
    /// A splitter tile (`-` or `|`).
    Splitter(SplitterTile),
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Tile> {
        match value {
            b'.' => Ok(Tile::Empty),
            b'/' => Ok(Tile::Mirror(MirrorTile::Forward)),
            b'\\' => Ok(Tile::Mirror(MirrorTile::Backward)),
            b'-' => Ok(Tile::Splitter(SplitterTile::Horizontal)),
            b'|' => Ok(Tile::Splitter(SplitterTile::Vertical)),
            _ => Err(anyhow!("Invalid tile character: {}", value as char)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => f.write_char('.'),
            Tile::Mirror(mirror) => write!(f, "{mirror}"),
            Tile::Splitter(splitter) => write!(f, "{splitter}"),
        }
    }
}

/// A light beam in the contraption.
#[derive(Debug)]
struct LightBeam {
    /// The position of the light beam.
    position: Position,
    /// The direction of the light beam.
    direction: CardinalDirection,
}

impl LightBeam {
    /// Creates a new light beam with the given position and direction.
    const fn new(position: Position, direction: CardinalDirection) -> LightBeam {
        LightBeam {
            position,
            direction,
        }
    }

    /// Creates a new light beam with the given position.
    const fn with_position(mut self, position: Position) -> LightBeam {
        self.position = position;
        self
    }

    /// Creates a new light beam with the given direction.
    const fn with_direction(mut self, direction: CardinalDirection) -> LightBeam {
        self.direction = direction;
        self
    }

    /// Returns the next position of the light beam, [`None`] if the light beam has moved out of
    /// the contraption.
    fn next_position(&self) -> Option<Position> {
        self.position.checked_neighbor(self.direction.into())
    }
}

/// The contraption matrix.
#[derive(Debug)]
struct Contraption(SquareMatrix<Tile>);

impl Contraption {
    /// Returns the number of energized tiles in the contraption for the given light beam.
    ///
    /// This simulates the movement of the light beam through the contraption until it moves out
    /// of the matrix.
    fn energized_count(&self, start: LightBeam) -> usize {
        let mut light_beams = vec![start];
        let mut energized = HashSet::new();
        let mut splitted = HashSet::new();

        while let Some(mut light_beam) = light_beams.pop() {
            let Some(tile) = self.0.get(light_beam.position) else {
                // The light beam has moved out of the matrix.
                continue;
            };

            energized.insert(light_beam.position);

            // The second light beam that got created using the splitter tile.
            let mut second_light_beam = None;

            match tile {
                Tile::Mirror(mirror_tile) => {
                    let new_direction = match mirror_tile {
                        MirrorTile::Forward => match light_beam.direction {
                            CardinalDirection::Up => CardinalDirection::Right,
                            CardinalDirection::Right => CardinalDirection::Up,
                            CardinalDirection::Down => CardinalDirection::Left,
                            CardinalDirection::Left => CardinalDirection::Down,
                        },
                        MirrorTile::Backward => match light_beam.direction {
                            CardinalDirection::Up => CardinalDirection::Left,
                            CardinalDirection::Right => CardinalDirection::Down,
                            CardinalDirection::Down => CardinalDirection::Right,
                            CardinalDirection::Left => CardinalDirection::Up,
                        },
                    };
                    light_beam = light_beam.with_direction(new_direction);
                }
                Tile::Splitter(splitter_tile) => {
                    match splitter_tile {
                        SplitterTile::Horizontal => {
                            if light_beam.direction.is_vertical() {
                                if !splitted.insert(light_beam.position) {
                                    continue;
                                }
                                second_light_beam = Some(LightBeam::new(
                                    light_beam.position,
                                    CardinalDirection::Left,
                                ));
                                light_beam = light_beam.with_direction(CardinalDirection::Right);
                            }
                        }
                        SplitterTile::Vertical => {
                            if light_beam.direction.is_horizontal() {
                                if !splitted.insert(light_beam.position) {
                                    continue;
                                }
                                second_light_beam = Some(LightBeam::new(
                                    light_beam.position,
                                    CardinalDirection::Up,
                                ));
                                light_beam = light_beam.with_direction(CardinalDirection::Down);
                            }
                        }
                    };
                }
                Tile::Empty => {}
            }

            for light_beam in std::iter::once(light_beam).chain(second_light_beam) {
                let Some(next_position) = light_beam.next_position() else {
                    // The light beam has moved out from the left or top edge of the matrix.
                    continue;
                };
                light_beams.push(light_beam.with_position(next_position));
            }
        }

        energized.len()
    }

    /// Returns the number of energized tiles in the contraption for the light beam starting at the
    /// origin, facing right.
    fn energized_count_at_origin(&self) -> usize {
        self.energized_count(LightBeam::new(Position::zero(), CardinalDirection::Right))
    }

    /// Returns the maximum number of energized tiles in the contraption for a light beam starting
    /// at all the edges of the matrix, facing inwards.
    fn max_energized_count(&self) -> usize {
        let mut current_max = 0;
        let last = self.0.nrows() - 1;
        for index in 0..self.0.nrows() {
            current_max = current_max
                .max(self.energized_count(LightBeam::new(
                    Position::new(0, index),
                    CardinalDirection::Down,
                )))
                .max(self.energized_count(LightBeam::new(
                    Position::new(last, index),
                    CardinalDirection::Up,
                )))
                .max(self.energized_count(LightBeam::new(
                    Position::new(index, 0),
                    CardinalDirection::Right,
                )))
                .max(self.energized_count(LightBeam::new(
                    Position::new(index, last),
                    CardinalDirection::Left,
                )));
        }
        current_max
    }
}

impl FromStr for Contraption {
    type Err = Error;

    fn from_str(s: &str) -> Result<Contraption> {
        Ok(Contraption(SquareMatrix::try_from_iter(
            s.lines().count(),
            s.lines().flat_map(|line| line.bytes().map(Tile::try_from)),
        )?))
    }
}

impl fmt::Display for Contraption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

#[allow(dead_code)]
struct DisplayEnergizedContraption<'a> {
    energized: &'a HashSet<Position>,
    size: usize,
}

impl fmt::Display for DisplayEnergizedContraption<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.size {
            for c in 0..self.size {
                let position = Position::new(r, c);
                if self.energized.contains(&position) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let contraption = Contraption::from_str(input)?;

    println!("Part 1: {}", contraption.energized_count_at_origin());
    println!("Part 2: {}", contraption.max_energized_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn sample() {
        let contraption = Contraption::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(contraption.energized_count_at_origin(), 46);
        assert_eq!(contraption.max_energized_count(), 51);
    }
}

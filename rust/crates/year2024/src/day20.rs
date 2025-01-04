use std::fmt::{self, Write};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};
use aoc_lib::matrix::{CardinalDirection, Position, SquareMatrix};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Track,
    Wall,
    Start,
    End,
}

impl Tile {
    const fn is_track(self) -> bool {
        matches!(self, Tile::Track | Tile::Start | Tile::End)
    }
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Tile> {
        Ok(match value {
            b'.' => Tile::Track,
            b'#' => Tile::Wall,
            b'S' => Tile::Start,
            b'E' => Tile::End,
            _ => bail!("Invalid tile character: {}", value as char),
        })
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Track => f.write_char('.'),
            Tile::Wall => f.write_char('#'),
            Tile::Start => f.write_char('S'),
            Tile::End => f.write_char('E'),
        }
    }
}

#[derive(Debug)]
struct Racetrack(Vec<Position>);

impl Racetrack {
    /// Count the number of cheats that when applied for `duration` picoseconds would save at least
    /// 100 picoseconds.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is less than or equal to 1.
    fn cheat_count(&self, duration: usize) -> usize {
        assert!(duration > 1);

        let mut count = 0;

        for (start_duration, start_position) in self.iter().enumerate() {
            for (end_duration, end_position) in
                (start_duration + 1..self.len()).zip(self[start_duration + 1..].iter())
            {
                let distance = start_position.manhattan_distance(end_position);
                if distance <= 1 || distance > duration {
                    continue;
                }
                if end_duration - start_duration <= distance {
                    continue;
                }
                let saved = end_duration - start_duration - distance;
                if saved >= 100 {
                    count += 1;
                }
            }
        }

        count
    }
}

impl Deref for Racetrack {
    type Target = [Position];

    fn deref(&self) -> &[Position] {
        &self.0
    }
}

impl FromStr for Racetrack {
    type Err = Error;

    fn from_str(s: &str) -> Result<Racetrack> {
        let map = SquareMatrix::try_from_iter(
            s.lines().count(),
            s.lines().flat_map(|line| line.bytes().map(Tile::try_from)),
        )?;
        let start = map
            .find_position(&Tile::Start)
            .ok_or_else(|| anyhow!("Start position ({}) not found in the input", Tile::Start))?;

        let mut track = vec![start];
        let mut current_position = start;
        let mut direction = CardinalDirection::ALL
            .into_iter()
            .find(|&direction| map[start + direction].is_track())
            .expect("start position should have a neighboring track");

        while map[current_position] != Tile::End {
            (current_position, direction) =
                [direction, direction.turn_left(), direction.turn_right()]
                    .into_iter()
                    .find_map(|direction| {
                        let next_position = current_position + direction;
                        map[next_position]
                            .is_track()
                            .then_some((next_position, direction))
                    })
                    .expect("track should be continuous");
            track.push(current_position);
        }

        Ok(Racetrack(track))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let racetrack = Racetrack::from_str(input)?;

    println!("Part 1: {:?}", racetrack.cheat_count(2));
    println!("Part 2: {:?}", racetrack.cheat_count(20));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn sample() {
        let _racetrack = Racetrack::from_str(SAMPLE_INPUT).unwrap();
    }
}

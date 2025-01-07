use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use aoc_lib::matrix::Matrix;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Tile> {
        match value {
            b'.' => Ok(Tile::Ash),
            b'#' => Ok(Tile::Rock),
            _ => Err(anyhow!("Invalid tile: {}", value as char)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Ash => f.write_char('.'),
            Tile::Rock => f.write_char('#'),
        }
    }
}

#[derive(Debug)]
struct Pattern(Matrix<Tile>);

impl Pattern {
    fn summarize(&self, diff_count: usize) -> usize {
        // Check the rows.
        for candidate in 1..self.0.nrows() {
            let size = candidate.min(self.0.nrows() - candidate);
            if (candidate - size..candidate)
                .rev()
                .zip(candidate..candidate + size)
                .map(|(r1, r2)| {
                    self.0
                        .row(r1)
                        .iter()
                        .zip(&self.0.row(r2))
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>()
                == diff_count
            {
                return candidate * 100;
            }
        }

        // Check the columns.
        for candidate in 1..self.0.ncols() {
            let size = candidate.min(self.0.ncols() - candidate);
            if (candidate - size..candidate)
                .rev()
                .zip(candidate..candidate + size)
                .map(|(c1, c2)| {
                    self.0
                        .column(c1)
                        .iter()
                        .zip(&self.0.column(c2))
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>()
                == diff_count
            {
                return candidate;
            }
        }

        unreachable!("No mirror found")
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Pattern, Error> {
        Ok(Pattern(Matrix::try_from_iter(
            s.lines().count(),
            s.lines()
                .next()
                .ok_or_else(|| anyhow!("Empty input"))?
                .len(),
            s.lines().flat_map(|line| line.bytes().map(Tile::try_from)),
        )?))
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug)]
struct PatternNotes(Vec<Pattern>);

impl PatternNotes {
    fn summarize(&self, diff_count: usize) -> usize {
        self.0
            .iter()
            .map(|pattern| pattern.summarize(diff_count))
            .sum()
    }
}

impl FromStr for PatternNotes {
    type Err = Error;

    fn from_str(s: &str) -> Result<PatternNotes, Error> {
        Ok(PatternNotes(
            s.split("\n\n").map(str::parse).collect::<Result<_, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let notes = PatternNotes::from_str(input)?;

    println!("Part 1: {}", notes.summarize(0));
    println!("Part 2: {}", notes.summarize(1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn sample() {
        let notes = PatternNotes::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(notes.summarize(0), 405);
        assert_eq!(notes.summarize(1), 400);
    }
}

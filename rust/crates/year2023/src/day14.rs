use std::collections::HashMap;
use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{MatrixError, SquareMatrix};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RockShape {
    Round,
    Cube,
}

impl fmt::Display for RockShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RockShape::Round => f.write_char('O'),
            RockShape::Cube => f.write_char('#'),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Rock(RockShape),
    Empty,
}

impl TryFrom<u8> for Tile {
    type Error = MatrixError;

    fn try_from(value: u8) -> Result<Tile, MatrixError> {
        match value {
            b'O' => Ok(Tile::Rock(RockShape::Round)),
            b'#' => Ok(Tile::Rock(RockShape::Cube)),
            b'.' => Ok(Tile::Empty),
            _ => Err(MatrixError::InvalidCharacter(value as char)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Rock(shape) => shape.fmt(f),
            Tile::Empty => f.write_char('.'),
        }
    }
}

#[derive(Debug)]
struct Platform(SquareMatrix<Tile>);

impl Platform {
    /// Returns the total load of the platform after the first tilt.
    ///
    /// This returns the value without modifying the platform.
    fn first_load(&self) -> usize {
        let mut load = 0;
        let nrows = self.0.nrows();
        for column in self.0.column_iter() {
            let mut empty_index = 0;
            for (index, tile) in column.iter().enumerate() {
                match tile {
                    Tile::Rock(RockShape::Round) => {
                        load += nrows - empty_index;
                        empty_index += 1;
                    }
                    Tile::Rock(RockShape::Cube) => {
                        empty_index = index + 1;
                    }
                    Tile::Empty => {}
                }
            }
        }
        load
    }

    /// Tilt the platform to the north, moving all round rocks to the top of each column as much as
    /// possible.
    ///
    /// This will mutate the platform in-place.
    fn tilt_north(&mut self) {
        let mut swaps = Vec::new();
        for (col, column) in self.0.column_iter().enumerate() {
            let mut empty_row = 0;
            for (row, tile) in column.iter().enumerate() {
                match tile {
                    Tile::Rock(RockShape::Round) => {
                        if row != empty_row {
                            swaps.push(((row, col), (empty_row, col)));
                        }
                        empty_row += 1;
                    }
                    Tile::Rock(RockShape::Cube) => {
                        empty_row = row + 1;
                    }
                    Tile::Empty => {}
                }
            }
        }
        for (from, to) in swaps {
            self.0.swap(from, to);
        }
    }

    /// Run a spin cycle on the platform.
    ///
    /// This will tilt the platform four times in each direction starting from the north, then
    /// west, south, and east in that order. This will mutate the platform in-place.
    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt_north();
            self.0.rotate_mut();
        }
    }

    /// Run a number of spin cycles on the platform.
    ///
    /// This will mutate the platform in-place.
    fn n_cycles(&mut self, n: usize) {
        let mut cache = HashMap::new();
        let mut cycle = 0;
        while cycle < n {
            self.cycle();
            if let Some(prev_cycle) = cache.insert(self.0.clone(), cycle) {
                let length = cycle - prev_cycle;
                cycle += (n - cycle) / length * length;
            }
            cycle += 1;
        }
    }

    /// Returns the total load of the platform.
    fn total_load(&self) -> usize {
        self.0
            .enumerate()
            .filter_map(|(pos, tile)| {
                if matches!(tile, Tile::Rock(RockShape::Round)) {
                    Some(self.0.nrows() - pos.row())
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for Platform {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Platform, MatrixError> {
        SquareMatrix::try_from_rows(s.lines().map(|line| line.bytes().map(Tile::try_from)))
            .map(Platform)
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let platform = Platform::from_str(input)?;

    println!("Part 1: {}", platform.first_load());

    let mut platform = platform;
    platform.n_cycles(1_000_000_000);

    println!("Part 2: {}", platform.total_load());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn sample() {
        let platform = Platform::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(platform.first_load(), 136);
        let mut platform = platform;
        platform.n_cycles(1_000_000_000);
        assert_eq!(platform.total_load(), 64);
    }
}

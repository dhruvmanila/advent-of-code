use std::{fmt, str::FromStr};

use anyhow::{anyhow, Result};
use aoc_lib::matrix::{Direction, Matrix, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl TryFrom<u8> for Letter {
    type Error = anyhow::Error;

    fn try_from(byte: u8) -> Result<Self> {
        match byte {
            b'X' => Ok(Self::X),
            b'M' => Ok(Self::M),
            b'A' => Ok(Self::A),
            b'S' => Ok(Self::S),
            _ => Err(anyhow!("Unexpected character: {}", byte as char)),
        }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct Board(Matrix<Letter>);

impl Board {
    /// Return an iterator over the positions of the letter "X" on the board.
    fn x_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.0
            .enumerate()
            .filter_map(|(position, letter)| (*letter == Letter::X).then_some(position))
    }

    /// Return an iterator over the positions of the letter "A" on the board.
    fn a_positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.0
            .enumerate()
            .filter_map(|(position, letter)| (*letter == Letter::A).then_some(position))
    }

    /// Return the number of occurence of the word "XMAS" on the board in any direction
    /// (horizontal, vertical, diagonal).
    fn xmas_count(&self) -> usize {
        let mut count = 0;

        // For each position of the letter "X" on the board, check in all directions if the
        // following letters are "M", "A", "S" in order.
        for x_position in self.x_positions() {
            for direction in Direction::ALL {
                let mut positions_in_direction =
                    self.0.positions_in_direction(x_position, direction);
                if [Letter::M, Letter::A, Letter::S].iter().all(|letter| {
                    positions_in_direction.next().map_or(false, |position| {
                        self.0.get(position.row(), position.col()) == Some(letter)
                    })
                }) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Return the number of occurence of the two "MAS" words on the board in the shape of an "X".
    ///
    /// One way to achieve that is:
    /// ```txt
    /// M.S
    /// .A.
    /// M.S
    /// ```
    ///
    /// There are three other ways to achieve that.
    fn x_mas_count(&self) -> usize {
        /// The two pairs of directions that forms an "X" i.e., the opposite corners pairs.
        static DIRECTION_PAIR: [(Direction, Direction); 2] = [
            (Direction::TopLeft, Direction::BottomRight),
            (Direction::TopRight, Direction::BottomLeft),
        ];

        let mut count = 0;

        'outer: for a_position in self.a_positions() {
            for (direction1, direction2) in DIRECTION_PAIR {
                let Some(letter1) = a_position
                    .neighbor(direction1)
                    .and_then(|position| self.0.get(position.row(), position.col()))
                else {
                    continue 'outer;
                };

                let Some(letter2) = a_position
                    .neighbor(direction2)
                    .and_then(|position| self.0.get(position.row(), position.col()))
                else {
                    continue 'outer;
                };

                if !matches!(
                    (letter1, letter2),
                    (Letter::M, Letter::S) | (Letter::S, Letter::M)
                ) {
                    continue 'outer;
                }
            }

            // If all the conditions are met, increment the count.
            count += 1;
        }

        count
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(Matrix::try_from_iter(
            s.lines().count(),
            s.lines()
                .next()
                .ok_or_else(|| anyhow!("Expected at least one line in the input"))?
                .len(),
            s.lines()
                .flat_map(|line| line.bytes().map(Letter::try_from)),
        )?))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.rows() {
            for cell in row.iter() {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let board = Board::from_str(input)?;

    println!("Part 1: {:?}", board.xmas_count());
    println!("Part 2: {:?}", board.x_mas_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn sample() {
        let board = Board::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(board.xmas_count(), 18);
        assert_eq!(board.x_mas_count(), 9);
    }
}

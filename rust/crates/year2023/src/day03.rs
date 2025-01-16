use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{Direction, Matrix, MatrixError, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Digit(u8),
    Symbol(char),
}

impl CellType {
    /// Returns `true` if this cell is empty.
    const fn is_symbol(self) -> bool {
        matches!(self, CellType::Symbol(_))
    }

    /// Returns `true` if this cell is a gear.
    const fn is_gear(self) -> bool {
        matches!(self, CellType::Symbol('*'))
    }

    /// Returns `true` if this cell is a digit.
    const fn is_digit(self) -> bool {
        matches!(self, CellType::Digit(_))
    }

    /// Returns the digit value of this cell, if it is a digit.
    const fn as_digit(self) -> Option<u8> {
        match self {
            CellType::Digit(digit) => Some(digit),
            _ => None,
        }
    }
}

impl From<u8> for CellType {
    fn from(byte: u8) -> CellType {
        match byte {
            b'.' => CellType::Empty,
            byte if byte.is_ascii_digit() => CellType::Digit(byte - b'0'),
            byte => CellType::Symbol(byte as char),
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellType::Empty => f.write_char('.'),
            CellType::Digit(digit) => write!(f, "{digit}"),
            CellType::Symbol(symbol) => write!(f, "{symbol}"),
        }
    }
}

#[derive(Debug)]
struct EngineSchematic(Matrix<CellType>);

impl EngineSchematic {
    /// Returns the sum of the part numbers in the schematic.
    ///
    /// A part number is a number that has at least one symbol in one of its neighboring cells.
    fn sum_part_numbers(&self) -> u32 {
        self.0
            .enumerate()
            .filter_map(|(position, cell)| cell.is_symbol().then_some(position))
            .flat_map(|symbol_position| self.surrounding_numbers(symbol_position).into_iter())
            .sum()
    }

    /// Returns the sum of the gear ratios in the schematic.
    ///
    /// A gear ratio is the product of the two numbers that are in the cells
    /// surrounding a gear. This is only valid if there are exactly two numbers.
    fn sum_gear_ratios(&self) -> u32 {
        self.0
            .enumerate()
            .filter_map(|(position, cell)| {
                if cell.is_gear() {
                    match self.surrounding_numbers(position).as_slice() {
                        [a, b] => Some(a * b),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    /// Extracts the number at the given position.
    ///
    /// This method assumes that the value at the given `position` is a digit.
    fn extract_number_at(&self, position: Position) -> u32 {
        // Get the start position of the number by going left until we find a non-digit or we reach
        // the left edge of the matrix.
        let start_position = self
            .0
            .positions_in_direction(position, Direction::Left)
            .take_while(|&pos| self.0.get(pos).is_some_and(|cell| cell.is_digit()))
            .last()
            .unwrap_or(position);

        // Extract the number by going right until we find a non-digit or we reach the right edge
        // of the matrix.
        std::iter::once(start_position)
            .chain(
                self.0
                    .positions_in_direction(start_position, Direction::Right),
            )
            .map_while(|pos| self.0.get(pos).and_then(|cell| cell.as_digit()))
            .fold(0, |number, digit| number * 10 + u32::from(digit))
    }

    /// Returns the numbers surrounding the given position.
    fn surrounding_numbers(&self, position: Position) -> Vec<u32> {
        let mut numbers = Vec::new();

        // We only need to perform the check for the top and bottom rows.
        // The middle row cannot contain any case of two consecutive digits.
        let mut checked_rows: HashSet<usize> = HashSet::with_capacity(2);

        for neighbor in position.neighbors() {
            let Some(cell) = self.0.get(neighbor) else {
                continue;
            };
            if cell.is_digit() {
                if position.row() == neighbor.row() || checked_rows.insert(neighbor.row()) {
                    numbers.push(self.extract_number_at(neighbor));
                }
            } else {
                checked_rows.remove(&neighbor.row());
            }
        }

        numbers
    }
}

impl FromStr for EngineSchematic {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<EngineSchematic, MatrixError> {
        Matrix::from_rows(s.lines().map(|line| line.bytes().map(CellType::from)))
            .map(EngineSchematic)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let schematic = EngineSchematic::from_str(input)?;

    println!("Part 1: {}", schematic.sum_part_numbers());
    println!("Part 2: {}", schematic.sum_gear_ratios());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn sample() {
        let schematic = EngineSchematic::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(schematic.sum_part_numbers(), 4361);
        assert_eq!(schematic.sum_gear_ratios(), 467_835);
    }
}

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use anyhow::Result;

use aoc_lib::matrix::{Matrix, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    Empty,
    Digit(u8),
    Symbol(char),
}

impl CellType {
    /// Returns `true` if this cell is empty.
    const fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    /// Returns `true` if this cell is a gear.
    const fn is_gear(self) -> bool {
        matches!(self, Self::Symbol('*'))
    }

    /// Returns `true` if this cell is a digit.
    const fn is_digit(self) -> bool {
        matches!(self, Self::Digit(_))
    }

    /// Returns the digit value of this cell, if it is a digit.
    const fn as_digit(self) -> Option<u8> {
        match self {
            Self::Digit(digit) => Some(digit),
            _ => None,
        }
    }
}

impl From<u8> for CellType {
    fn from(byte: u8) -> Self {
        match byte {
            b'.' => Self::Empty,
            byte if byte.is_ascii_digit() => Self::Digit(byte - b'0'),
            byte => Self::Symbol(byte as char),
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("."),
            Self::Digit(digit) => f.write_str(&digit.to_string()),
            Self::Symbol(symbol) => f.write_str(&symbol.to_string()),
        }
    }
}

#[derive(Debug)]
struct EngineSchematic(Matrix<CellType>);

impl EngineSchematic {
    /// Returns an iterator over the position of all the symbols in the schematic.
    fn symbols(&self) -> impl Iterator<Item = Position> + '_ {
        self.0
            .enumerate()
            .filter_map(|(position, cell)| cell.is_symbol().then_some(position))
    }

    /// Returns an iterator over the position of all the gears in the schematic.
    fn gears(&self) -> impl Iterator<Item = Position> + '_ {
        self.0
            .enumerate()
            .filter_map(|(position, cell)| cell.is_gear().then_some(position))
    }

    /// Extracts the number at the given position.
    fn extract_number_at(&self, position: Position) -> u32 {
        // Get the start position of the number by going left until we find a non-digit
        // or we reach the left edge of the matrix.
        let start = (0..position.col())
            .rev()
            .take_while(|&col| {
                self.0
                    .get(position.row(), col)
                    .map_or(false, |cell| cell.is_digit())
            })
            .last()
            .unwrap_or(position.col());

        (start..self.0.ncols())
            .map_while(|col| {
                self.0
                    .get(position.row(), col)
                    .and_then(|cell| cell.as_digit())
            })
            .fold(0, |number, digit| number * 10 + u32::from(digit))
    }

    fn surrounding_numbers(&self, position: Position) -> Vec<u32> {
        let mut numbers = Vec::new();

        // We only need to perform the check for the top and bottom rows.
        // The middle row cannot contain any case of two consecutive digits.
        let mut checked_rows: HashSet<usize> = HashSet::with_capacity(2);

        for neighbor in position.neighbors() {
            let Some(cell) = self.0.get(neighbor.row(), neighbor.col()) else {
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

    /// Returns an iterator over the part numbers in the schematic.
    ///
    /// A part number is a number that has at least one symbol in one of its
    /// neighboring cells.
    fn part_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.symbols()
            .flat_map(|symbol_position| self.surrounding_numbers(symbol_position).into_iter())
    }

    /// Returns an iterator over the gear ratios in the schematic.
    ///
    /// A gear ratio is the product of the two numbers that are in the cells
    /// surrounding a gear. This is only valid if there are exactly two numbers.
    fn gear_ratios(&self) -> impl Iterator<Item = u32> + '_ {
        self.gears().filter_map(|gear_position| {
            match self.surrounding_numbers(gear_position).as_slice() {
                [a, b] => Some(a * b),
                _ => None,
            }
        })
    }
}

impl FromStr for EngineSchematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(Matrix::from_iter(
            s.lines().count(),
            s.lines()
                .next()
                .ok_or_else(|| anyhow::anyhow!("Expected at least one line in input"))?
                .len(),
            s.lines().flat_map(|line| line.bytes().map(CellType::from)),
        )))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let schematic = input.parse::<EngineSchematic>()?;

    println!("Part 1: {}", schematic.part_numbers().sum::<u32>());
    println!("Part 2: {}", schematic.gear_ratios().sum::<u32>());

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
    fn test_sample_part_1() -> Result<()> {
        let schematic = SAMPLE_INPUT.parse::<EngineSchematic>()?;
        let mut part_numbers = schematic.part_numbers().collect::<Vec<_>>();
        part_numbers.sort_unstable();

        assert_eq!(part_numbers, &[35, 467, 592, 598, 617, 633, 664, 755]);
        assert_eq!(part_numbers.iter().sum::<u32>(), 4361);

        Ok(())
    }

    #[test]
    fn test_sample_part_2() -> Result<()> {
        let schematic = SAMPLE_INPUT.parse::<EngineSchematic>()?;
        let mut part_numbers = schematic.gear_ratios().collect::<Vec<_>>();
        part_numbers.sort_unstable();

        assert_eq!(part_numbers, &[16345, 451_490]);
        assert_eq!(part_numbers.iter().sum::<u32>(), 467_835);

        Ok(())
    }
}

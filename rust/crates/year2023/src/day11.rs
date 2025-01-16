use std::fmt::{self, Write};
use std::num::NonZeroUsize;
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{MatrixError, Position, SquareMatrix};
use itertools::Itertools;

/// Cell types for each position in the universe matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellType {
    EmptySpace,
    Galaxy,
}

impl CellType {
    /// Returns `true` if the cell type is empty space.
    const fn is_empty_space(self) -> bool {
        matches!(self, CellType::EmptySpace)
    }

    /// Returns `true` if the cell type is a galaxy.
    const fn is_galaxy(self) -> bool {
        matches!(self, CellType::Galaxy)
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellType::EmptySpace => f.write_char('.'),
            CellType::Galaxy => f.write_char('#'),
        }
    }
}

impl TryFrom<u8> for CellType {
    type Error = MatrixError;

    fn try_from(value: u8) -> Result<CellType, MatrixError> {
        match value {
            b'.' => Ok(CellType::EmptySpace),
            b'#' => Ok(CellType::Galaxy),
            _ => Err(MatrixError::InvalidCharacter(value as char)),
        }
    }
}

/// The universe.
#[derive(Debug)]
struct Universe(SquareMatrix<CellType>);

impl Universe {
    /// Expands the universe by the given size.
    fn expand_by(&self, size: NonZeroUsize) -> ExpandedUniverse {
        let mut galaxies = self
            .0
            .enumerate()
            .filter_map(|(pos, cell)| cell.is_galaxy().then_some(pos))
            .collect::<Vec<_>>();

        // Subtract 1 from the size to avoid counting the empty row / column itself.
        let size = size.get() - 1;
        if size == 0 {
            return ExpandedUniverse {
                rows: self.0.nrows(),
                cols: self.0.ncols(),
                galaxies,
            };
        }

        let mut empty_rows = 0;
        for (idx, row) in self.0.row_iter().enumerate() {
            if !row.iter().all(|cell| cell.is_empty_space()) {
                continue;
            }
            let boundary = idx + empty_rows;
            for galaxy_position in galaxies.iter_mut().filter(|pos| pos.row() > boundary) {
                *galaxy_position = galaxy_position.add_row(size);
            }
            empty_rows += size;
        }

        let mut empty_columns = 0;
        for (idx, column) in self.0.column_iter().enumerate() {
            if !column.iter().all(|cell| cell.is_empty_space()) {
                continue;
            }
            let boundary = idx + empty_columns;
            for galaxy_position in galaxies.iter_mut().filter(|pos| pos.col() > boundary) {
                *galaxy_position = galaxy_position.add_col(size);
            }
            empty_columns += size;
        }

        ExpandedUniverse {
            rows: self.0.nrows() + empty_rows,
            cols: self.0.ncols() + empty_columns,
            galaxies,
        }
    }
}

impl FromStr for Universe {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Universe, MatrixError> {
        SquareMatrix::try_from_rows(s.lines().map(|line| line.bytes().map(CellType::try_from)))
            .map(Universe)
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

/// Represents an expanded universe.
#[derive(Debug)]
struct ExpandedUniverse {
    rows: usize,
    cols: usize,
    galaxies: Vec<Position>,
}

impl ExpandedUniverse {
    /// Returns an iterator over the shortest paths between each unique pair of
    /// galaxies in the expanded universe.
    fn shortest_paths(&self) -> impl Iterator<Item = usize> + '_ {
        self.galaxies
            .iter()
            .combinations_with_replacement(2)
            .map(|combination| {
                let [galaxy1, galaxy2] = combination.as_slice() else {
                    unreachable!("There should always be two galaxies in a combination");
                };
                galaxy1.manhattan_distance(galaxy2)
            })
    }
}

impl fmt::Display for ExpandedUniverse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = if self.galaxies.contains(&Position::new(row, col)) {
                    CellType::Galaxy
                } else {
                    CellType::EmptySpace
                };
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let universe = Universe::from_str(input)?;

    println!(
        "Part 1: {}",
        universe
            .expand_by(NonZeroUsize::try_from(2)?)
            .shortest_paths()
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        universe
            .expand_by(NonZeroUsize::try_from(1_000_000)?)
            .shortest_paths()
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test_case(2, 374)]
    #[test_case(10, 1030)]
    #[test_case(100, 8410)]
    fn sample(expand_size: usize, expected: usize) {
        let universe = Universe::from_str(SAMPLE_INPUT).unwrap();

        assert_eq!(
            universe
                .expand_by(NonZeroUsize::try_from(expand_size).unwrap())
                .shortest_paths()
                .sum::<usize>(),
            expected
        );
    }
}

use std::{borrow::Cow, str::FromStr};

use anyhow::Result;
use aoc_lib::matrix::{Matrix, MatrixError, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    PaperRoll,
}

impl TryFrom<u8> for Tile {
    type Error = MatrixError;

    fn try_from(value: u8) -> Result<Tile, MatrixError> {
        match value {
            b'.' => Ok(Tile::Empty),
            b'@' => Ok(Tile::PaperRoll),
            _ => Err(MatrixError::InvalidCharacter(value as char)),
        }
    }
}

#[derive(Debug)]
struct Grid(Matrix<Tile>);

impl Grid {
    /// Return `true` if the paper roll at the given position is forklift-accessible.
    ///
    /// This method assumes that there _is_ a paper roll at the given position.
    fn is_forklift_accessible(&self, position: Position) -> bool {
        position
            .neighbors()
            .filter(|&neighbor| self.0.get(neighbor) == Some(&Tile::PaperRoll))
            .count()
            < 4
    }

    /// Return the number of forklift-accessible paper rolls.
    fn forklift_accessible_count(&self) -> usize {
        self.0
            .enumerate()
            .filter(|&(position, &tile)| {
                tile == Tile::PaperRoll && self.is_forklift_accessible(position)
            })
            .count()
    }

    /// Return the total number of accessible paper rolls that will be removed by iteratively
    /// removing forklift-accessible paper rolls until no more can be removed.
    fn remove_accessible_paper_rolls(&self) -> usize {
        let mut total_removed = 0;
        let mut grid = Cow::Borrowed(&self.0);

        loop {
            let mut removed = 0;
            let new_grid = grid.map_with_position(|position, &tile| {
                if tile == Tile::PaperRoll && self.is_forklift_accessible(position) {
                    removed += 1;
                    Tile::Empty
                } else {
                    tile
                }
            });
            if removed == 0 {
                break;
            }
            total_removed += removed;
            grid = Cow::Owned(new_grid);
        }

        total_removed
    }
}

impl FromStr for Grid {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Grid, MatrixError> {
        Matrix::try_from_rows(s.lines().map(|line| line.bytes().map(Tile::try_from))).map(Grid)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let grid = Grid::from_str(input)?;

    println!("Part 1: {}", grid.forklift_accessible_count());
    println!("Part 2: {}", grid.remove_accessible_paper_rolls());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn sample() {
        let grid = Grid::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(grid.forklift_accessible_count(), 13);
        assert_eq!(grid.remove_accessible_paper_rolls(), 43);
    }
}

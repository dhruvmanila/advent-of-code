use std::collections::HashSet;
use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{MatrixError, Position, SquareMatrix};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Start,
    GardenPlot,
    Rock,
}

impl Tile {
    const fn is_garden_plot(self) -> bool {
        matches!(self, Tile::GardenPlot | Tile::Start)
    }
}

impl TryFrom<u8> for Tile {
    type Error = MatrixError;

    fn try_from(value: u8) -> Result<Tile, MatrixError> {
        match value {
            b'S' => Ok(Tile::Start),
            b'.' => Ok(Tile::GardenPlot),
            b'#' => Ok(Tile::Rock),
            _ => Err(MatrixError::InvalidCharacter(value as char)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Start => f.write_char('S'),
            Tile::GardenPlot => f.write_char('.'),
            Tile::Rock => f.write_char('#'),
        }
    }
}

#[derive(Debug)]
struct Farm(SquareMatrix<Tile>);

impl Farm {
    /// Return the number of garden plots that can be reached from the start position in `n` steps.
    fn reachable_count(&self, n: u32) -> usize {
        let start = self
            .0
            .find_position(&Tile::Start)
            .expect("Start position not found in the farm");

        let mut reachable = HashSet::new();
        reachable.insert(start);

        for _ in 0..n {
            let mut next_reachable = HashSet::new();
            for position in reachable.drain() {
                for neighbor in position.cardinal_neighbors() {
                    if self
                        .0
                        .get(neighbor)
                        .is_some_and(|tile| tile.is_garden_plot())
                    {
                        next_reachable.insert(neighbor);
                    }
                }
            }
            reachable = next_reachable;
        }

        reachable.len()
    }

    //fn reachable_count_in_infinite_grid(&self) -> i64 {
    //    let a0 = self.reachable_count(65) as f64;
    //    let a1 = self.reachable_count(65 + 131) as f64;
    //    let a2 = self.reachable_count(65 + 131 + 131) as f64;
    //
    //    println!("a0: {}, a1: {}, a2: {}", a0, a1, a2);
    //
    //    // Define the Vandermonde matrix
    //    let vandermonde = nalgebra::Matrix3::new(
    //        0.0, 0.0, 1.0, // row 1
    //        1.0, 1.0, 1.0, // row 2
    //        4.0, 2.0, 1.0, // row 3
    //    );
    //
    //    // Define the vector b
    //    let b = nalgebra::Vector3::new(a0, a1, a2);
    //
    //    // Solve the linear system vandermonde * x = b
    //    if let Some(solution) = vandermonde.lu().solve(&b) {
    //        let solution = solution.map(|val| val as i64);
    //        let n = 202_300;
    //        solution[0] * n * n + solution[1] * n + solution[2]
    //    } else {
    //        panic!("The system cannot be solved (singular matrix).");
    //    }
    //}
}

impl FromStr for Farm {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Farm, MatrixError> {
        SquareMatrix::try_from_rows(s.lines().map(|line| line.bytes().map(Tile::try_from)))
            .map(Farm)
    }
}

impl fmt::Display for Farm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.0, f)
    }
}

#[allow(dead_code)]
struct DisplayFarmWithReachable<'a> {
    farm: &'a Farm,
    reachable: &'a HashSet<Position>,
}

impl fmt::Display for DisplayFarmWithReachable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (r, row) in self.farm.0.row_iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if tile.is_garden_plot() && self.reachable.contains(&Position::new(r, c)) {
                    f.write_char('O')?;
                } else {
                    fmt::Display::fmt(tile, f)?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> Result<()> {
    let farm = Farm::from_str(input)?;

    println!("Part 1: {}", farm.reachable_count(64));
    //let part2 = farm.reachable_count_in_infinite_grid();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn sample() {
        let farm = Farm::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(farm.reachable_count(6), 16);
    }
}

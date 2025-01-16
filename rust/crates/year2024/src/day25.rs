use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{Matrix, MatrixError};

const MAX_PIN_HEIGHT: usize = 7;

struct PinHeights(Vec<usize>);

impl PinHeights {
    /// Check if the pins can fit together.
    fn fits(&self, other: &PinHeights) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(&a, &b)| a + b + 2 <= MAX_PIN_HEIGHT)
    }
}

impl fmt::Debug for PinHeights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = f.debug_tuple("PinHeights");
        for height in &self.0 {
            result.field(height);
        }
        result.finish()
    }
}

#[derive(Debug)]
struct Schematics {
    keys: Vec<PinHeights>,
    locks: Vec<PinHeights>,
}

impl Schematics {
    /// Count the number of (key, lock) pairs that can fit together.
    fn fit_count(&self) -> usize {
        self.keys
            .iter()
            .map(|key| self.locks.iter().filter(|lock| key.fits(lock)).count())
            .sum()
    }
}

impl FromStr for Schematics {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Schematics, MatrixError> {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        for section in s.split("\n\n") {
            let layout = Matrix::from_rows(section.lines().map(str::chars))?;

            // SAFETY: The layout is guaranteed to be 7x5
            let is_lock = layout.row(0).iter().all(|&c| c == '#');

            let mut heights = Vec::new();
            for column in layout.column_iter() {
                let height = if is_lock {
                    column.iter().skip(1).filter(|&&c| c == '#').count()
                } else {
                    let dots = column.iter().filter(|&&c| c == '.').count();
                    layout.nrows() - dots - 1
                };
                heights.push(height);
            }
            if is_lock {
                locks.push(PinHeights(heights));
            } else {
                keys.push(PinHeights(heights));
            }
        }

        Ok(Schematics { keys, locks })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let schematics = Schematics::from_str(input)?;

    println!("Part 1: {}", schematics.fit_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn sample() {
        let schematics = Schematics::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(schematics.fit_count(), 3);
    }
}

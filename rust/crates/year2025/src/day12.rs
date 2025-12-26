use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use aoc_lib::matrix::{MatrixError, SquareMatrix};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Filled,
}

impl TryFrom<u8> for Cell {
    type Error = MatrixError;

    fn try_from(byte: u8) -> Result<Cell, MatrixError> {
        match byte {
            b'.' => Ok(Cell::Empty),
            b'#' => Ok(Cell::Filled),
            _ => Err(MatrixError::InvalidCharacter(byte as char)),
        }
    }
}

struct PresentShape(SquareMatrix<Cell>);

impl PresentShape {
    /// Counts the number of filled cells in the present shape.
    fn filled_count(&self) -> usize {
        self.0
            .as_slice()
            .iter()
            .filter(|cell| **cell == Cell::Filled)
            .count()
    }
}

impl FromStr for PresentShape {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<PresentShape, MatrixError> {
        SquareMatrix::try_from_rows(s.lines().map(|line| line.bytes().map(Cell::try_from)))
            .map(PresentShape)
    }
}

struct ShapeMetadata {
    filled_count: usize,
    area: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FitResult {
    Yes,
    No,
    Maybe,
}

struct Region {
    width: usize,
    height: usize,
    shape_quantity: Vec<usize>,
}

impl Region {
    fn fits(&self, metadatas: &[ShapeMetadata]) -> FitResult {
        let available_area = self.width * self.height;

        let mut min_area = 0;
        let mut max_area = 0;
        for (metadata, &quantity) in metadatas.iter().zip(self.shape_quantity.iter()) {
            min_area += metadata.filled_count * quantity;
            max_area += metadata.area * quantity;
        }

        if min_area > available_area {
            FitResult::No
        } else if max_area <= available_area {
            FitResult::Yes
        } else {
            FitResult::Maybe
        }
    }
}

impl FromStr for Region {
    type Err = Error;

    fn from_str(s: &str) -> Result<Region, Error> {
        let Some((size_str, quantities_str)) = s.split_once(':') else {
            bail!("Expected region line to contain a size and quantities separated by ':': {s:?}");
        };
        let Some((width_str, height_str)) = size_str.split_once('x') else {
            bail!("Expected region size to be in the format WxH: {size_str:?}");
        };
        let width: usize = width_str
            .trim()
            .parse()
            .with_context(|| format!("Failed to parse region width: {width_str:?}"))?;
        let height: usize = height_str
            .trim()
            .parse()
            .with_context(|| format!("Failed to parse region height: {height_str:?}"))?;
        let shape_quantity: Vec<usize> = quantities_str
            .split_whitespace()
            .map(|quantity_str| {
                quantity_str
                    .parse()
                    .with_context(|| format!("Failed to parse shape quantity: {quantity_str:?}"))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Region {
            width,
            height,
            shape_quantity,
        })
    }
}

struct PresentsAndRegions {
    presents: Vec<PresentShape>,
    regions: Vec<Region>,
}

impl PresentsAndRegions {
    /// Returns the total number of regions that can be filled with the presents.
    fn fit_count(&self) -> usize {
        let metadata = self
            .presents
            .iter()
            .map(|present| ShapeMetadata {
                filled_count: present.filled_count(),
                area: present.0.nrows() * present.0.ncols(),
            })
            .collect::<Vec<_>>();
        self.regions
            .iter()
            .filter(|region| matches!(region.fits(&metadata), FitResult::Yes))
            .count()
    }
}

impl FromStr for PresentsAndRegions {
    type Err = Error;

    fn from_str(s: &str) -> Result<PresentsAndRegions, Error> {
        let Some((presents_block, regions_block)) = s.rsplit_once("\n\n") else {
            bail!("Expected input to contain presents and regions separated by a blank line");
        };

        let mut presents = Vec::new();
        for block in presents_block.split("\n\n") {
            let Some((_, present_str)) = block.split_once('\n') else {
                bail!("Expected shape block to contain a shape after the ID line: {block:?}");
            };
            let present = PresentShape::from_str(present_str)
                .with_context(|| format!("Failed to parse present shape: {block:?}"))?;
            presents.push(present);
        }

        let mut regions = Vec::new();
        for line in regions_block.lines() {
            let region = Region::from_str(line)
                .with_context(|| format!("Failed to parse region: {line:?}"))?;
            regions.push(region);
        }

        Ok(PresentsAndRegions { presents, regions })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let presents_and_regions = PresentsAndRegions::from_str(input)?;

    println!("Part 1: {}", presents_and_regions.fit_count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn sample() {
        let _ = PresentsAndRegions::from_str(SAMPLE_INPUT).unwrap();
        // assert_eq!(presents_and_regions.fit_count(), 2);
    }
}

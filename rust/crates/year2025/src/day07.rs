use std::{
    collections::{HashMap, HashSet},
    fmt,
    str::FromStr,
};

use anyhow::{Result, anyhow, bail};
use aoc_lib::matrix::{Direction, Matrix, MatrixError, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Splitter,
    TachyonBeam,
}

impl TryFrom<u8> for Tile {
    type Error = MatrixError;

    fn try_from(byte: u8) -> Result<Tile, MatrixError> {
        match byte {
            b'.' => Ok(Tile::Empty),
            b'^' => Ok(Tile::Splitter),
            b'S' => Ok(Tile::TachyonBeam),
            _ => Err(MatrixError::InvalidCharacter(byte as char)),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_str("."),
            Tile::Splitter => f.write_str("^"),
            Tile::TachyonBeam => f.write_str("S"),
        }
    }
}

#[derive(Debug)]
struct Manifold(Matrix<Tile>);

impl Manifold {
    fn split_count(&self) -> Result<usize> {
        let mut splitters = HashSet::new();
        let mut start_positions = vec![
            self.0
                .find_position(&Tile::TachyonBeam)
                .ok_or_else(|| anyhow!("No tachyon beam found in the manifold"))?,
        ];

        while let Some(start_position) = start_positions.pop() {
            for next_position in self
                .0
                .positions_in_direction(start_position, Direction::Down)
            {
                match self.0[next_position] {
                    Tile::Empty => {} // continue moving down
                    Tile::Splitter => {
                        if !splitters.insert(next_position) {
                            break; // this splitter has already been processed
                        }
                        for direction in [Direction::Left, Direction::Right] {
                            if let Some(next_start_position) =
                                next_position.checked_neighbor(direction)
                            {
                                start_positions.push(next_start_position);
                            }
                        }
                        break;
                    }
                    Tile::TachyonBeam => bail!(
                        "Unexpected tachyon beam encountered at {next_position:?} starting from {start_position:?}"
                    ),
                }
            }
        }

        Ok(splitters.len())
    }

    fn timeline_count(&self) -> Result<usize> {
        fn inner(
            manifold: &Matrix<Tile>,
            start_position: Position,
            cache: &mut HashMap<Position, usize>,
        ) -> Result<usize> {
            if let Some(&count) = cache.get(&start_position) {
                return Ok(count);
            }

            let mut count = 0;
            let mut next_positions =
                manifold.positions_in_direction(start_position, Direction::Down);
            loop {
                let Some(next_position) = next_positions.next() else {
                    // Reached the bottom of the manifold, count this as one timeline.
                    count += 1;
                    break;
                };
                match manifold[next_position] {
                    Tile::Empty => {} // continue moving down
                    Tile::Splitter => {
                        for direction in [Direction::Left, Direction::Right] {
                            if let Some(next_start_position) =
                                next_position.checked_neighbor(direction)
                            {
                                count += inner(manifold, next_start_position, cache)?;
                            }
                        }
                        break;
                    }
                    Tile::TachyonBeam => bail!(
                        "Unexpected tachyon beam encountered at {next_position:?} starting from {start_position:?}"
                    ),
                }
            }

            cache.insert(start_position, count);
            Ok(count)
        }

        let start_position = self
            .0
            .find_position(&Tile::TachyonBeam)
            .ok_or_else(|| anyhow!("No tachyon beam found in the manifold"))?;

        inner(&self.0, start_position, &mut HashMap::new())
    }
}

impl FromStr for Manifold {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<Manifold, MatrixError> {
        Matrix::try_from_rows(s.lines().map(|line| line.bytes().map(Tile::try_from))).map(Manifold)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let manifold = Manifold::from_str(input)?;

    println!("Part 1: {}", manifold.split_count()?);
    println!("Part 2: {}", manifold.timeline_count()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn sample() {
        let manifold = Manifold::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(manifold.split_count().unwrap(), 21);
        assert_eq!(manifold.timeline_count().unwrap(), 40);
    }
}

/*

.......S.......  .......S.......  .......S.......  .......S.......
.......|.......  .......|.......  .......|.......  .......|.......
......|^.......  ......|^.......  ......|^.......  ......|^.......
......|........  ......|........  ......|........  ......|........
.....|^.^......  .....|^.^......  ......^|^......  ......^|^......
.....|.........  .....|.........  .......|.......  .......|.......
....|^.^.^.....  .....^|^.^.....  .....^|^.^.....  .....^.^|^.....
....|..........  ......|........  ......|........  ........|......

.......S.......  .......S.......  .......S.......  .......S.......
.......|.......  .......|.......  .......|.......  .......|.......
.......^|......  .......^|......  .......^|......  .......^|......
........|......  ........|......  ........|......  ........|......
......^|^......  ......^|^......  ......^.^|.....  ......^.^|.....
.......|.......  .......|.......  .........|.....  .........|.....
.....^|^.^.....  .....^.^|^.....  .....^.^|^.....  .....^.^.^|....
......|........  ........|......  ........|......  ..........|....

*/

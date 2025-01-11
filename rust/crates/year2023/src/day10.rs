use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};
use aoc_lib::matrix::{CardinalDirection, Matrix, Position};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Connection {
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndEast,
    SouthAndWest,
}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Connection::Vertical => '│',
            Connection::Horizontal => '─',
            Connection::NorthAndEast => '└',
            Connection::NorthAndWest => '┘',
            Connection::SouthAndEast => '┌',
            Connection::SouthAndWest => '┐',
        };
        f.write_char(c)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Pipe(Connection),
    Ground,
    Animal,
}

impl Tile {
    /// Returns `true` if this is a corner pipe.
    const fn is_corner_pipe(self) -> bool {
        matches!(
            self,
            Tile::Pipe(
                Connection::SouthAndEast
                    | Connection::SouthAndWest
                    | Connection::NorthAndEast
                    | Connection::NorthAndWest
            )
        )
    }

    /// Returns `true` if the animal is connected to the other `tile` in the given `direction`.
    const fn is_animal_connected_to(other: Tile, direction: CardinalDirection) -> bool {
        match direction {
            CardinalDirection::Up => matches!(
                other,
                Tile::Pipe(
                    Connection::Vertical | Connection::SouthAndWest | Connection::SouthAndEast
                )
            ),
            CardinalDirection::Right => matches!(
                other,
                Tile::Pipe(
                    Connection::Horizontal | Connection::NorthAndWest | Connection::SouthAndWest
                )
            ),
            CardinalDirection::Down => matches!(
                other,
                Tile::Pipe(
                    Connection::Vertical | Connection::NorthAndEast | Connection::NorthAndWest
                )
            ),
            CardinalDirection::Left => matches!(
                other,
                Tile::Pipe(
                    Connection::Horizontal | Connection::NorthAndEast | Connection::SouthAndEast
                )
            ),
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = Error;

    fn try_from(value: u8) -> Result<Tile> {
        match value {
            b'|' => Ok(Tile::Pipe(Connection::Vertical)),
            b'-' => Ok(Tile::Pipe(Connection::Horizontal)),
            b'L' => Ok(Tile::Pipe(Connection::NorthAndEast)),
            b'J' => Ok(Tile::Pipe(Connection::NorthAndWest)),
            b'7' => Ok(Tile::Pipe(Connection::SouthAndWest)),
            b'F' => Ok(Tile::Pipe(Connection::SouthAndEast)),
            b'.' => Ok(Tile::Ground),
            b'S' => Ok(Tile::Animal),
            _ => bail!("Unexpected tile: {}", value as char),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Pipe(connection) => write!(f, "{connection}"),
            Tile::Ground => f.write_char('.'),
            Tile::Animal => f.write_char('S'),
        }
    }
}

#[derive(Debug)]
struct Grid(Matrix<Tile>);

impl Grid {
    /// Returns a tuple of two values where:
    /// 1. Number of steps it takes to reach the farthest point starting from animal
    /// 2. Number of enclosed tiles in the loop formed by the path of the animal
    ///
    /// This utilizes the [Pick's theorem] to calculate the total number of interior points in the
    /// polygon formed by the loop and uses the [Shoelace formula] to calculate the area of the
    /// polygon.
    ///
    /// [Pick's theorem]: https://en.wikipedia.org/wiki/Pick%27s_theorem
    /// [Shoelace formula]: https://en.wikipedia.org/wiki/Shoelace_formula
    fn analyze_loop(&self) -> Result<(usize, i128)> {
        let animal_position = self
            .0
            .find_position(&Tile::Animal)
            .ok_or_else(|| anyhow!("Animal position not found on the grid"))?;

        // Find the next position and direction to move to starting from the animal position.
        let (mut position, mut direction) = CardinalDirection::ALL
            .into_iter()
            .find_map(|direction| {
                let neighbor = animal_position.checked_neighbor(direction.into())?;
                Tile::is_animal_connected_to(*self.0.get(neighbor)?, direction)
                    .then_some((neighbor, direction))
            })
            .ok_or_else(|| anyhow!("No connection found for the animal at {animal_position:?}"))?;

        // Initiate the steps with 1 as we already moved one step forward from the start position.
        let mut steps = 1usize;
        // The area of the polygon formed by the loop.
        let mut area = 0;
        // The animal enters the loop from one of the corners, so this position is should be a
        // corner position.
        let mut previous_corner = animal_position;

        loop {
            let current_tile = self.0[position];

            match current_tile {
                Tile::Pipe(Connection::Vertical | Connection::Horizontal) => {
                    // Keep moving ahead in the straight direction until we reach a corner.
                }
                Tile::Pipe(Connection::NorthAndEast) => {
                    direction = match direction {
                        CardinalDirection::Down => CardinalDirection::Right,
                        CardinalDirection::Left => CardinalDirection::Up,
                        _ => {
                            bail!("Unexpected NorthAndEast connection when facing {direction:?}")
                        }
                    };
                }
                Tile::Pipe(Connection::NorthAndWest) => {
                    direction = match direction {
                        CardinalDirection::Down => CardinalDirection::Left,
                        CardinalDirection::Right => CardinalDirection::Up,
                        _ => {
                            bail!("Unexpected NorthAndWest connection when facing {direction:?}")
                        }
                    };
                }
                Tile::Pipe(Connection::SouthAndWest) => {
                    direction = match direction {
                        CardinalDirection::Up => CardinalDirection::Left,
                        CardinalDirection::Right => CardinalDirection::Down,
                        _ => {
                            bail!("Unexpected SouthAndWest connection when facing {direction:?}")
                        }
                    };
                }
                Tile::Pipe(Connection::SouthAndEast) => {
                    direction = match direction {
                        CardinalDirection::Up => CardinalDirection::Right,
                        CardinalDirection::Left => CardinalDirection::Down,
                        _ => {
                            bail!("Unexpected SouthAndEast connection when facing {direction:?}")
                        }
                    };
                }
                Tile::Ground => bail!("Unexpected ground position hit at {position:?}"),
                Tile::Animal => {
                    // We've reached the animal tile again completing the loop. Update the area for
                    // the final corner point.
                    area += determinant(&previous_corner, &position);
                    break;
                }
            }

            if current_tile.is_corner_pipe() {
                area += determinant(&previous_corner, &position);
                previous_corner = position;
            }

            steps += 1;
            position += direction;
        }

        let farthest_steps = steps / 2;
        let enclosed_tiles = area.abs() / 2 - steps as i128 / 2 + 1;

        Ok((farthest_steps, enclosed_tiles))
    }
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Grid> {
        Ok(Grid(Matrix::try_from_iter(
            s.lines().count(),
            s.lines()
                .next()
                .ok_or_else(|| anyhow!("Empty input"))?
                .len(),
            s.lines().flat_map(|line| line.bytes().map(Tile::try_from)),
        )?))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

const fn determinant(p1: &Position, p2: &Position) -> i128 {
    // This is a bit unfortunate, but we need to cast to a larger type to prevent overflow.
    let a = p1.row() as i128;
    let b = p1.col() as i128;
    let c = p2.row() as i128;
    let d = p2.col() as i128;

    a * d - b * c
}

pub fn solve(input: &str) -> Result<()> {
    let (farthest_steps, enclosed_tiles) = Grid::from_str(input)?.analyze_loop()?;

    println!("Part 1: {farthest_steps}");
    println!("Part 1: {enclosed_tiles}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const SAMPLE_INPUT2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const SAMPLE_INPUT3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const SAMPLE_INPUT4: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const SAMPLE_INPUT5: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test_case(SAMPLE_INPUT1, 4)]
    #[test_case(SAMPLE_INPUT2, 8)]
    fn farthest_steps(input: &str, expected: usize) {
        let grid = Grid::from_str(input).unwrap();
        let (farthest_steps, _) = grid.analyze_loop().unwrap();
        assert_eq!(farthest_steps, expected);
    }

    #[test_case(SAMPLE_INPUT3, 4)]
    #[test_case(SAMPLE_INPUT4, 8)]
    #[test_case(SAMPLE_INPUT5, 10)]
    fn enclosed_tiles(input: &str, expected: i128) {
        let grid = Grid::from_str(input).unwrap();
        let (_, enclosed_tiles) = grid.analyze_loop().unwrap();
        assert_eq!(enclosed_tiles, expected);
    }
}

use std::collections::HashSet;

use anyhow::{anyhow, Result};
use aoc_lib::matrix::{CardinalDirection, Position};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    /// The current position of the guard.
    position: Position,
    /// The direction the guard is facing currently.
    direction: CardinalDirection,
}

impl Guard {
    /// Returns the next position of the guard based on its current direction.
    fn next_position(&self) -> Option<Position> {
        self.position.checked_neighbor(self.direction.into())
    }

    /// Turns the guard to the right by 90 degrees.
    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

// Optimization:
//
// Create a jump table that maps the current position and direction to the number of steps it will
// take to reach the next obstruction. https://en.wikipedia.org/wiki/Branch_table

#[derive(Debug)]
struct ManufacturingLab {
    /// A set of positions where the obstructions are.
    obstructions: HashSet<Position>,
    /// The maximum number of rows and columns in the lab.
    size: usize,
}

impl ManufacturingLab {
    /// Returns a set of distinct positions that will be visited by the guard by simulating its
    /// movement, including the starting position.
    fn distinct_positions(&self, mut guard: Guard) -> HashSet<Position> {
        let mut positions = HashSet::new();

        while guard.position.row() < self.size && guard.position.col() < self.size {
            // Inserting the current position here means that it is within the lab boundary and
            // we need not worry about it being an obstruction.
            positions.insert(guard.position);
            let Some(next_position) = guard.next_position() else {
                break;
            };
            if self.obstructions.contains(&next_position) {
                guard.turn_right();
            } else {
                guard.position = next_position;
            }
        }

        positions
    }

    /// Simulate the guard's movement with an additional obstruction at the given position.
    ///
    /// The obstruction is removed after the simulation is done. Returns a boolean indicating
    /// whether the guard would get stuck in a loop.
    fn simulate_with_obstruction(&mut self, obstruction: Position, mut guard: Guard) -> bool {
        self.obstructions.insert(obstruction);

        // A set of guard position and direction pairs. The direction is required to determine
        // whether the guard is stuck in a loop.
        let mut visited: HashSet<(Position, CardinalDirection)> = HashSet::new();

        let stuck = loop {
            if guard.position.row() >= self.size || guard.position.col() >= self.size {
                break false;
            }
            if !visited.insert((guard.position, guard.direction)) {
                break true;
            }
            let Some(next_position) = guard.next_position() else {
                break false;
            };
            if self.obstructions.contains(&next_position) {
                guard.turn_right();
            } else {
                guard.position = next_position;
            }
        };

        self.obstructions.remove(&obstruction);
        stuck
    }

    /// Returns the number of positions where if an obstruction is placed, the guard will get stuck
    /// in a loop.
    fn obstruction_position_count(
        &mut self,
        mut distinct_positions: HashSet<Position>,
        guard: &Guard,
    ) -> usize {
        // The distinct position might contain the guard's starting position which is not a
        // candidate for obstruction placement.
        distinct_positions.remove(&guard.position);

        let mut count = 0;
        for position in distinct_positions {
            if self.simulate_with_obstruction(position, guard.clone()) {
                count += 1;
            }
        }

        count
    }
}

fn parse_input(input: &str) -> Result<(ManufacturingLab, Guard)> {
    let mut guard = None;
    let mut size = 0;
    let mut obstructions = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        size += 1;
        for (col, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => {
                    obstructions.insert(Position::new(row, col));
                }
                b'^' => {
                    guard = Some(Guard {
                        position: Position::new(row, col),
                        direction: CardinalDirection::Up,
                    });
                }
                _ => {}
            }
        }
    }

    Ok((
        ManufacturingLab { obstructions, size },
        guard.ok_or_else(|| anyhow!("No guard found in the input"))?,
    ))
}

pub fn solve(input: &str) -> Result<()> {
    let (mut lab, guard) = parse_input(input)?;
    let distinct_positions = lab.distinct_positions(guard.clone());

    println!("Part 1: {:?}", distinct_positions.len());
    println!(
        "Part 2: {:?}",
        lab.obstruction_position_count(distinct_positions, &guard)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn sample() {
        let (mut lab, guard) = parse_input(SAMPLE_INPUT).unwrap();
        let distinct_positions = lab.distinct_positions(guard.clone());

        assert_eq!(distinct_positions.len(), 41);
        assert_eq!(
            lab.obstruction_position_count(distinct_positions, &guard),
            6
        );
    }
}

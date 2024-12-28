use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::Write;
use std::ops::Deref;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_lib::matrix::{Matrix, Position};

#[derive(Debug)]
struct CorruptedBytes(Vec<Position>);

impl Deref for CorruptedBytes {
    type Target = [Position];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CorruptedBytes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = Vec::new();
        for line in s.lines() {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| anyhow!("expected a comma"))?;
            positions.push(Position::new(y.parse()?, x.parse()?));
        }
        Ok(CorruptedBytes(positions))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Byte {
    Safe,
    Corrupted,
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Byte::Safe => f.write_char('.'),
            Byte::Corrupted => f.write_char('#'),
        }
    }
}

#[derive(Debug)]
struct MemorySpace(Matrix<Byte>);

impl MemorySpace {
    /// Create a memory space of the given `size` with all the safe bytes.
    fn new(size: usize) -> Self {
        MemorySpace(Matrix::new_with(size, size, Byte::Safe))
    }

    /// Creates a new memory space with the corrupted bytes set at the given `positions` in the
    /// `self` memory space.
    fn with_corrupted(mut self, positions: &[Position]) -> Self {
        for &position in positions {
            self.0[position] = Byte::Corrupted;
        }
        self
    }

    /// Find the shortest path from the top-left corner to the bottom-right corner of the memory
    /// space. Returns the distance of the shortest path if it exists, [`None`] otherwise.
    fn shortest_path(&self) -> Option<u32> {
        let start = Position::new(0, 0);
        let end = Position::new(self.0.nrows() - 1, self.0.ncols() - 1);

        let mut queue = VecDeque::new();
        queue.push_back((start, 0u32));

        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((position, distance)) = queue.pop_front() {
            for neighbor in position.cardinal_neighbors() {
                if self
                    .0
                    .get(neighbor)
                    .map_or(true, |byte| *byte == Byte::Corrupted)
                {
                    continue;
                }
                if neighbor == end {
                    return Some(distance + 1);
                }
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        None
    }
}

impl fmt::Display for MemorySpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Perform a binary search to find the first position that satisfies the given predicate `f`.
/// Returns the index of the first position that satisfies the predicate.
fn binary_search<F>(mut low: usize, mut high: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while high - low > 1 {
        let mid = low + (high - low) / 2;
        if f(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    low
}

/// Find the first position of the corrupted byte that has no shortest path in a memory space of
/// the given `size` set with the corrupted bytes up to that position. The search starts from the
/// `start` position in the corrupted bytes.
fn find_first_position_with_no_shortest_path(
    size: usize,
    start: usize,
    corrupted: &CorruptedBytes,
) -> Position {
    let index = binary_search(start, corrupted.len(), |end| {
        MemorySpace::new(size)
            .with_corrupted(&corrupted[..end])
            .shortest_path()
            .is_none()
    });
    corrupted[index]
}

pub fn solve(input: &str) -> Result<()> {
    static SIZE: usize = 71;
    static KILOBYTE: usize = 1024;

    let bytes = CorruptedBytes::from_str(input)?;
    let memory = MemorySpace::new(SIZE).with_corrupted(&bytes[..KILOBYTE]);

    println!(
        "Part 1: {}",
        memory
            .shortest_path()
            .expect("shortest path should exist for memory with the first kilobyte")
    );

    let first_position = find_first_position_with_no_shortest_path(SIZE, KILOBYTE, &bytes);
    println!("Part 2: {},{}", first_position.col(), first_position.row());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn sample() {
        let bytes = CorruptedBytes::from_str(SAMPLE_INPUT).unwrap();
        let memory = MemorySpace::new(7).with_corrupted(&bytes[..12]);
        assert_eq!(memory.shortest_path().unwrap(), 22);
        let first_position = find_first_position_with_no_shortest_path(7, 12, &bytes);
        assert_eq!(first_position, Position::new(1, 6));
    }
}

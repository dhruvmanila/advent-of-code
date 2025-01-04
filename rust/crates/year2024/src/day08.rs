use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{Error, Result};
use aoc_lib::geom::{point2, Point2D, Vector2D};
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Frequency(u8);

impl Frequency {
    /// Returns the character representation of this frequency.
    const fn as_char(self) -> char {
        self.0 as char
    }
}

impl From<u8> for Frequency {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl fmt::Debug for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.as_char())
    }
}

#[derive(Debug)]
struct CityMap {
    frequencies: HashMap<Frequency, Vec<Point2D<i32>>>,
    size: i32,
}

impl CityMap {
    /// Returns `true` if the given point is out of bounds for this city map.
    fn is_out_of_bounds(&self, p: &Point2D<i32>) -> bool {
        p.x < 0 || p.y < 0 || p.x >= self.size || p.y >= self.size
    }

    /// Returns a set of positions where antinodes are located.
    fn antinodes(&self) -> HashSet<Point2D<i32>> {
        self.frequencies
            .values()
            .flat_map(|points| points.iter().tuple_combinations())
            .flat_map(|(p1, p2)| [(p1, p2), (p2, p1)])
            .filter_map(|(a, b)| {
                let antinode = Vector2D::between_points(a, b).transform_point(b);
                if self.is_out_of_bounds(&antinode) {
                    None
                } else {
                    Some(antinode)
                }
            })
            .collect::<HashSet<_>>()
    }

    /// Returns a set of positions where antinodes are located, considering the effects of resonant
    /// harmonics.
    fn antinodes_with_resonant_harmonics(&self) -> HashSet<Point2D<i32>> {
        let mut antinodes = HashSet::new();

        for (p1, p2) in self
            .frequencies
            .values()
            .flat_map(|points| points.iter().tuple_combinations())
        {
            // The antennas themselves are also antinodes.
            antinodes.insert(p1.clone());
            antinodes.insert(p2.clone());

            for (a, b) in [(p1, p2), (p2, p1)] {
                let vector = Vector2D::between_points(a, b);
                let mut end = Cow::Borrowed(b);

                // Keep on finding the next antinode on the same vector until we reach the edge of
                // the city map.
                loop {
                    let antinode = vector.transform_point(&end);
                    if self.is_out_of_bounds(&antinode) {
                        break;
                    }
                    antinodes.insert(antinode.clone());
                    end = Cow::Owned(antinode);
                }
            }
        }

        antinodes
    }

    /// Returns a displayable version of this city map.
    #[allow(dead_code)]
    fn display(&self) -> DisplayCityMap {
        DisplayCityMap::new(self)
    }
}

/// A displayable version of a city map.
struct DisplayCityMap<'a> {
    map: &'a CityMap,
    antinodes: Option<&'a HashSet<Point2D<i32>>>,
}

impl<'a> DisplayCityMap<'a> {
    fn new(map: &'a CityMap) -> DisplayCityMap<'a> {
        DisplayCityMap {
            map,
            antinodes: None,
        }
    }

    /// Sets the antinodes to display on this city map.
    #[allow(dead_code)]
    fn with_antinodes(mut self, antinodes: &'a HashSet<Point2D<i32>>) -> DisplayCityMap<'a> {
        self.antinodes = Some(antinodes);
        self
    }
}

impl fmt::Display for DisplayCityMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.map.size {
            'next_point: for x in 0..self.map.size {
                let point = point2(x, y);

                // First, check if there's a frequency at this point.
                for (frequency, points) in &self.map.frequencies {
                    if points.contains(&point) {
                        f.write_char(frequency.as_char())?;
                        continue 'next_point;
                    }
                }

                // If there's no frequency, check if there's an antinode.
                if self
                    .antinodes
                    .is_some_and(|antinodes| antinodes.contains(&point))
                {
                    f.write_char('#')?;
                    continue;
                }

                // Othewise, it's just an empty space.
                f.write_char('.')?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl FromStr for CityMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<CityMap> {
        let mut size = 0;
        let mut frequencies = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            size += 1;
            for (x, byte) in line.bytes().enumerate() {
                if byte == b'.' {
                    continue;
                }
                frequencies
                    .entry(Frequency::from(byte))
                    .or_insert_with(Vec::new)
                    .push(point2(i32::try_from(x)?, i32::try_from(y)?));
            }
        }

        Ok(CityMap { frequencies, size })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let map = CityMap::from_str(input)?;

    let antinodes = map.antinodes();
    println!("Part 1: {}", antinodes.len());

    let antinodes = map.antinodes_with_resonant_harmonics();
    println!("Part 2: {}", antinodes.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    const SAMPLE_INPUT2: &str = "\
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";

    const SAMPLE_INPUT3: &str = "\
..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

    const SAMPLE_INPUT4: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const SAMPLE_INPUT5: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test_case(SAMPLE_INPUT1, 2)]
    #[test_case(SAMPLE_INPUT2, 4)]
    #[test_case(SAMPLE_INPUT3, 4)]
    #[test_case(SAMPLE_INPUT4, 14)]
    fn antinodes(input: &str, expected: usize) {
        let city_map = input.parse::<CityMap>().unwrap();
        assert_eq!(city_map.antinodes().len(), expected);
    }

    #[test_case(SAMPLE_INPUT5, 9)]
    #[test_case(SAMPLE_INPUT4, 34)]
    fn antinodes_with_resonant_harmonics(input: &str, expected: usize) {
        let city_map = input.parse::<CityMap>().unwrap();
        let antinodes = city_map.antinodes_with_resonant_harmonics();
        assert_eq!(antinodes.len(), expected);
    }
}

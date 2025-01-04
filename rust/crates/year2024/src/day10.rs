use std::collections::HashSet;

use anyhow::Result;
use aoc_lib::matrix::{Position, SquareMatrix};

/// A topographic map of the surrounding area.
#[derive(Debug)]
struct TopographicMap(SquareMatrix<u8>);

impl TopographicMap {
    /// Returns an iterator over all the trailheads in the map.
    fn trailheads(&self) -> impl Iterator<Item = Trailhead> + '_ {
        self.0.enumerate().filter_map(move |(position, elevation)| {
            if *elevation == 0 {
                Some(Trailhead {
                    map: self,
                    position,
                })
            } else {
                None
            }
        })
    }

    /// Returns the sum of all the scores and ratings of all the trailheads in the map.
    fn sum_trailhead_scores_and_ratings(&self) -> (usize, usize) {
        self.trailheads()
            .map(|trailhead| trailhead.score_and_rating())
            .fold((0, 0), |(sum_score, sum_rating), (score, rating)| {
                (sum_score + score, sum_rating + rating)
            })
    }
}

impl From<&str> for TopographicMap {
    fn from(s: &str) -> TopographicMap {
        TopographicMap(SquareMatrix::from_iter(
            s.lines().count(),
            s.lines().flat_map(|line| {
                line.bytes().map(|byte| {
                    if byte.is_ascii_digit() {
                        byte - b'0'
                    } else {
                        // Keep the impassable tiles as is
                        byte
                    }
                })
            }),
        ))
    }
}

/// A trailhead in the topographic map.
struct Trailhead<'a> {
    /// The map in which the trailhead is located.
    map: &'a TopographicMap,
    /// The starting position of the trail.
    position: Position,
}

impl Trailhead<'_> {
    /// Returns the score and rating of the trailhead.
    fn score_and_rating(&self) -> (usize, usize) {
        let mut rating = 0;
        let mut queue = vec![(self.position, 1u8)];
        let mut visited_nines = HashSet::new();

        while let Some((position, elevation)) = queue.pop() {
            for neighbor in position.cardinal_neighbors() {
                if self.map.0.get(neighbor).copied() == Some(elevation) {
                    if elevation == 9 {
                        visited_nines.insert(neighbor);
                        rating += 1;
                    } else {
                        queue.push((neighbor, elevation + 1));
                    }
                }
            }
        }

        (visited_nines.len(), rating)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let map = TopographicMap::from(input);
    let (score, rating) = map.sum_trailhead_scores_and_ratings();

    println!("Part 1: {score}");
    println!("Part 2: {rating}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT0: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    const SAMPLE_INPUT1: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";

    const SAMPLE_INPUT2: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const SAMPLE_INPUT3: &str = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";

    const SAMPLE_INPUT4: &str = "\
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";

    const SAMPLE_INPUT5: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const SAMPLE_INPUT6: &str = "\
012345
123456
234567
345678
4.6789
56789.
";

    #[test_case(SAMPLE_INPUT0, 36)]
    #[test_case(SAMPLE_INPUT1, 2)]
    #[test_case(SAMPLE_INPUT2, 4)]
    #[test_case(SAMPLE_INPUT3, 3)]
    fn scores(input: &str, expected: usize) {
        let map = TopographicMap::from(input);
        assert_eq!(map.sum_trailhead_scores_and_ratings().0, expected);
    }

    #[test_case(SAMPLE_INPUT0, 81)]
    #[test_case(SAMPLE_INPUT4, 3)]
    #[test_case(SAMPLE_INPUT5, 13)]
    #[test_case(SAMPLE_INPUT6, 227)]
    fn ratings(input: &str, expected: usize) {
        let map = TopographicMap::from(input);
        assert_eq!(map.sum_trailhead_scores_and_ratings().1, expected);
    }
}

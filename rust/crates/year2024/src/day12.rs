use std::collections::{HashMap, HashSet};
use std::fmt;

use anyhow::Result;
use aoc_lib::matrix::{Direction, Position, SquareMatrix};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct PlantLabel(u8);

impl fmt::Debug for PlantLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

/// A garden with regions of plants.
#[derive(Debug)]
struct Garden(HashMap<PlantLabel, Vec<Region>>);

impl Garden {
    fn from_plots(plots: &SquareMatrix<PlantLabel>) -> Garden {
        let mut plant_regions: HashMap<PlantLabel, Vec<Region>> = HashMap::new();

        for (position, label) in plots.enumerate() {
            // Check whether this position is already in an existing region.
            //
            // This is the reason to use a HashMap instead of just a vector of regions. We will
            // only check the regions of the current plant label instead of all the regions.
            if let Some(regions) = plant_regions.get(label) {
                if regions.iter().any(|region| region.contains(&position)) {
                    continue;
                }
            }

            // Otherwise, find all the positions in this region using a flood fill.
            let mut region = HashSet::with_capacity(1);
            region.insert(position);

            let mut queue = vec![position];
            while let Some(position) = queue.pop() {
                for neighbor in position.cardinal_neighbors() {
                    if plots.get(neighbor) == Some(label) && region.insert(neighbor) {
                        queue.push(neighbor);
                    }
                }
            }

            plant_regions
                .entry(*label)
                .or_default()
                .push(Region(region));
        }

        Garden(plant_regions)
    }

    /// Returns the total price of fencing the garden using the perimeter of each region.
    fn fencing_price_using_perimeter(&self) -> usize {
        self.0
            .values()
            .flat_map(|regions| regions.iter())
            .map(|region| region.perimeter() * region.area())
            .sum()
    }

    /// Returns the total price of fencing the garden using the number of sides for each region.
    fn fencing_price_using_sides(&self) -> usize {
        self.0
            .values()
            .flat_map(|regions| regions.iter())
            .map(|region| region.sides() * region.area())
            .sum()
    }
}

impl From<&str> for Garden {
    fn from(s: &str) -> Garden {
        Garden::from_plots(&SquareMatrix::from_iter(
            s.lines().count(),
            s.lines().flat_map(|line| line.bytes().map(PlantLabel)),
        ))
    }
}

/// A region of the garden represented by a set of positions.
#[derive(Debug)]
struct Region(HashSet<Position>);

impl Region {
    /// Returns true if the given position is within the region.
    fn contains(&self, position: &Position) -> bool {
        self.0.contains(position)
    }

    /// Returns the number of sides of the region.
    fn sides(&self) -> usize {
        // Corners are defined by three directions: the two sides and the diagonal.
        static CORNERS: [(Direction, Direction, Direction); 4] = [
            (Direction::Left, Direction::Up, Direction::TopLeft),
            (Direction::Up, Direction::Right, Direction::TopRight),
            (Direction::Right, Direction::Down, Direction::BottomRight),
            (Direction::Down, Direction::Left, Direction::BottomLeft),
        ];

        let mut sides = 0;
        for point in &self.0 {
            for (direction1, direction2, diagonal) in CORNERS {
                match (
                    point.checked_neighbor(direction1),
                    point.checked_neighbor(direction2),
                ) {
                    (Some(neighbor1), Some(neighbor2)) => {
                        match (self.contains(&neighbor1), self.contains(&neighbor2)) {
                            (true, true) => {
                                // If both neighbors are in the region, then this might be a
                                // convex corner. This can be confirmed by checking the diagonal
                                // neighbor and if it's not in the region, then this is a corner.
                                //
                                // SAFETY: The diagonal neighbor is always within the bounds of a
                                // square matrix (which it is) for a convex corner.
                                if !self.contains(&(*point + diagonal)) {
                                    sides += 1;
                                }
                            }
                            (false, false) => {
                                // If both neighbors are outside the region, then this is a corner.
                                sides += 1;
                            }
                            _ => {}
                        }
                    }
                    (None, Some(neighbor)) | (Some(neighbor), None) => {
                        // Here, one of the neighbors is outside the garden so if the other one is
                        // not in the region, then this is a corner.
                        if !self.contains(&neighbor) {
                            sides += 1;
                        }
                    }
                    (None, None) => {
                        // This is the upper-left corner of the entire garden and there's only one
                        // such corner.
                        sides += 1;
                    }
                }
            }
        }
        sides
    }

    /// Returns the perimeter of the region.
    fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        for point in &self.0 {
            for direction in Direction::CARDINAL {
                if let Some(neighbor) = point.checked_neighbor(direction) {
                    if !self.contains(&neighbor) {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    /// Returns the area of the region.
    fn area(&self) -> usize {
        self.0.len()
    }
}

pub fn solve(input: &str) -> Result<()> {
    let garden = Garden::from(input);

    println!("Part 1: {}", garden.fencing_price_using_perimeter());
    println!("Part 2: {}", garden.fencing_price_using_sides());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const SAMPLE_INPUT2: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const SAMPLE_INPUT3: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const SAMPLE_INPUT4: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const SAMPLE_INPUT5: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test_case(SAMPLE_INPUT1, 140)]
    #[test_case(SAMPLE_INPUT2, 772)]
    #[test_case(SAMPLE_INPUT3, 1930)]
    fn fencing_price_using_perimeter(input: &str, expected: usize) {
        let garden = Garden::from(input);
        assert_eq!(garden.fencing_price_using_perimeter(), expected);
    }

    #[test_case(SAMPLE_INPUT1, 80)]
    #[test_case(SAMPLE_INPUT2, 436)]
    #[test_case(SAMPLE_INPUT4, 236)]
    #[test_case(SAMPLE_INPUT5, 368)]
    fn fencing_price_using_sides(input: &str, expected: usize) {
        let garden = Garden::from(input);
        assert_eq!(garden.fencing_price_using_sides(), expected);
    }
}

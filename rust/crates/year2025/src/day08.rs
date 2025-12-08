use std::{collections::HashMap, fmt::Debug, hash::Hash, str::FromStr};

use anyhow::{Error, Result, bail};
use aoc_lib::geom::Point3D;

/// A union-find (disjoint set) data structure.
///
/// Reference: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
#[derive(Debug)]
struct UnionFind<T> {
    payloads: HashMap<T, usize>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
    count: usize,
}

impl<T: Debug + Eq + Hash> UnionFind<T> {
    /// Creates a new union-find structure with the given capacity.
    fn with_capacity(capacity: usize) -> UnionFind<T> {
        UnionFind {
            payloads: HashMap::with_capacity(capacity),
            parents: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            count: 0,
        }
    }

    /// Inserts a new item into the data structure if it is not already present.
    fn insert(&mut self, item: T) {
        if !self.payloads.contains_key(&item) {
            let index = self.parents.len();
            self.payloads.insert(item, index);
            self.parents.push(index);
            self.sizes.push(1);
            self.count += 1;
        }
    }

    /// Finds the representative index of the set containing the given item, [`None`] if not found.
    ///
    /// The representative index is the index of the root element of the set. This is done using
    /// the path compression optimization.
    fn find(&mut self, item: &T) -> Option<usize> {
        fn find_by_index<T>(union_find: &mut UnionFind<T>, index: usize) -> usize {
            if union_find.parents[index] != index {
                union_find.parents[index] = find_by_index(union_find, union_find.parents[index]);
            }
            union_find.parents[index]
        }

        self.payloads
            .get(item)
            .copied()
            .map(|index| find_by_index(self, index))
    }

    /// Unions the sets containing the two given items.
    ///
    /// This method does nothing if either item is not found or if they are already in the same
    /// set. The union is done using the union by size optimization.
    ///
    /// Returns `true` if a union was performed, `false` if either item was not found or they were
    /// already in the same set.
    fn union(&mut self, first: &T, second: &T) -> bool {
        let (Some(mut first_root), Some(mut second_root)) = (self.find(first), self.find(second))
        else {
            return false;
        };

        if first_root == second_root {
            return false;
        }

        if self.sizes[first_root] < self.sizes[second_root] {
            std::mem::swap(&mut first_root, &mut second_root);
        }

        self.parents[second_root] = first_root;
        self.sizes[first_root] += self.sizes[second_root];
        self.count -= 1;
        true
    }

    /// Returns the number of disjoint sets in the data structure.
    fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug)]
struct LastConnection(Point3D<u32>, Point3D<u32>);

impl LastConnection {
    /// Returns the product of the x-coordinates of the two junction boxes in the last connection.
    fn xproduct(&self) -> u32 {
        self.0.x * self.1.x
    }
}

/// A distance between two junction boxes represented as a tuple of:
/// 1. The distance
/// 2. The two junction box positions
type JunctionBoxDistance = (f64, (Point3D<u32>, Point3D<u32>));

#[derive(Debug)]
struct JunctionBoxes {
    distances: Vec<JunctionBoxDistance>,
    union_find: UnionFind<Point3D<u32>>,
}

impl JunctionBoxes {
    fn new(positions: &[Point3D<u32>]) -> JunctionBoxes {
        let mut union_find = UnionFind::with_capacity(positions.len());
        let mut distances = Vec::with_capacity((positions.len() * (positions.len() - 1)) / 2);

        for (index, &first) in positions.iter().enumerate() {
            union_find.insert(first);
            for second in positions.get(index + 1..).unwrap_or(&[]) {
                distances.push((first.squared_distance(second), (first, *second)));
            }
        }

        distances.sort_unstable_by(|(d1, _), (d2, _)| d1.total_cmp(d2));

        JunctionBoxes {
            distances,
            union_find,
        }
    }

    /// Connects the junction boxes until they're all in the same circuit.
    ///
    /// The return value is a tuple containing:
    /// 1. Product of the sizes of the three largest circuits after making `checkpoint` connections
    /// 2. The last connection made to connect all junction boxes
    ///
    /// # Panics
    ///
    /// Panics if `checkpoint` exceeds the number of possible connections.
    fn connect(&mut self, checkpoint: usize) -> (usize, LastConnection) {
        assert!(
            checkpoint <= self.distances.len(),
            "Checkpoint {} exceeds number of possible connections {}",
            checkpoint,
            self.distances.len()
        );

        for &(_, (first, second)) in &self.distances[..checkpoint] {
            self.union_find.union(&first, &second);
        }

        let mut sizes = self.union_find.sizes.clone();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        let product_of_largest_three: usize = sizes.iter().take(3).product();

        let mut last_connection = None;
        for &(_, (first, second)) in &self.distances[checkpoint..] {
            self.union_find.union(&first, &second);
            if self.union_find.count() == 1 {
                last_connection = Some(LastConnection(first, second));
                break;
            }
        }

        (
            product_of_largest_three,
            last_connection.expect("all junction boxes should be connected"),
        )
    }
}

impl FromStr for JunctionBoxes {
    type Err = Error;

    fn from_str(s: &str) -> Result<JunctionBoxes, Error> {
        Ok(JunctionBoxes::new(
            &s.lines()
                .map(|line| {
                    let coordinates = line
                        .split(',')
                        .map(str::parse::<u32>)
                        .collect::<Result<Vec<_>, _>>()?;
                    let &[x, y, z] = coordinates.as_slice() else {
                        bail!(
                            "Expected 3 coordinates per line, got {} on {line:?}",
                            coordinates.len()
                        );
                    };
                    Ok(Point3D::new(x, y, z))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

pub fn solve(input: &str) -> Result<()> {
    let mut junction_boxes = JunctionBoxes::from_str(input)?;
    let (product_of_largest_three_circuits, last_connection) = junction_boxes.connect(1000);

    println!("Part 1: {product_of_largest_three_circuits}");
    println!("Part 2: {}", last_connection.xproduct());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn sample() {
        let mut junction_boxes = JunctionBoxes::from_str(SAMPLE_INPUT).unwrap();
        let (product_of_largest_three_circuits, last_connection) = junction_boxes.connect(10);
        assert_eq!(product_of_largest_three_circuits, 40);
        assert_eq!(last_connection.xproduct(), 25272);
    }
}

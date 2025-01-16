use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use anyhow::Result;
use aoc_lib::matrix::{CardinalDirection, Matrix, MatrixError, Position};
use aoc_lib::MinHeap;

/// Heat loss for each city block.
type HeatLoss = u8;

/// A node in the search graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    /// The position of the city block.
    position: Position,
    /// The direction that the node is facing.
    direction: CardinalDirection,
    /// The number of consecutive steps taken till this node in a single direction.
    steps: u32,
}

impl Node {
    /// Return the node to start the search from.
    ///
    /// This is the top left corner of the graph.
    const fn start_in(direction: CardinalDirection) -> Node {
        Node {
            position: Position::zero(),
            direction,
            steps: 0,
        }
    }
}

impl Node {
    /// Returns the successors of this node based on the minimum and maximum steps allowed.
    fn successors(&self, min_steps: u32, max_steps: u32) -> Vec<Node> {
        let mut successors = Vec::new();
        if self.steps < min_steps {
            if let Some(next_position) = self.position.checked_neighbor(self.direction.into()) {
                successors.push(Node {
                    position: next_position,
                    direction: self.direction,
                    steps: self.steps + 1,
                });
            }
        } else {
            for (next_direction, next_steps) in [
                (self.direction, self.steps + 1),
                (self.direction.turn_left(), 1),
                (self.direction.turn_right(), 1),
            ] {
                if next_steps > max_steps {
                    continue;
                }
                if let Some(next_position) = self.position.checked_neighbor(next_direction.into()) {
                    successors.push(Node {
                        position: next_position,
                        direction: next_direction,
                        steps: next_steps,
                    });
                }
            }
        }
        successors
    }
}

#[derive(Debug)]
struct CityMap(Matrix<HeatLoss>);

impl CityMap {
    /// Return the minimum heat loss when traversing the city map from the top-left corner to the
    /// bottom-right corner.
    ///
    /// The `min_step` is the minimum number of consecutive steps that **need** to be taken in a
    /// single direction before making a turn. The `max_step` is the maximum number of steps that
    /// **can** be taken in a single direction before making a turn.
    ///
    /// This uses [Dijkstra's algorithm] to find the shortest path.
    ///
    /// [Dijkstra's algorithm]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    fn min_heat_loss(&self, min_step: u32, max_step: u32) -> u32 {
        // The end position is the bottom-right corner of the map.
        let end = Position::new(self.0.nrows() - 1, self.0.ncols() - 1);

        // Total cost to reach each node (position, direction, steps).
        let mut total_cost = HashMap::new();

        // Priority queue of nodes to visit, ordered by total cost.
        let mut queue = MinHeap::new();

        // Start at the top-left corner of the map facing right and down.
        queue.push(0, Node::start_in(CardinalDirection::Right));
        queue.push(0, Node::start_in(CardinalDirection::Down));

        while let Some((cost, node)) = queue.pop() {
            if node.position == end && node.steps >= min_step {
                return cost;
            }
            for next_node in node.successors(min_step, max_step) {
                let Some(&heat_loss) = self.0.get(next_node.position) else {
                    continue;
                };
                let new_cost = cost + u32::from(heat_loss);
                if total_cost
                    .get(&next_node)
                    .map_or(true, |&existing_cost| new_cost < existing_cost)
                {
                    total_cost.insert(next_node.clone(), new_cost);
                    queue.push(new_cost, next_node);
                }
            }
        }

        unreachable!("End position not reachable")
    }

    /// Return the minimum heat loss when directing the crucible from the laval pool to the machine
    /// parts factory.
    fn min_heat_loss_for_crucible(&self) -> u32 {
        self.min_heat_loss(1, 3)
    }

    /// Return the minimum heat loss when directing the ultra crucible from the laval pool to the
    /// machine parts factory.
    fn min_heat_loss_for_ultra_crucible(&self) -> u32 {
        self.min_heat_loss(4, 10)
    }
}

impl FromStr for CityMap {
    type Err = MatrixError;

    fn from_str(s: &str) -> Result<CityMap, MatrixError> {
        Matrix::from_rows(s.lines().map(|line| line.bytes().map(|byte| byte - b'0'))).map(CityMap)
    }
}

impl fmt::Display for CityMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let map = CityMap::from_str(input)?;

    println!("Part 1: {}", map.min_heat_loss_for_crucible());
    println!("Part 2: {}", map.min_heat_loss_for_ultra_crucible());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const SAMPLE_INPUT2: &str = "\
111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn sample1() {
        let map = CityMap::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(map.min_heat_loss_for_crucible(), 102);
        assert_eq!(map.min_heat_loss_for_ultra_crucible(), 94);
    }

    #[test]
    fn sample2() {
        let map = CityMap::from_str(SAMPLE_INPUT2).unwrap();
        assert_eq!(map.min_heat_loss_for_ultra_crucible(), 71);
    }
}

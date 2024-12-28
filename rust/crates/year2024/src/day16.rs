use std::array::IntoIter;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{self, Write};
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use aoc_lib::matrix::{CardinalDirection, Matrix, Position};

/// A tile in the maze.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => f.write_char('#'),
            Tile::Empty => f.write_char('.'),
        }
    }
}

/// A node in the maze which is a combination of a position and a direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MazeNode {
    position: Position,
    direction: CardinalDirection,
}

/// A [`MazeNode`] with a cost associated with it.
///
/// The [`Ord`] implementation is reversed so that the priority queue pops the node
/// with the lowest cost first.
#[derive(Debug)]
struct MazeNodeWithCost {
    node: MazeNode,
    cost: u32,
}

impl PartialEq for MazeNodeWithCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for MazeNodeWithCost {}

impl PartialOrd for MazeNodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MazeNodeWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug)]
struct ReindeerMaze {
    /// The maze represented as a matrix of tiles.
    map: Matrix<Tile>,
    /// The source position.
    source: Position,
    /// The target position.
    target: Position,
}

impl ReindeerMaze {
    /// Solve the maze using [Dijkstra's algorithm] to find the path between the `source` and
    /// `target` positions with the lowest cost.
    ///
    /// Returns a tuple where the elements are:
    /// 1. The lowest score to reach the end of the maze
    /// 2. The number of unique positions for all paths between the source and target positions
    ///    with the lowest score.
    ///
    /// [Dijkstra's algorithm]: https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    fn solve(&self) -> (u32, usize) {
        let start = MazeNode {
            position: self.source,
            direction: CardinalDirection::EAST,
        };

        // Keep track of the total cost to reach each node.
        let mut total_cost: HashMap<MazeNode, u32> = HashMap::new();
        total_cost.insert(start.clone(), 0);

        // Keep track of the previous node for each node to reconstruct the path.
        //
        // The difference here is that the value is a vector of nodes instead of just a single node
        // because we need to keep track of all paths that lead to end node with the lowest cost.
        let mut previous: HashMap<MazeNode, Vec<MazeNode>> = HashMap::new();

        // Priority queue to keep track of the nodes to visit. The ordering for the custom struct
        // is reversed so that the node with the lowest cost is popped first (i.e. a min-heap).
        let mut queue = BinaryHeap::new();
        queue.push(MazeNodeWithCost {
            node: start,
            cost: 0,
        });

        // The end node. This is required to reconstruct the path and we can't use `self.end`
        // as we need the direction of the end node as well.
        let mut target_node = None;

        while let Some(MazeNodeWithCost { node, cost }) = queue.pop() {
            if node.position == self.target {
                target_node = Some(node);
                break;
            }
            for (successor, move_cost) in self.successors(&node) {
                let new_cost = cost + move_cost;
                let existing_cost = total_cost.get(&successor);
                if existing_cost.map_or(true, |&existing_cost| new_cost < existing_cost) {
                    // This branch is taken if the node has not been visited yet or if the new cost
                    // of reaching the node is lower than the existing cost.
                    total_cost.insert(successor.clone(), new_cost);
                    previous.insert(successor.clone(), vec![node.clone()]);
                    queue.push(MazeNodeWithCost {
                        node: successor,
                        cost: new_cost,
                    });
                } else if existing_cost == Some(&new_cost) {
                    // This branch is taken when there's an alternative path to the node with the
                    // same cost.
                    //
                    // SAFETY: If the cost is the same, then we've already visited the node
                    // from a different path, so the node must exist.
                    previous.get_mut(&successor).unwrap().push(node.clone());
                }
            }
        }

        let target_node = target_node.expect("path with the lowest cost should exist");
        let lowest_score = *total_cost.get(&target_node).unwrap();

        let mut positions = HashSet::new();
        let mut stack = vec![target_node];
        while let Some(node) = stack.pop() {
            // We can't skip if the position already exists in the set because a turn only differs
            // in the direction while the position remains the same.
            positions.insert(node.position);
            if let Some(prev) = previous.get(&node) {
                stack.extend(prev.iter().cloned());
            }
        }

        (lowest_score, positions.len())
    }

    fn successors<'a>(&'a self, node: &'a MazeNode) -> Successors<'a> {
        Successors::new(&self.map, node)
    }
}

/// The different states to represent all possible successors of a node in the maze.
#[derive(Debug, Clone, Copy)]
enum SuccessorState {
    /// Move forward in the current direction.
    MoveForward,
    /// Turn in the given direction.
    Turn(TurnDirection),
}

impl SuccessorState {
    /// Returns an iterator over all possible successor states in the order of priority.
    fn iter() -> IntoIter<SuccessorState, 3> {
        [
            SuccessorState::MoveForward,
            SuccessorState::Turn(TurnDirection::Left),
            SuccessorState::Turn(TurnDirection::Right),
        ]
        .into_iter()
    }

    /// Returns the cost of the state.
    const fn cost(self) -> u32 {
        match self {
            SuccessorState::MoveForward => 1,
            SuccessorState::Turn(_) => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    Left,
    Right,
}

/// An iterator over the successor nodes from a given node in the maze.
struct Successors<'a> {
    /// The maze to which the node belongs.
    maze: &'a Matrix<Tile>,
    /// The node for which to find the successors.
    node: &'a MazeNode,
    /// The current state of the successor iterator.
    state: IntoIter<SuccessorState, 3>,
}

impl<'a> Successors<'a> {
    fn new(maze: &'a Matrix<Tile>, node: &'a MazeNode) -> Self {
        Self {
            maze,
            node,
            state: SuccessorState::iter(),
        }
    }
}

impl Iterator for Successors<'_> {
    type Item = (MazeNode, u32);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_state = self.state.next()?;

            match next_state {
                SuccessorState::MoveForward => {
                    let next_position = self.node.position.neighbor(self.node.direction.into())?;
                    if self.maze[next_position] == Tile::Empty {
                        return Some((
                            MazeNode {
                                position: next_position,
                                direction: self.node.direction,
                            },
                            next_state.cost(),
                        ));
                    }
                }
                SuccessorState::Turn(turn_direction) => {
                    let next_direction = match turn_direction {
                        TurnDirection::Left => self.node.direction.turn_left(),
                        TurnDirection::Right => self.node.direction.turn_right(),
                    };
                    let next_position = self.node.position.neighbor(next_direction.into())?;
                    if self.maze[next_position] == Tile::Empty {
                        return Some((
                            MazeNode {
                                position: self.node.position,
                                direction: next_direction,
                            },
                            next_state.cost(),
                        ));
                    }
                }
            }
        }
    }
}

impl FromStr for ReindeerMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s
            .lines()
            .next()
            .ok_or_else(|| anyhow!("Empty input"))?
            .len();

        let mut tiles = Vec::with_capacity(nrows * ncols);
        let mut start = None;
        let mut end = None;

        for (row, line) in s.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                tiles.push(match byte {
                    b'#' => Tile::Wall,
                    b'.' => Tile::Empty,
                    b'S' => {
                        start = Some(Position::new(row, col));
                        Tile::Empty
                    }
                    b'E' => {
                        end = Some(Position::new(row, col));
                        Tile::Empty
                    }
                    _ => bail!(
                        "row {row} col {col}: unexpected tile character: {}",
                        byte as char
                    ),
                });
            }
        }

        let (Some(start), Some(end)) = (start, end) else {
            bail!("Expected the maze to contain start and end position marked by 'S' and 'E'");
        };

        Ok(Self {
            map: Matrix::from_vec(nrows, ncols, tiles),
            source: start,
            target: end,
        })
    }
}

impl fmt::Display for ReindeerMaze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.map, f)
    }
}

pub fn solve(input: &str) -> Result<()> {
    let maze = ReindeerMaze::from_str(input)?;
    let (lowest_score, unique_positions) = maze.solve();

    println!("Part 1: {lowest_score}");
    println!("Part 2: {unique_positions}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SAMPLE_INPUT2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test_case(SAMPLE_INPUT1, 7036, 45)]
    #[test_case(SAMPLE_INPUT2, 11048, 64)]
    fn solve(input: &str, expected_lowest_score: u32, expected_unique_positions: usize) {
        let maze = ReindeerMaze::from_str(input).unwrap();
        let (lowest_score, unique_positions) = maze.solve();
        assert_eq!(lowest_score, expected_lowest_score);
        assert_eq!(unique_positions, expected_unique_positions);
    }
}

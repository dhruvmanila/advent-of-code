use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};
use num_integer::Integer;

/// Direction to take from the current node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

/// A sequence of directions to take.
#[derive(Debug)]
struct Instructions(Vec<Direction>);

impl Instructions {
    /// Returns an iterator over the directions, repeating forever.
    fn cycle(&self) -> impl Iterator<Item = Direction> + '_ {
        self.0.iter().copied().cycle()
    }
}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instructions> {
        let directions = s
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(anyhow!("Invalid direction: {:?}", c)),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Instructions(directions))
    }
}

/// A node in the desert map.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node(u8, u8, u8);

impl Node {
    /// The starting node (AAA).
    const START: Node = Node(b'A', b'A', b'A');
    /// The ending node (ZZZ).
    const END: Node = Node(b'Z', b'Z', b'Z');

    /// Check if this node is a ghost start node i.e., a node that ends in A.
    const fn is_ghost_start(self) -> bool {
        self.2 == b'A'
    }

    /// Check if this node is a ghost end node i.e., a node that ends in Z.
    const fn is_ghost_end(self) -> bool {
        self.2 == b'Z'
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0 as char, self.1 as char, self.2 as char)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Node> {
        let [first, second, third] = s.as_bytes() else {
            bail!("Expected 3 character long string: {:?}", s);
        };
        Ok(Node(*first, *second, *third))
    }
}

/// A desert map.
#[derive(Debug)]
struct DesertMap {
    /// The instructions to follow.
    instructions: Instructions,
    /// Mapping from node to the following two nodes in (left, right) order.
    map: HashMap<Node, (Node, Node)>,
}

impl DesertMap {
    /// Calculate the number of steps it takes to get from the `start` node to
    /// reach a node that satisfies the `is_end` predicate.
    ///
    /// This loops over the instructions until the `is_end` predicate returns
    /// true for the current node. The caller is responsible for ensuring that
    /// the instructions don't loop forever through the `is_end` predicate.
    ///
    /// Returns an error if the instructions end unexpectedly or if there's no
    /// entry in the map for the current node.
    fn steps_impl<F>(&self, start: Node, is_end: F) -> Result<u64>
    where
        F: Fn(Node) -> bool,
    {
        let mut steps = 0;
        let mut current = start;
        let mut instructions = self.instructions.cycle();

        while !is_end(current) {
            let Some(direction) = instructions.next() else {
                bail!("Unexpected end of instructions");
            };
            let Some((left, right)) = self.map.get(&current) else {
                bail!("No entry found for {} in map", current);
            };
            current = match direction {
                Direction::Left => *left,
                Direction::Right => *right,
            };
            steps += 1;
        }

        Ok(steps)
    }

    /// Calculate the number of steps it takes to get from the node `AAA` to
    /// the node `ZZZ`.
    fn steps(&self) -> Result<u64> {
        self.steps_impl(Node::START, |node| node == Node::END)
    }

    /// Calculate the number of steps it takes for all the ghost start nodes to
    /// reach all the ghost end nodes.
    fn ghost_steps(&self) -> Result<u64> {
        self.map
            .keys()
            .copied()
            .filter(|node| node.is_ghost_start())
            .map(|start| self.steps_impl(start, Node::is_ghost_end))
            .try_fold(1, |acc, steps| Ok(acc.lcm(&steps?)))
    }
}

impl FromStr for DesertMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<DesertMap> {
        let mut lines = s.lines();

        let instructions = lines
            .next()
            .ok_or_else(|| anyhow!("Expected instruction line"))?
            .parse::<Instructions>()?;

        let lines = lines.skip(1); // Skip the empty line

        let mut map = HashMap::new();
        for (idx, line) in lines.enumerate() {
            if line.len() != 16 {
                bail!("Instruction {} is not 15 characters long: {:?}", idx, line);
            }

            map.insert(
                line[0..3].parse()?,
                (line[7..10].parse()?, line[12..15].parse()?),
            );
        }

        Ok(DesertMap { instructions, map })
    }
}

pub fn solve(input: &str) -> Result<()> {
    let map = DesertMap::from_str(input)?;

    println!("Part 1: {}", map.steps()?);
    println!("Part 2: {}", map.ghost_steps()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const SAMPLE_INPUT2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const SAMPLE_INPUT3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test_case(SAMPLE_INPUT1, 2)]
    #[test_case(SAMPLE_INPUT2, 6)]
    fn steps(input: &str, expected: u64) {
        let map = DesertMap::from_str(input).unwrap();
        assert_eq!(map.steps().unwrap(), expected);
    }

    #[test]
    fn ghost_steps() {
        let map = DesertMap::from_str(SAMPLE_INPUT3).unwrap();
        assert_eq!(map.ghost_steps().unwrap(), 6);
    }
}

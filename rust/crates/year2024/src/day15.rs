use std::collections::VecDeque;
use std::fmt::{self, Write};
use std::slice::Iter;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use aoc_lib::matrix::{CardinalDirection, Matrix, Position};

#[derive(Debug)]
struct Moves(Vec<CardinalDirection>);

impl Moves {
    fn iter(&self) -> Iter<'_, CardinalDirection> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a Moves {
    type Item = &'a CardinalDirection;
    type IntoIter = Iter<'a, CardinalDirection>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        Self(
            s.lines()
                .flat_map(|line| {
                    line.bytes().map(|byte| match byte {
                        b'^' => CardinalDirection::Up,
                        b'v' => CardinalDirection::Down,
                        b'<' => CardinalDirection::Left,
                        b'>' => CardinalDirection::Right,
                        _ => panic!("Invalid move: {}", byte as char),
                    })
                })
                .collect(),
        )
    }
}

/// A tile in the warehouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    Box,
}

impl From<u8> for Tile {
    fn from(byte: u8) -> Self {
        match byte {
            b'#' => Tile::Wall,
            // Mark the robot position as an open tile, we store the position separately.
            b'.' | b'@' => Tile::Open,
            b'O' => Tile::Box,
            _ => panic!("Invalid tile: {}", byte as char),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Wall => f.write_char('#'),
            Tile::Open => f.write_char('.'),
            Tile::Box => f.write_char('O'),
        }
    }
}

/// A warehouse with a robot and boxes.
#[derive(Debug)]
struct Warehouse {
    /// The map of the warehouse.
    map: Matrix<Tile>,
    /// The position of the robot.
    robot: Position,
}

impl Warehouse {
    /// Apply the given moves to the robot which may move boxes.
    fn apply_moves(&mut self, moves: &Moves) {
        for &direction in moves {
            let mut has_open_pos = false;
            let mut boxes_to_move = Vec::new();

            for next_pos in self
                .map
                .positions_in_direction(self.robot, direction.into())
            {
                match self.map[next_pos] {
                    Tile::Wall => break,
                    Tile::Open => {
                        has_open_pos = true;
                        break;
                    }
                    Tile::Box => {
                        boxes_to_move.push(next_pos);
                    }
                }
            }

            if !has_open_pos {
                continue;
            }

            while let Some(box_pos) = boxes_to_move.pop() {
                let neighbor = box_pos.neighbor(direction.into()).unwrap();
                self.map[neighbor] = Tile::Box;
                self.map[box_pos] = Tile::Open;
            }

            self.robot = self.robot.neighbor(direction.into()).unwrap();
        }
    }

    /// Converts the warehouse to the wide version where each tile is twice as wide.
    fn to_wide(&self) -> WideWarehouse {
        WideWarehouse {
            map: Matrix::from_iter(
                self.map.nrows(),
                self.map.ncols() * 2,
                self.map.as_slice().iter().flat_map(|tile| match tile {
                    Tile::Wall => [ScaledTile::Wall, ScaledTile::Wall],
                    Tile::Open => [ScaledTile::Open, ScaledTile::Open],
                    Tile::Box => [
                        ScaledTile::Box(BoxEdge::Open),
                        ScaledTile::Box(BoxEdge::Close),
                    ],
                }),
            ),
            robot: self.robot.add_col(self.robot.col()),
        }
    }

    /// Returns the sum of the GPS coordinates of the boxes in the warehouse.
    fn sum_gps_coordinates(&self) -> usize {
        self.map
            .enumerate()
            .filter(|(_, tile)| matches!(tile, Tile::Box))
            .map(|(pos, _)| pos.row() * 100 + pos.col())
            .sum()
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{}", self.map);
        let idx = self.robot.col() + self.robot.row() * self.map.ncols();
        s.replace_range(idx..=idx, "@");
        write!(f, "{s}")
    }
}

impl FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (row, line) in s.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                if byte == b'@' {
                    return Ok(Self {
                        map: Matrix::from_iter(
                            s.lines().count(),
                            s.lines()
                                .next()
                                .ok_or_else(|| anyhow!("Empty input"))?
                                .len(),
                            s.lines().flat_map(|line| line.bytes().map(Tile::from)),
                        ),
                        robot: Position::new(row, col),
                    });
                }
            }
        }

        Err(anyhow!("No robot (@) found in the input"))
    }
}

/// A box edge in the wide warehouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxEdge {
    Open,
    Close,
}

/// A scaled tile for the wide warehouse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScaledTile {
    Wall,
    Open,
    Box(BoxEdge),
}

impl fmt::Display for ScaledTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScaledTile::Wall => f.write_char('#'),
            ScaledTile::Open => f.write_char('.'),
            ScaledTile::Box(BoxEdge::Open) => f.write_char('['),
            ScaledTile::Box(BoxEdge::Close) => f.write_char(']'),
        }
    }
}

/// A wide warehouse with a robot and boxes.
#[derive(Debug)]
struct WideWarehouse {
    /// The map of the warehouse.
    map: Matrix<ScaledTile>,
    /// The position of the robot.
    robot: Position,
}

impl WideWarehouse {
    /// Apply the given moves to the robot which may move the boxes.
    fn apply_moves(&mut self, moves: &Moves) {
        for &direction in moves {
            if direction.is_horizontal() {
                self.apply_horizontal_move(direction);
            } else {
                self.apply_vertical_move(direction);
            }
        }
    }

    /// Apply a horizontal move to the robot which may move the boxes.
    ///
    /// # Panics
    ///
    /// Panics if the given direction is not horizontal (left or right).
    fn apply_horizontal_move(&mut self, direction: CardinalDirection) {
        assert!(direction.is_horizontal());

        let mut boxes_to_move = Vec::new();
        for next_pos in self
            .map
            .positions_in_direction(self.robot, direction.into())
        {
            match self.map[next_pos] {
                ScaledTile::Wall => return,
                ScaledTile::Open => break,
                ScaledTile::Box(_) => boxes_to_move.push(next_pos),
            }
        }

        while let Some(box_pos) = boxes_to_move.pop() {
            let neighbor = box_pos.neighbor(direction.into()).unwrap();
            self.map[neighbor] = self.map[box_pos];
            self.map[box_pos] = ScaledTile::Open;
        }

        self.robot = self.robot.neighbor(direction.into()).unwrap();
    }

    /// Apply a vertical move to the robot which may move the boxes.
    ///
    /// # Panics
    ///
    /// Panics if the given direction is not vertical (up or down).
    fn apply_vertical_move(&mut self, direction: CardinalDirection) {
        assert!(direction.is_vertical());

        let mut boxes_to_move = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(self.robot);

        // Use BFS (Breadth-First Search) to find all the boxes that needs to be moved.
        while let Some(position) = queue.pop_front() {
            let next_position = position.neighbor(direction.into()).expect("out of bounds");
            match self.map[next_position] {
                ScaledTile::Wall => return,
                ScaledTile::Open => continue,
                ScaledTile::Box(_) if boxes_to_move.contains(&next_position) => continue,
                ScaledTile::Box(box_edge) => {
                    boxes_to_move.push(next_position);
                    queue.push_back(next_position);
                    let neighbor = match box_edge {
                        BoxEdge::Open => next_position.add_col(1),
                        BoxEdge::Close => next_position.sub_col(1),
                    };
                    if !boxes_to_move.contains(&neighbor) {
                        boxes_to_move.push(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        while let Some(box_pos) = boxes_to_move.pop() {
            let neighbor = box_pos.neighbor(direction.into()).unwrap();
            self.map[neighbor] = self.map[box_pos];
            self.map[box_pos] = ScaledTile::Open;
        }

        self.robot = self.robot.neighbor(direction.into()).unwrap();
    }

    /// Returns the sum of the GPS coordinates of the boxes in the warehouse.
    fn sum_gps_coordinates(&self) -> usize {
        self.map
            .enumerate()
            .filter(|(_, tile)| matches!(tile, ScaledTile::Box(BoxEdge::Open)))
            .map(|(pos, _)| pos.row() * 100 + pos.col())
            .sum()
    }
}

impl fmt::Display for WideWarehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{}", self.map);
        let idx = self.robot.col() + self.robot.row() * self.map.ncols();
        s.replace_range(idx..=idx, "@");
        write!(f, "{s}")
    }
}

/// Parses the input into a warehouse and moves.
fn parse_input(input: &str) -> Result<(Warehouse, Moves)> {
    let (map_section, moves_section) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Expected two sections separated by two newlines"))?;

    Ok((
        Warehouse::from_str(map_section)?,
        Moves::from(moves_section),
    ))
}

pub fn solve(input: &str) -> Result<()> {
    let (mut warehouse, moves) = parse_input(input)?;
    let mut wide_warehouse = warehouse.to_wide();

    warehouse.apply_moves(&moves);
    println!("Part 1: {:?}", warehouse.sum_gps_coordinates());

    wide_warehouse.apply_moves(&moves);
    println!("Part 2: {:?}", wide_warehouse.sum_gps_coordinates());

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const SAMPLE_INPUT1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const SAMPLE_INPUT2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    const SAMPLE_INPUT3: &str = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    #[test_case(SAMPLE_INPUT1, 2028)]
    #[test_case(SAMPLE_INPUT2, 10092)]
    fn unscaled(input: &str, expected: usize) {
        let (mut warehouse, moves) = parse_input(input).unwrap();
        warehouse.apply_moves(&moves);
        assert_eq!(warehouse.sum_gps_coordinates(), expected);
    }

    #[test_case(SAMPLE_INPUT2, 9021)]
    #[test_case(SAMPLE_INPUT3, 618)]
    fn scaled(input: &str, expected: usize) {
        let (warehouse, moves) = parse_input(input).unwrap();
        let mut scaled_warehouse = warehouse.to_wide();
        scaled_warehouse.apply_moves(&moves);
        assert_eq!(scaled_warehouse.sum_gps_coordinates(), expected);
    }
}
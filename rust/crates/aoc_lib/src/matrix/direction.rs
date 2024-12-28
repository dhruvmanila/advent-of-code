/// The direction in which a position can move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Direction {
    /// All possible directions in a clockwise order starting from top-left.
    pub const ALL: [Direction; 8] = [
        Direction::TopLeft,
        Direction::Up,
        Direction::TopRight,
        Direction::Right,
        Direction::BottomRight,
        Direction::Down,
        Direction::BottomLeft,
        Direction::Left,
    ];

    /// The four cardinal directions: up, down, left, right.
    pub const CARDINAL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

/// The four cardinal directions: up, right, down, left.
///
/// This is a subset of [`Direction`] that only includes the four cardinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    Up,
    Right,
    Down,
    Left,
}

impl CardinalDirection {
    /// The north direction (up).
    pub const NORTH: CardinalDirection = CardinalDirection::Up;
    /// The east direction (right).
    pub const EAST: CardinalDirection = CardinalDirection::Right;
    /// The south direction (down).
    pub const SOUTH: CardinalDirection = CardinalDirection::Down;
    /// The west direction (left).
    pub const WEST: CardinalDirection = CardinalDirection::Left;

    /// All possible cardinal directions in a clockwise order starting from up.
    pub const ALL: [CardinalDirection; 4] = [
        CardinalDirection::Up,
        CardinalDirection::Right,
        CardinalDirection::Down,
        CardinalDirection::Left,
    ];

    /// Returns `true` if the direction is horizontal (right or left).
    #[inline]
    pub const fn is_horizontal(self) -> bool {
        matches!(self, CardinalDirection::Right | CardinalDirection::Left)
    }

    /// Returns `true` if the direction is vertical (up or down).
    #[inline]
    pub const fn is_vertical(self) -> bool {
        matches!(self, CardinalDirection::Up | CardinalDirection::Down)
    }

    /// Returns the direction that's 90 degrees to the left of the current direction.
    #[inline]
    #[must_use]
    pub const fn turn_left(self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Left,
            CardinalDirection::Left => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Up,
        }
    }

    /// Returns the direction that's 90 degrees to the right of the current direction.
    #[inline]
    #[must_use]
    pub const fn turn_right(self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Left,
            CardinalDirection::Left => CardinalDirection::Up,
        }
    }
}

impl From<CardinalDirection> for Direction {
    fn from(cardinal: CardinalDirection) -> Self {
        match cardinal {
            CardinalDirection::Up => Direction::Up,
            CardinalDirection::Right => Direction::Right,
            CardinalDirection::Down => Direction::Down,
            CardinalDirection::Left => Direction::Left,
        }
    }
}

use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::{CardinalDirection, Direction};

/// A position in a [`Matrix`](crate::matrix::Matrix).
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    /// The row value of this position.
    pub(super) row: usize,
    /// The column value of this position.
    pub(super) col: usize,
}

impl Position {
    /// Create a new position from the given `row` and `col`.
    #[inline]
    pub const fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    /// Create a new position at the origin.
    ///
    /// The origin for a [`Matrix`](crate::matrix::Matrix) is the top-left
    /// corner which is at `(0, 0)`.
    #[inline]
    pub const fn zero() -> Position {
        Position::new(0, 0)
    }

    /// Return the row value of this position.
    #[inline]
    pub const fn row(&self) -> usize {
        self.row
    }

    /// Return the column value of this position.
    #[inline]
    pub const fn col(&self) -> usize {
        self.col
    }

    /// Return the position as a tuple of `(row, col)`.
    #[inline]
    pub const fn as_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Adds an offset to the row value of this position.
    ///
    /// # Panics
    ///
    /// If `self.row + amount > usize::MAX`.
    #[inline]
    #[must_use]
    pub const fn add_row(&self, amount: usize) -> Position {
        Position::new(self.row + amount, self.col)
    }

    /// Checked row addition. Computes `self.row + amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub const fn checked_add_row(&self, amount: usize) -> Option<Position> {
        if let Some(row) = self.row.checked_add(amount) {
            Some(Position::new(row, self.col))
        } else {
            None
        }
    }

    /// Saturating row addition. Computes `self.row + amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    #[must_use]
    pub const fn saturating_add_row(&self, amount: usize) -> Position {
        Position::new(self.row.saturating_add(amount), self.col)
    }

    /// Subtracts an offset from the row value of this position.
    ///
    /// # Panics
    ///
    /// If `self.row - amount` is less than zero.
    #[inline]
    #[must_use]
    pub const fn sub_row(&self, amount: usize) -> Position {
        Position::new(self.row - amount, self.col)
    }

    /// Checked row subtraction. Computes `self.row - amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub const fn checked_sub_row(&self, amount: usize) -> Option<Position> {
        if let Some(row) = self.row.checked_sub(amount) {
            Some(Position::new(row, self.col))
        } else {
            None
        }
    }

    /// Saturating row subtraction. Computes `self.row - amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    #[must_use]
    pub const fn saturating_sub_row(&self, amount: usize) -> Position {
        Position::new(self.row.saturating_sub(amount), self.col)
    }

    /// Adds an offset to the column value of this position.
    ///
    /// # Panics
    ///
    /// If `self.col + amount > usize::MAX`.
    #[inline]
    #[must_use]
    pub const fn add_col(&self, amount: usize) -> Position {
        Position::new(self.row, self.col + amount)
    }

    /// Checked column addition. Computes `self.col + amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub const fn checked_add_col(&self, amount: usize) -> Option<Position> {
        if let Some(col) = self.col.checked_add(amount) {
            Some(Position::new(self.row, col))
        } else {
            None
        }
    }

    /// Saturating column addition. Computes `self.col + amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    #[must_use]
    pub const fn saturating_add_col(&self, amount: usize) -> Position {
        Position::new(self.row, self.col.saturating_add(amount))
    }

    /// Subtracts an offset from the column value of this position.
    ///
    /// # Panics
    ///
    /// If `self.col - amount` is less than zero.
    #[must_use]
    pub const fn sub_col(&self, amount: usize) -> Position {
        Position::new(self.row, self.col - amount)
    }

    /// Checked column subtraction. Computes `self.col - amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub const fn checked_sub_col(&self, amount: usize) -> Option<Position> {
        if let Some(col) = self.col.checked_sub(amount) {
            Some(Position::new(self.row, col))
        } else {
            None
        }
    }

    /// Saturating column subtraction. Computes `self.col - amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    #[must_use]
    pub const fn saturating_sub_col(&self, amount: usize) -> Position {
        Position::new(self.row, self.col.saturating_sub(amount))
    }

    /// Returns the manhattan distance between self and the other position.
    #[inline]
    pub const fn manhattan_distance(&self, other: &Position) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    /// Returns an iterator over the neighboring positions of [`Self`] filtering
    /// out the ones that are out of bounds.
    ///
    /// Use [`neighbor`] if you want to get the neighboring position in a specific direction.
    ///
    /// [`neighbor`]: Position::neighbor
    #[inline]
    pub fn neighbors(&self) -> impl Iterator<Item = Position> + '_ {
        Direction::ALL
            .iter()
            .filter_map(|direction| self.neighbor(*direction))
    }

    /// Returns an iterator over the cardinal neighboring positions of [`Self`] filtering
    /// out the ones that are out of bounds.
    ///
    /// Use [`neighbor`] if you want to get the neighboring position in a specific direction.
    ///
    /// [`neighbor`]: Position::neighbor
    #[inline]
    pub fn cardinal_neighbors(&self) -> impl Iterator<Item = Position> + '_ {
        Direction::CARDINAL
            .iter()
            .filter_map(|direction| self.neighbor(*direction))
    }

    /// Returns the neighboring position in the given direction, [`None`] if the position is out of
    /// bounds.
    #[inline]
    pub const fn neighbor(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::TopLeft => self.top_left(),
            Direction::TopRight => self.top_right(),
            Direction::BottomLeft => self.bottom_left(),
            Direction::BottomRight => self.bottom_right(),
        }
    }

    /// Returns the position that's above the current position, [`None`] if the row is 0.
    #[inline]
    pub const fn up(&self) -> Option<Position> {
        self.checked_sub_row(1)
    }

    /// Returns the position that's below the current position, [`None`] if the row is at the
    /// [`usize::MAX`] value.
    #[inline]
    pub const fn down(&self) -> Option<Position> {
        self.checked_add_row(1)
    }

    /// Returns the position that's to the left of the current position, [`None`] if the column is
    /// 0.
    #[inline]
    pub const fn left(&self) -> Option<Position> {
        self.checked_sub_col(1)
    }

    /// Returns the position that's to the right of the current position, [`None`] if the column is
    /// at the [`usize::MAX`] value.
    #[inline]
    pub const fn right(&self) -> Option<Position> {
        self.checked_add_col(1)
    }

    /// Returns the position that's to the top-left of the current position, [`None`] if the row
    /// or column is 0.
    #[inline]
    pub const fn top_left(&self) -> Option<Position> {
        match (self.row.checked_sub(1), self.col.checked_sub(1)) {
            (Some(row), Some(col)) => Some(Position::new(row, col)),
            _ => None,
        }
    }

    /// Returns the position that's to the top-right of the current position, [`None`] if the row
    /// is 0 or the column is at the [`usize::MAX`] value.
    #[inline]
    pub const fn top_right(&self) -> Option<Position> {
        match (self.row.checked_sub(1), self.col.checked_add(1)) {
            (Some(row), Some(col)) => Some(Position::new(row, col)),
            _ => None,
        }
    }

    /// Returns the position that's to the bottom-left of the current position, [`None`] if the row
    /// is at the [`usize::MAX`] value or the column is 0.
    #[inline]
    pub const fn bottom_left(&self) -> Option<Position> {
        match (self.row.checked_add(1), self.col.checked_sub(1)) {
            (Some(row), Some(col)) => Some(Position::new(row, col)),
            _ => None,
        }
    }

    /// Returns the position that's to the bottom-right of the current position, [`None`] if the
    /// row or column is at the [`usize::MAX`] value.
    #[inline]
    pub const fn bottom_right(&self) -> Option<Position> {
        match (self.row.checked_add(1), self.col.checked_add(1)) {
            (Some(row), Some(col)) => Some(Position::new(row, col)),
            _ => None,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    #[inline]
    fn add(self, rhs: Direction) -> Position {
        self.neighbor(rhs)
            .expect("adding direction to position should not overflow")
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Add<CardinalDirection> for Position {
    type Output = Position;

    fn add(self, rhs: CardinalDirection) -> Position {
        self + Direction::from(rhs)
    }
}

impl AddAssign<CardinalDirection> for Position {
    fn add_assign(&mut self, rhs: CardinalDirection) {
        *self = *self + rhs;
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Position) {
        *self = *self - rhs;
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.row).field(&self.col).finish()
    }
}

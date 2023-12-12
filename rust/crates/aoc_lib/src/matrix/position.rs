use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A position in a [`Matrix`](crate::matrix::Matrix).
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    /// The row value of this position.
    row: usize,
    /// The column value of this position.
    col: usize,
}

impl Position {
    /// Create a new position from the given `row` and `col`.
    #[inline]
    pub const fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Create a new position at the origin.
    ///
    /// The origin for a [`Matrix`](crate::matrix::Matrix) is the top-left
    /// corner which is at `(0, 0)`.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0, 0)
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
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.add_row(1), Position::new(2, 1));
    /// ```
    #[inline]
    pub const fn add_row(&self, amount: usize) -> Position {
        Position::new(self.row() + amount, self.col())
    }

    /// Checked row addition. Computes `self.row + amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub fn checked_add_row(&self, amount: usize) -> Option<Position> {
        self.row
            .checked_add(amount)
            .map(|row| Position::new(row, self.col()))
    }

    /// Saturating row addition. Computes `self.row + amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_add_row(&self, amount: usize) -> Position {
        Position::new(self.row().saturating_add(amount), self.col())
    }

    /// Subtracts an offset from the row value of this position.
    ///
    /// # Panics
    ///
    /// If `self.row - amount` is less than zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.sub_row(1), Position::new(0, 1));
    /// ```
    #[inline]
    pub const fn sub_row(&self, amount: usize) -> Position {
        Position::new(self.row() - amount, self.col())
    }

    /// Checked row subtraction. Computes `self.row - amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub fn checked_sub_row(&self, amount: usize) -> Option<Position> {
        self.row
            .checked_sub(amount)
            .map(|row| Position::new(row, self.col()))
    }

    /// Saturating row subtraction. Computes `self.row - amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_sub_row(&self, amount: usize) -> Position {
        Position::new(self.row().saturating_sub(amount), self.col())
    }

    /// Adds an offset to the column value of this position.
    ///
    /// # Panics
    ///
    /// If `self.col + amount > usize::MAX`.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.add_col(1), Position::new(1, 2));
    /// ```
    #[inline]
    pub const fn add_col(&self, amount: usize) -> Position {
        Position::new(self.row(), self.col() + amount)
    }

    /// Checked column addition. Computes `self.col + amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub fn checked_add_col(&self, amount: usize) -> Option<Position> {
        self.col
            .checked_add(amount)
            .map(|col| Position::new(self.row(), col))
    }

    /// Saturating column addition. Computes `self.col + amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_add_col(&self, amount: usize) -> Position {
        Position::new(self.row(), self.col().saturating_add(amount))
    }

    /// Subtracts an offset from the column value of this position.
    ///
    /// # Panics
    ///
    /// If `self.col - amount` is less than zero.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.sub_col(1), Position::new(1, 0));
    /// ```
    #[inline]
    pub const fn sub_col(&self, amount: usize) -> Position {
        Position::new(self.row(), self.col() - amount)
    }

    /// Checked column subtraction. Computes `self.col - amount`, returning `None`
    /// if overflow occurred.
    #[inline]
    pub fn checked_sub_col(&self, amount: usize) -> Option<Position> {
        self.col
            .checked_sub(amount)
            .map(|col| Position::new(self.row(), col))
    }

    /// Saturating column subtraction. Computes `self.col - amount`, saturating
    /// at the numeric bounds instead of overflowing.
    #[inline]
    pub const fn saturating_sub_col(&self, amount: usize) -> Position {
        Position::new(self.row(), self.col().saturating_sub(amount))
    }

    /// Returns an iterator over the neighboring positions of [`Self`] filtering
    /// out the ones that are out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// let mut neighbors_iter = position.neighbors();
    ///
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 0)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 2)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 2)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 0)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 2)));
    /// assert_eq!(neighbors_iter.next(), None);
    /// ```
    ///
    /// Neighboring positions that are out of bounds are filtered out:
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(0, 0);
    /// let mut neighbors_iter = position.neighbors();
    ///
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 1)));
    /// assert_eq!(neighbors_iter.next(), None);
    /// ```
    pub fn neighbors(&self) -> impl Iterator<Item = Position> {
        neighbors_impl(self.row, self.col)
    }
}

/// Implementation of [`neighbors`] method on [`Position`] that takes the
/// row and column values as arguments.
///
/// [`neighbors`]: Position#method.neighbors
fn neighbors_impl(row: usize, col: usize) -> impl Iterator<Item = Position> {
    (row.saturating_sub(1)..=row.saturating_add(1))
        .flat_map(move |row| {
            (col.saturating_sub(1)..=col.saturating_add(1)).map(move |col| Position::new(row, col))
        })
        .filter(move |position| position.row() != row || position.col() != col)
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

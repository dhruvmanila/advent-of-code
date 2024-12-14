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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub const fn saturating_sub_col(&self, amount: usize) -> Position {
        Position::new(self.row(), self.col().saturating_sub(amount))
    }

    /// Returns the manhattan distance between self and the other position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position1 = Position::new(0, 0);
    /// let position2 = Position::new(3, 4);
    ///
    /// assert_eq!(position1.manhattan_distance(&position2), 7);
    /// ```
    #[inline]
    pub const fn manhattan_distance(&self, other: &Position) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    /// Returns an iterator over the neighboring positions of [`Self`] filtering
    /// out the ones that are out of bounds.
    ///
    /// Use [`neighbor`] if you want to get the neighboring position in a specific direction.
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
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 2)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 2)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 0)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
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
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
    /// assert_eq!(neighbors_iter.next(), None);
    /// ```
    ///
    /// [`neighbor`]: Position::neighbor
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
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// let mut neighbors_iter = position.cardinal_neighbors();
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 2)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(2, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
    /// ```
    ///
    /// Neighboring positions that are out of bounds are filtered out:
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(0, 0);
    /// let mut neighbors_iter = position.cardinal_neighbors();
    ///
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(0, 1)));
    /// assert_eq!(neighbors_iter.next(), Some(Position::new(1, 0)));
    /// assert_eq!(neighbors_iter.next(), None);
    /// ```
    ///
    /// [`neighbor`]: Position::neighbor
    pub fn cardinal_neighbors(&self) -> impl Iterator<Item = Position> + '_ {
        Direction::CARDINAL
            .iter()
            .filter_map(|direction| self.neighbor(*direction))
    }

    /// Returns the neighboring position in the given direction, [`None`] if the position is out of
    /// bounds.
    pub fn neighbor(&self, direction: Direction) -> Option<Position> {
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
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.up(), Some(Position::new(0, 1)));
    ///
    /// let position = Position::new(0, 1);
    /// assert_eq!(position.up(), None);
    /// ```
    pub fn up(&self) -> Option<Position> {
        Some(Position::new(self.row.checked_sub(1)?, self.col))
    }

    /// Returns the position that's below the current position, [`None`] if the row is at the
    /// [`usize::MAX`] value.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.down(), Some(Position::new(2, 1)));
    ///
    /// let position = Position::new(usize::MAX, 1);
    /// assert_eq!(position.down(), None);
    /// ```
    pub fn down(&self) -> Option<Position> {
        Some(Position::new(self.row.checked_add(1)?, self.col))
    }

    /// Returns the position that's to the left of the current position, [`None`] if the column is
    /// 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.left(), Some(Position::new(1, 0)));
    ///
    /// let position = Position::new(1, 0);
    /// assert_eq!(position.left(), None);
    /// ```
    pub fn left(&self) -> Option<Position> {
        Some(Position::new(self.row, self.col.checked_sub(1)?))
    }

    /// Returns the position that's to the right of the current position, [`None`] if the column is
    /// at the [`usize::MAX`] value.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.right(), Some(Position::new(1, 2)));
    ///
    /// let position = Position::new(1, usize::MAX);
    /// assert_eq!(position.right(), None);
    /// ```
    pub fn right(&self) -> Option<Position> {
        Some(Position::new(self.row, self.col.checked_add(1)?))
    }

    /// Returns the position that's to the top-left of the current position, [`None`] if the row
    /// or column is 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.top_left(), Some(Position::new(0, 0)));
    ///
    /// let position = Position::new(0, 1);
    /// assert_eq!(position.top_left(), None);
    ///
    /// let position = Position::new(1, 0);
    /// assert_eq!(position.top_left(), None);
    /// ```
    pub fn top_left(&self) -> Option<Position> {
        Some(Position::new(
            self.row.checked_sub(1)?,
            self.col.checked_sub(1)?,
        ))
    }

    /// Returns the position that's to the top-right of the current position, [`None`] if the row
    /// is 0 or the column is at the [`usize::MAX`] value.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.top_right(), Some(Position::new(0, 2)));
    ///
    /// let position = Position::new(0, 1);
    /// assert_eq!(position.top_right(), None);
    ///
    /// let position = Position::new(1, usize::MAX);
    /// assert_eq!(position.top_right(), None);
    /// ```
    pub fn top_right(&self) -> Option<Position> {
        Some(Position::new(
            self.row.checked_sub(1)?,
            self.col.checked_add(1)?,
        ))
    }

    /// Returns the position that's to the bottom-left of the current position, [`None`] if the row
    /// is at the [`usize::MAX`] value or the column is 0.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.bottom_left(), Some(Position::new(2, 0)));
    ///
    /// let position = Position::new(usize::MAX, 1);
    /// assert_eq!(position.bottom_left(), None);
    ///
    /// let position = Position::new(1, 0);
    /// assert_eq!(position.bottom_left(), None);
    /// ```
    pub fn bottom_left(&self) -> Option<Position> {
        Some(Position::new(
            self.row.checked_add(1)?,
            self.col.checked_sub(1)?,
        ))
    }

    /// Returns the position that's to the bottom-right of the current position, [`None`] if the
    /// row or column is at the [`usize::MAX`] value.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Position;
    /// let position = Position::new(1, 1);
    /// assert_eq!(position.bottom_right(), Some(Position::new(2, 2)));
    ///
    /// let position = Position::new(usize::MAX, 1);
    /// assert_eq!(position.bottom_right(), None);
    ///
    /// let position = Position::new(1, usize::MAX);
    /// assert_eq!(position.bottom_right(), None);
    /// ```
    pub fn bottom_right(&self) -> Option<Position> {
        Some(Position::new(
            self.row.checked_add(1)?,
            self.col.checked_add(1)?,
        ))
    }
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

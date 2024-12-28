use std::iter::FusedIterator;

use super::{Direction, Matrix, Position, Vector};

/// An iterator over the rows of a [`Matrix`].
///
/// This struct is created by the [`rows`] method on [`Matrix`]. See its
/// documentation for more.
///
/// [`rows`]: Matrix::rows
pub struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    current: usize,
}

impl<'a, T> RowIter<'a, T> {
    pub(super) fn new(matrix: &'a Matrix<T>) -> Self {
        Self { matrix, current: 0 }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = Vector<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.matrix.row(self.current) {
            self.current += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.matrix.nrows() - self.current;
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for RowIter<'_, T> {}
impl<T> FusedIterator for RowIter<'_, T> {}

/// An iterator over the columns of a [`Matrix`].
///
/// This struct is created by the [`columns`] method on [`Matrix`]. See its
/// documentation for more.
///
/// [`columns`]: Matrix::columns
pub struct ColumnIter<'a, T> {
    matrix: &'a Matrix<T>,
    current: usize,
}

impl<'a, T> ColumnIter<'a, T> {
    pub(super) fn new(matrix: &'a Matrix<T>) -> Self {
        Self { matrix, current: 0 }
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = Vector<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(column) = self.matrix.column(self.current) {
            self.current += 1;
            Some(column)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.matrix.ncols() - self.current;
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for ColumnIter<'_, T> {}
impl<T> FusedIterator for ColumnIter<'_, T> {}

/// An iterator that yields the position and value of each cell in the [`Matrix`]
/// during iteration.
///
/// This struct is created by the [`enumerate`] method on [`Matrix`]. See its
/// documentation for more.
///
/// [`enumerate`]: Matrix::enumerate
pub struct MatrixEnumerate<'a, T> {
    matrix: &'a Matrix<T>,
    current: Position,
}

impl<'a, T> MatrixEnumerate<'a, T> {
    pub(super) fn new(matrix: &'a Matrix<T>) -> Self {
        Self {
            matrix,
            current: Position::new(0, 0),
        }
    }
}

impl<'a, T> Iterator for MatrixEnumerate<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.row >= self.matrix.nrows() {
            return None;
        }

        let item = (self.current, &self.matrix[self.current]);

        self.current = self.current.add_col(1);
        if self.current.col == self.matrix.ncols() {
            self.current = Position::new(self.current.row + 1, 0);
        }

        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.matrix.nrows() - self.current.row - 1) * self.matrix.ncols()
            + (self.matrix.ncols() - self.current.col);
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for MatrixEnumerate<'_, T> {}
impl<T> FusedIterator for MatrixEnumerate<'_, T> {}

/// An iterator that yields the position of each cell in the [`Matrix`] along a specific
/// direction excluding the starting position.
///
/// This struct is created by the [`positions_in_direction`] method on [`Matrix`]. See its
/// documentation for more.
///
/// [`positions_in_direction`]: Matrix::positions_in_direction
pub struct PositionsInDirectionIter {
    dimension: (usize, usize),
    current: Position,
    direction: Direction,
}

impl PositionsInDirectionIter {
    pub(super) fn new(dimension: (usize, usize), start: Position, direction: Direction) -> Self {
        Self {
            dimension,
            current: start,
            direction,
        }
    }

    /// Returns `true` if the current position is out of bounds of the matrix.
    fn is_out_of_bounds(&self) -> bool {
        self.current.row >= self.dimension.0 || self.current.col >= self.dimension.1
    }
}

impl Iterator for PositionsInDirectionIter {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_out_of_bounds() {
            return None;
        }
        self.current = self.current.neighbor(self.direction)?;
        Some(self.current)
    }
}

impl FusedIterator for PositionsInDirectionIter {}

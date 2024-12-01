use std::iter::FusedIterator;

use crate::matrix::position::Position;
use crate::matrix::{Matrix, Vector};

/// An iterator over the rows of a [`Matrix`].
///
/// This struct is created by the [`rows`] method on [`Matrix`]. See its
/// documentation for more.
///
/// [`rows`]: Matrix#method.rows
pub struct RowIter<'a, T> {
    mat: &'a Matrix<T>,
    curr: usize,
}

impl<'a, T> RowIter<'a, T> {
    pub(crate) fn new(mat: &'a Matrix<T>) -> Self {
        Self { mat, curr: 0 }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = Vector<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.mat.row(self.curr) {
            self.curr += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.mat.nrows() - self.curr;
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
/// [`columns`]: Matrix#method.columns
pub struct ColumnIter<'a, T> {
    mat: &'a Matrix<T>,
    curr: usize,
}

impl<'a, T> ColumnIter<'a, T> {
    pub(crate) fn new(mat: &'a Matrix<T>) -> Self {
        Self { mat, curr: 0 }
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = Vector<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(column) = self.mat.column(self.curr) {
            self.curr += 1;
            Some(column)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.mat.ncols() - self.curr;
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
/// [`enumerate`]: Matrix#method.enumerate
pub struct MatrixEnumerate<'a, T> {
    mat: &'a Matrix<T>,
    curr: Position,
}

impl<'a, T> MatrixEnumerate<'a, T> {
    pub(crate) fn new(matrix: &'a Matrix<T>) -> Self {
        Self {
            mat: matrix,
            curr: Position::new(0, 0),
        }
    }
}

impl<'a, T> Iterator for MatrixEnumerate<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.row() >= self.mat.nrows() {
            return None;
        }

        let item = (self.curr, &self.mat[self.curr]);

        self.curr = self.curr.add_col(1);
        if self.curr.col() == self.mat.ncols() {
            self.curr = Position::new(self.curr.row() + 1, 0);
        }

        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.mat.nrows() - self.curr.row() - 1) * self.mat.ncols()
            + (self.mat.ncols() - self.curr.col());
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for MatrixEnumerate<'_, T> {}
impl<T> FusedIterator for MatrixEnumerate<'_, T> {}

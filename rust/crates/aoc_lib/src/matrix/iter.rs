use std::iter::FusedIterator;
use std::marker::PhantomData;

use super::position::Position;
use super::vector::{ColumnVector, ColumnVectorMut, RowVector, RowVectorMut};
use super::Matrix;

/// An iterator over the rows of a [`Matrix`].
///
/// This struct is created by the [`rows`] method on [`Matrix`]. See its documentation for more.
///
/// [`rows`]: Matrix::rows
pub struct RowIter<'a, T> {
    matrix: &'a Matrix<T>,
    // The start and end indices are separated to allow for double-ended iteration.
    start: usize,
    end: usize,
}

impl<'a, T> RowIter<'a, T> {
    pub(super) const fn new(matrix: &'a Matrix<T>) -> RowIter<'a, T> {
        RowIter {
            matrix,
            start: 0,
            end: matrix.nrows(),
        }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = RowVector<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<RowVector<'a, T>> {
        if self.start < self.end {
            let row = self.matrix.row(self.start);
            self.start += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'a, T> DoubleEndedIterator for RowIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<RowVector<'a, T>> {
        if self.start < self.end {
            self.end -= 1;
            let row = self.matrix.row(self.end);
            Some(row)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for RowIter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<T> FusedIterator for RowIter<'_, T> {}

/// An iterator over the mutable rows of a [`Matrix`].
///
/// This struct is created by the [`rows_mut`] method on [`Matrix`]. See its documentation for more.
///
/// [`rows_mut`]: Matrix::rows_mut
pub struct RowIterMut<'a, T> {
    matrix: *mut Matrix<T>,
    // The start and end indices are separated to allow for double-ended iteration.
    start: usize,
    end: usize,
    _marker: PhantomData<&'a Matrix<T>>,
}

impl<'a, T> RowIterMut<'a, T> {
    pub(super) const fn new(matrix: &'a mut Matrix<T>) -> RowIterMut<'a, T> {
        RowIterMut {
            matrix,
            start: 0,
            end: matrix.nrows(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for RowIterMut<'a, T> {
    type Item = RowVectorMut<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<RowVectorMut<'a, T>> {
        if self.start < self.end {
            let row = unsafe { (*self.matrix).row_mut(self.start) };
            self.start += 1;
            Some(row)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'a, T> DoubleEndedIterator for RowIterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<RowVectorMut<'a, T>> {
        if self.start < self.end {
            self.end -= 1;
            let row = unsafe { (*self.matrix).row_mut(self.end) };
            Some(row)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for RowIterMut<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<T> FusedIterator for RowIterMut<'_, T> {}

/// An iterator over the columns of a [`Matrix`].
///
/// This struct is created by the [`columns`] method on [`Matrix`]. See its documentation for more.
///
/// [`columns`]: Matrix::columns
pub struct ColumnIter<'a, T> {
    matrix: &'a Matrix<T>,
    start: usize,
    end: usize,
}

impl<'a, T> ColumnIter<'a, T> {
    pub(super) const fn new(matrix: &'a Matrix<T>) -> ColumnIter<'a, T> {
        ColumnIter {
            matrix,
            start: 0,
            end: matrix.ncols(),
        }
    }
}

impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = ColumnVector<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<ColumnVector<'a, T>> {
        if self.start < self.end {
            let column = self.matrix.column(self.start);
            self.start += 1;
            Some(column)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<ColumnVector<'a, T>> {
        if self.start < self.end {
            self.end -= 1;
            let column = self.matrix.column(self.end);
            Some(column)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for ColumnIter<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<T> FusedIterator for ColumnIter<'_, T> {}

/// An iterator over the columns of a [`Matrix`].
///
/// This struct is created by the [`columns`] method on [`Matrix`]. See its documentation for more.
///
/// [`columns`]: Matrix::columns
pub struct ColumnIterMut<'a, T> {
    matrix: *mut Matrix<T>,
    start: usize,
    end: usize,
    _marker: PhantomData<&'a Matrix<T>>,
}

impl<'a, T> ColumnIterMut<'a, T> {
    pub(super) const fn new(matrix: &'a mut Matrix<T>) -> ColumnIterMut<'a, T> {
        ColumnIterMut {
            matrix,
            start: 0,
            end: matrix.ncols(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for ColumnIterMut<'a, T> {
    type Item = ColumnVectorMut<'a, T>;

    #[inline]
    fn next(&mut self) -> Option<ColumnVectorMut<'a, T>> {
        if self.start < self.end {
            let column = unsafe { (*self.matrix).column_mut(self.start) };
            self.start += 1;
            Some(column)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.size_hint().0
    }
}

impl<'a, T> DoubleEndedIterator for ColumnIterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<ColumnVectorMut<'a, T>> {
        if self.start < self.end {
            self.end -= 1;
            let column = unsafe { (*self.matrix).column_mut(self.end) };
            Some(column)
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for ColumnIterMut<'_, T> {
    #[inline]
    fn len(&self) -> usize {
        self.size_hint().0
    }
}

impl<T> FusedIterator for ColumnIterMut<'_, T> {}

/// An iterator that yields the position and value of each cell in the [`Matrix`] during iteration.
///
/// This struct is created by the [`enumerate`] method on [`Matrix`]. See its documentation for
/// more.
///
/// [`enumerate`]: Matrix::enumerate
pub struct MatrixEnumerate<'a, T> {
    matrix: &'a Matrix<T>,
    current: Position,
}

impl<'a, T> MatrixEnumerate<'a, T> {
    pub(super) fn new(matrix: &'a Matrix<T>) -> MatrixEnumerate<'a, T> {
        MatrixEnumerate {
            matrix,
            current: Position::zero(),
        }
    }
}

impl<'a, T> Iterator for MatrixEnumerate<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<(Position, &'a T)> {
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

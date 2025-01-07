use std::ops::RangeFull;

use super::position::Position;
use super::vector::{ColumnVector, ColumnVectorMut, RowVector, RowVectorMut, Vector, VectorMut};
use super::Matrix;

/// A helper trait for indexing into a [`Matrix`].
pub trait MatrixIndex<'a, T>: Sized {
    /// The output type of the index operation.
    type Output: 'a;

    /// Returns `true` if this index is contained by the matrix.
    fn contained_by(&self, matrix: &Matrix<T>) -> bool;

    /// Returns a reference to the element at this index without checking bounds.
    unsafe fn get_unchecked(self, matrix: &'a Matrix<T>) -> Self::Output;

    /// Returns a reference to the element at this index, or [`None`] if the index is out of
    /// bounds.
    fn get(self, matrix: &'a Matrix<T>) -> Option<Self::Output> {
        if self.contained_by(matrix) {
            unsafe { Some(self.get_unchecked(matrix)) }
        } else {
            None
        }
    }

    /// Returns a reference to the element at this index, or panics if the index is out of bounds.
    fn index(self, matrix: &'a Matrix<T>) -> Self::Output {
        self.get(matrix).expect("index out of bounds")
    }
}

/// A helper trait for indexing into a [`Matrix`] with mutable references.
pub trait MatrixIndexMut<'a, T>: MatrixIndex<'a, T> {
    /// The mutable output type of the index operation.
    type OutputMut: 'a;

    /// Returns a mutable reference to the element at this index without checking bounds.
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T>) -> Self::OutputMut;

    /// Returns a mutable reference to the element at this index, or [`None`] if the index is out
    /// of bounds.
    fn get_mut(self, matrix: &'a mut Matrix<T>) -> Option<Self::OutputMut> {
        if self.contained_by(matrix) {
            unsafe { Some(self.get_unchecked_mut(matrix)) }
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at this index, or panics if the index is out of
    /// bounds.
    fn index_mut(self, matrix: &'a mut Matrix<T>) -> Self::OutputMut {
        self.get_mut(matrix).expect("index out of bounds")
    }
}

impl<T> Matrix<T> {
    /// Returns a reference to the value at given index.
    ///
    /// This is the non-panicking alternative to indexing the matrix. Returns [`None`] whenever
    /// equivalent indexing operation would panic.
    pub fn get<'a, I>(&'a self, index: I) -> Option<I::Output>
    where
        I: MatrixIndex<'a, T>,
    {
        index.get(self)
    }

    /// Returns a mutable reference to the value at given index.
    ///
    /// This is the non-panicking alternative to indexing the matrix. Returns [`None`] whenever
    /// equivalent indexing operation would panic.
    pub fn get_mut<'a, I>(&'a mut self, index: I) -> Option<I::OutputMut>
    where
        I: MatrixIndexMut<'a, T>,
    {
        index.get_mut(self)
    }

    /// Returns a reference to the value at given index, or panics if the index is out of bounds.
    pub fn index<'a, I>(&'a self, index: I) -> I::Output
    where
        I: MatrixIndex<'a, T>,
    {
        index.index(self)
    }

    /// Returns a mutable reference to the value at given index, or panics if the index is out of
    /// bounds.
    pub fn index_mut<'a, I>(&'a mut self, index: I) -> I::OutputMut
    where
        I: MatrixIndexMut<'a, T>,
    {
        index.index_mut(self)
    }
}

// Extract a single element by a pair of indices (row, col)

impl<'a, T> MatrixIndex<'a, T> for (usize, usize)
where
    T: 'a,
{
    type Output = &'a T;

    fn contained_by(&self, matrix: &Matrix<T>) -> bool {
        let &(row, col) = self;
        let (nrows, ncols) = matrix.shape();
        row < nrows && col < ncols
    }

    unsafe fn get_unchecked(self, matrix: &'a Matrix<T>) -> &'a T {
        let (row, col) = self;
        let (_, ncols) = matrix.shape();
        &matrix.data[col + row * ncols]
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for (usize, usize)
where
    T: 'a,
{
    type OutputMut = &'a mut T;

    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T>) -> Self::OutputMut {
        let (row, col) = self;
        let (_, ncols) = matrix.shape();
        &mut matrix.data[col + row * ncols]
    }
}

// Extract a single element by a position

impl<'a, T> MatrixIndex<'a, T> for Position
where
    T: 'a,
{
    type Output = &'a T;

    fn contained_by(&self, matrix: &Matrix<T>) -> bool {
        self.as_tuple().contained_by(matrix)
    }

    unsafe fn get_unchecked(self, matrix: &'a Matrix<T>) -> &'a T {
        self.as_tuple().get_unchecked(matrix)
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for Position
where
    T: 'a,
{
    type OutputMut = &'a mut T;

    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T>) -> Self::OutputMut {
        self.as_tuple().get_unchecked_mut(matrix)
    }
}

// Extract a specific row

impl<'a, T> MatrixIndex<'a, T> for (usize, RangeFull)
where
    T: 'a,
{
    type Output = RowVector<'a, T>;

    fn contained_by(&self, matrix: &Matrix<T>) -> bool {
        self.0 < matrix.shape().0
    }

    unsafe fn get_unchecked(self, matrix: &'a Matrix<T>) -> RowVector<'a, T> {
        Vector::new_unchecked_row(matrix, self.0)
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for (usize, RangeFull)
where
    T: 'a,
{
    type OutputMut = RowVectorMut<'a, T>;

    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T>) -> RowVectorMut<'a, T> {
        VectorMut::new_unchecked_row(matrix, self.0)
    }
}

// Extract a specific column

impl<'a, T> MatrixIndex<'a, T> for (RangeFull, usize)
where
    T: 'a,
{
    type Output = ColumnVector<'a, T>;

    fn contained_by(&self, matrix: &Matrix<T>) -> bool {
        self.1 < matrix.shape().1
    }

    unsafe fn get_unchecked(self, matrix: &'a Matrix<T>) -> ColumnVector<'a, T> {
        Vector::new_unchecked_column(matrix, self.1)
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for (RangeFull, usize)
where
    T: 'a,
{
    type OutputMut = ColumnVectorMut<'a, T>;

    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<T>) -> ColumnVectorMut<'a, T> {
        VectorMut::new_unchecked_column(matrix, self.1)
    }
}

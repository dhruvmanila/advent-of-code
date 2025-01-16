use std::ops::{Deref, DerefMut};

use super::{Matrix, MatrixError};

/// A square matrix, a special case of [`Matrix`] where the number of rows and columns are equal.
///
/// This type de-references to [`Matrix`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SquareMatrix<T> {
    inner: Matrix<T>,
}

impl<T> Deref for SquareMatrix<T> {
    type Target = Matrix<T>;

    fn deref(&self) -> &Matrix<T> {
        &self.inner
    }
}

impl<T> DerefMut for SquareMatrix<T> {
    fn deref_mut(&mut self) -> &mut Matrix<T> {
        &mut self.inner
    }
}

// Constructors which delegates to the inner matrix.
impl<T> SquareMatrix<T> {
    /// Constructs a new square matrix from a given matrix. Returns an error if it is not a square
    /// matrix.
    #[inline]
    fn new(matrix: Matrix<T>) -> Result<SquareMatrix<T>, MatrixError> {
        if matrix.is_square() {
            Ok(SquareMatrix { inner: matrix })
        } else {
            Err(MatrixError::NotSquare {
                nrows: matrix.nrows,
                ncols: matrix.ncols,
            })
        }
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::new_with`].
    #[inline]
    pub fn new_with(size: usize, value: T) -> SquareMatrix<T>
    where
        T: Clone,
    {
        SquareMatrix {
            inner: Matrix::new_with(size, size, value),
        }
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::from_vec`].
    #[inline]
    pub fn from_vec(size: usize, data: Vec<T>) -> Result<SquareMatrix<T>, MatrixError> {
        Ok(SquareMatrix {
            inner: Matrix::from_vec(size, size, data)?,
        })
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::from_rows`].
    #[inline]
    pub fn from_rows<R, C>(rows: R) -> Result<SquareMatrix<T>, MatrixError>
    where
        R: IntoIterator<Item = C>,
        C: IntoIterator<Item = T>,
    {
        Matrix::from_rows(rows)
            .map(SquareMatrix::new)
            .and_then(|m| m)
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::try_from_rows`].
    #[inline]
    pub fn try_from_rows<R, C, E>(rows: R) -> Result<SquareMatrix<T>, MatrixError>
    where
        R: IntoIterator<Item = C>,
        C: IntoIterator<Item = Result<T, E>>,
        E: Into<MatrixError>,
    {
        Matrix::try_from_rows(rows)
            .map(SquareMatrix::new)
            .and_then(|m| m)
    }
}

impl<T> SquareMatrix<T> {
    /// Transposes the matrix in-place.
    pub fn transpose_mut(&mut self) {
        for i in 1..self.inner.nrows {
            for j in 0..i {
                self.inner.swap((i, j), (j, i));
            }
        }
    }

    /// Rotates the matrix 90 degrees clockwise in-place.
    pub fn rotate_mut(&mut self) {
        self.transpose_mut();
        for mut row in self.inner.row_iter_mut() {
            row.reverse();
        }
    }
}

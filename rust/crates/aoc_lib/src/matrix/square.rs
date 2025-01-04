use std::ops::{Deref, DerefMut};

use super::Matrix;

/// A square matrix, a special case of [`Matrix`] where the number of rows and columns are equal.
///
/// This type de-references to [`Matrix`].
#[derive(Debug, PartialEq, Eq)]
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
    pub fn from_vec(size: usize, data: Vec<T>) -> SquareMatrix<T> {
        SquareMatrix {
            inner: Matrix::from_vec(size, size, data),
        }
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::from_iter`].
    #[inline]
    pub fn from_iter(size: usize, data: impl IntoIterator<Item = T>) -> SquareMatrix<T> {
        SquareMatrix {
            inner: Matrix::from_iter(size, size, data),
        }
    }

    /// Constructs a new square matrix of given `size` using [`Matrix::try_from_iter`].
    #[inline]
    pub fn try_from_iter<E>(
        size: usize,
        data: impl IntoIterator<Item = Result<T, E>>,
    ) -> Result<SquareMatrix<T>, E> {
        Ok(SquareMatrix {
            inner: Matrix::try_from_iter(size, size, data)?,
        })
    }
}

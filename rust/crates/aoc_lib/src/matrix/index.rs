use super::{Matrix, Position};

/// A helper trait for indexing into a [`Matrix`].
pub trait MatrixIndex<'a, T>: Sized {
    /// Returns a reference to the element in the `matrix` at this index, or [`None`] if the index
    /// is out of bounds.
    fn get(self, matrix: &'a Matrix<T>) -> Option<&'a T>;
}

/// A helper trait for indexing into a [`Matrix`] with mutable references.
pub trait MatrixIndexMut<'a, T>: MatrixIndex<'a, T> {
    /// Returns a mutable reference to the element at the given index, or [`None`] if the index is
    /// out of bounds.
    fn get_mut(self, matrix: &'a mut Matrix<T>) -> Option<&'a mut T>;
}

impl<T> Matrix<T> {
    /// Returns a reference to the value at given index.
    ///
    /// This is the non-panicking alternative to indexing the matrix. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.get((0, 0)), Some(&0));
    /// assert_eq!(matrix.get((2, 5)), Some(&17));
    ///
    /// // Out of bounds
    /// assert!(matrix.get((10, 2)).is_none());
    /// ```
    pub fn get<'a, I>(&'a self, index: I) -> Option<&'a T>
    where
        I: MatrixIndex<'a, T>,
    {
        index.get(self)
    }

    /// Returns a mutable reference to the value at given index.
    ///
    /// This is the non-panicking alternative to indexing the matrix. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get((0, 0)), Some(&0));
    ///
    /// let cell = matrix.get_mut((0, 0)).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(matrix.get((0, 0)), Some(&5));
    /// ```
    pub fn get_mut<'a, I>(&'a mut self, index: I) -> Option<&'a mut T>
    where
        I: MatrixIndexMut<'a, T>,
    {
        index.get_mut(self)
    }
}

impl<'a, T> MatrixIndex<'a, T> for Position {
    fn get(self, matrix: &'a Matrix<T>) -> Option<&'a T> {
        let (row, col) = self.as_tuple();
        let (nrows, ncols) = matrix.shape();

        if row < nrows && col < ncols {
            Some(&matrix.data[col + row * ncols])
        } else {
            None
        }
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for Position {
    fn get_mut(self, matrix: &'a mut Matrix<T>) -> Option<&'a mut T> {
        let (row, col) = self.as_tuple();
        let (nrows, ncols) = matrix.shape();

        if row < nrows && col < ncols {
            Some(&mut matrix.data[col + row * ncols])
        } else {
            None
        }
    }
}

impl<'a, T> MatrixIndex<'a, T> for (usize, usize) {
    fn get(self, matrix: &'a Matrix<T>) -> Option<&'a T> {
        let (row, col) = self;
        let (nrows, ncols) = matrix.shape();

        if row < nrows && col < ncols {
            Some(&matrix.data[col + row * ncols])
        } else {
            None
        }
    }
}

impl<'a, T> MatrixIndexMut<'a, T> for (usize, usize) {
    fn get_mut(self, matrix: &'a mut Matrix<T>) -> Option<&'a mut T> {
        let (row, col) = self;
        let (nrows, ncols) = matrix.shape();

        if row < nrows && col < ncols {
            Some(&mut matrix.data[col + row * ncols])
        } else {
            None
        }
    }
}

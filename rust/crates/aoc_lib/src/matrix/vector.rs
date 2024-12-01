use std::ops::{Index, IndexMut};

/// A view into a column or row of a matrix.
///
/// This is a read-only view. If you need to mutate the matrix, use [`VectorMut`].
///
/// This struct is created by the [`column`] and [`row`] methods on
/// [`Matrix`](crate::Matrix).
///
/// [`column`]: crate::Matrix#method.column
/// [`row`]: crate::Matrix#method.row
#[derive(Debug)]
pub struct Vector<'a, T> {
    pub(super) len: usize,
    pub(super) inc: usize,
    pub(super) data: &'a [T],
}

impl<T> Vector<'_, T> {
    /// Returns the number of elements in the vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 4, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// assert_eq!(row.len(), 4);
    ///
    /// let column = matrix.column(1).unwrap();
    /// assert_eq!(column.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// assert_eq!(row.get(0), Some(&2));
    /// assert_eq!(row.get(1), Some(&3));
    /// assert_eq!(row.get(2), None);
    ///
    /// let column = matrix.column(0).unwrap();
    /// assert_eq!(column.get(0), Some(&0));
    /// assert_eq!(column.get(1), Some(&2));
    /// assert_eq!(column.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self[index])
        } else {
            None
        }
    }

    /// Returns an iterator over the elements of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// let mut row_iter = row.iter();
    /// assert_eq!(row_iter.next(), Some(&2));
    /// assert_eq!(row_iter.next(), Some(&3));
    /// assert_eq!(row_iter.next(), None);
    ///
    /// let column = matrix.column(0).unwrap();
    /// let mut column_iter = column.iter();
    /// assert_eq!(column_iter.next(), Some(&0));
    /// assert_eq!(column_iter.next(), Some(&2));
    /// assert_eq!(column_iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().step_by(self.inc)
    }
}

impl<T> Index<usize> for Vector<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.inc]
    }
}

/// A mutable view into a column or row of a matrix.
///
/// The backing data storage is shared with the original matrix. This means that
/// changes to the vector will be reflected in the matrix and vice versa.
///
/// This struct is created by the [`get_column_mut`] and [`get_row_mut`] methods
/// on [`Matrix`](crate::Matrix).
///
/// [`get_column_mut`]: crate::Matrix#method.get_column_mut
/// [`get_row_mut`]: crate::Matrix#method.get_row_mut
pub struct VectorMut<'a, T> {
    pub(super) len: usize,
    pub(super) inc: usize,
    pub(super) data: &'a mut [T],
}

impl<T> VectorMut<'_, T> {
    /// Returns the number of elements in the vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 4, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// assert_eq!(row.len(), 4);
    ///
    /// let column = matrix.column(1).unwrap();
    /// assert_eq!(column.len(), 2);
    /// ```
    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// assert_eq!(row.get(0), Some(&2));
    /// assert_eq!(row.get(1), Some(&3));
    /// assert_eq!(row.get(2), None);
    ///
    /// let column = matrix.column(0).unwrap();
    /// assert_eq!(column.get(0), Some(&0));
    /// assert_eq!(column.get(1), Some(&2));
    /// assert_eq!(column.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self[index])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let mut row = matrix.row_mut(1).unwrap();
    /// assert_eq!(row.get(0), Some(&2));
    ///
    /// let cell = row.get_mut(0).unwrap();
    /// // Update the cell
    /// *cell = 7;
    ///
    /// assert_eq!(row.get(0), Some(&7));
    /// // The matrix is also updated
    /// assert_eq!(matrix.get(1, 0), Some(&7));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self[index])
        } else {
            None
        }
    }

    /// Returns an iterator over the elements of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let row = matrix.row(1).unwrap();
    /// let mut row_iter = row.iter();
    /// assert_eq!(row_iter.next(), Some(&2));
    /// assert_eq!(row_iter.next(), Some(&3));
    /// assert_eq!(row_iter.next(), None);
    ///
    /// let column = matrix.column(0).unwrap();
    /// let mut column_iter = column.iter();
    /// assert_eq!(column_iter.next(), Some(&0));
    /// assert_eq!(column_iter.next(), Some(&2));
    /// assert_eq!(column_iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().step_by(self.inc)
    }

    /// Returns a mutable iterator over the elements of the vector.
    ///
    /// # Example
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    ///
    /// let mut row = matrix.row_mut(1).unwrap();
    /// assert_eq!(row.get(0), Some(&2));
    /// assert_eq!(row.get(1), Some(&3));
    ///
    /// // Update the row cells
    /// for cell in row.iter_mut() {
    ///    *cell += 2;
    /// }
    ///
    /// assert_eq!(row.get(0), Some(&4));
    /// assert_eq!(row.get(1), Some(&5));
    ///
    /// // The matrix is also updated
    /// assert_eq!(matrix.get(1, 0), Some(&4));
    /// assert_eq!(matrix.get(1, 1), Some(&5));
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().step_by(self.inc)
    }
}

impl<T> Index<usize> for VectorMut<'_, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.inc]
    }
}

impl<T> IndexMut<usize> for VectorMut<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.inc]
    }
}

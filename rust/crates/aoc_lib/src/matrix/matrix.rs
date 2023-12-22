use std::ops::{Index, IndexMut};

use super::{ColumnIter, MatrixEnumerate, Position, RowIter, Vector, VectorMut};

/// A generic implementation of a dynamically sized matrix.
///
/// The matrix is stored in row-major order, meaning that the first row is
/// stored first, then the second row, etc. The data is backed by a [`Vec`].
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    /// Number of rows in the matrix
    nrows: usize,
    /// Number of columns in the matrix
    ncols: usize,
    /// The matrix data, stored in row-major order
    data: Vec<T>,
}

/// Constructors
impl<T> Matrix<T> {
    /// Constructs a new, empty matrix.
    ///
    /// Use [`Matrix::from_iter`] if you want to set the matrix from an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::new();
    ///
    /// assert_eq!(matrix.nrows(), 0);
    /// assert_eq!(matrix.ncols(), 0);
    /// ```
    #[inline]
    pub fn new() -> Matrix<T>
    where
        T: Default,
    {
        Matrix::default()
    }

    /// Constructs a new, non-empty matrix of given size filled with the values
    /// returned by the given iterator.
    ///
    /// The matrix cells are set in a row-major order. The iterator can be infinite
    /// as this method only consumes `rows * cols` values from the iterator.
    ///
    /// # Panics
    ///
    /// Panics if either `rows` or `cols` are equal to `0` or if the iterator
    /// does not have `rows * cols` values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.get(0, 0), Some(&0));
    /// assert_eq!(matrix.get(0, 1), Some(&1));
    /// assert_eq!(matrix.get(1, 0), Some(&6));
    /// ```
    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            nrows: rows,
            ncols: cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                // This is required to ensure that the iterator had enough values
                // to fill the matrix as `take` will stop as soon as it reaches
                // the end of the iterator.
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }
}

/// Methods
impl<T> Matrix<T> {
    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.nrows(), 3);
    /// ```
    #[inline]
    pub const fn nrows(&self) -> usize {
        self.nrows
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(matrix.ncols(), 6);
    /// ```
    #[inline]
    pub const fn ncols(&self) -> usize {
        self.ncols
    }

    /// Returns the dimensions of the matrix as a tuple of `(rows, cols)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.dim(), (3, 6));
    /// ```
    #[inline]
    pub const fn dim(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Returns a reference to the value at given row & column.
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
    /// assert_eq!(matrix.get(0, 0), Some(&0));
    /// assert_eq!(matrix.get(2, 5), Some(&17));
    ///
    /// // Out of bounds
    /// assert!(matrix.get(10, 2).is_none());
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.nrows && col < self.ncols {
            Some(&self.data[col + row * self.ncols])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value at given row & column.
    ///
    /// This is the non-panicking alternative to indexing the matrix. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(0, 0), Some(&0));
    ///
    /// let cell = matrix.get_mut(0, 0).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(matrix.get(0, 0), Some(&5));
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.nrows && col < self.ncols {
            Some(&mut self.data[col + row * self.ncols])
        } else {
            None
        }
    }

    /// Set the content of a cell at the given `row` and `col` to `value`.
    ///
    /// # Panics
    ///
    /// Panics if either `row` or `col` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(0, 0), Some(&0));
    ///
    /// matrix.set(0, 0, 5);
    /// assert_eq!(matrix.get(0, 0), Some(&5));
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
        } else {
            panic!("`row` or `col` out of bounds");
        }
    }

    /// Returns a view into a `row` of the matrix.
    ///
    /// Returns [`None`] if `row` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// let row = matrix.row(1).unwrap();
    ///
    /// assert_eq!(row.len(), 6);
    /// assert_eq!(row.get(0), Some(&6));
    /// assert_eq!(row.get(5), Some(&11));
    /// ```
    pub fn row(&self, row: usize) -> Option<Vector<T>> {
        if row < self.nrows {
            Some(Vector {
                len: self.ncols,
                inc: 1,
                data: &self.data[row * self.ncols..(row + 1) * self.ncols],
            })
        } else {
            None
        }
    }

    /// Returns a mutable view into a `row` of the matrix.
    ///
    /// Returns [`None`] if `row` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(1, 1), Some(&7));
    ///
    /// let mut row = matrix.row_mut(1).unwrap();
    /// assert_eq!(row.get(1), Some(&7));
    ///
    /// // Change the value
    /// let cell = row.get_mut(1).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(row.get(1), Some(&5));
    ///
    /// // The change is reflected in the matrix
    /// assert_eq!(matrix.get(1, 1), Some(&5));
    /// ```
    pub fn row_mut(&mut self, row: usize) -> Option<VectorMut<T>> {
        if row < self.nrows {
            Some(VectorMut {
                len: self.ncols,
                inc: 1,
                data: &mut self.data[row * self.ncols..(row + 1) * self.ncols],
            })
        } else {
            None
        }
    }

    /// Returns a view into a `column` of the matrix.
    ///
    /// Returns [`None`] if `column` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// let column = matrix.column(1).unwrap();
    ///
    /// assert_eq!(column.len(), 3);
    /// assert_eq!(column.get(0), Some(&1));
    /// assert_eq!(column.get(2), Some(&13));
    /// ```
    pub fn column(&self, column: usize) -> Option<Vector<T>> {
        if column < self.ncols {
            Some(Vector {
                len: self.nrows,
                inc: self.ncols,
                data: &self.data[column..=(column + (self.nrows - 1) * self.ncols)],
            })
        } else {
            None
        }
    }

    /// Returns a mutable view into a `column` of the matrix.
    ///
    /// Returns [`None`] if `column` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let mut matrix: Matrix<u32> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(matrix.get(1, 2), Some(&8));
    ///
    /// let mut column = matrix.column_mut(2).unwrap();
    /// assert_eq!(column.get(1), Some(&8));
    ///
    /// // Change the value
    /// let cell = column.get_mut(1).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(column.get(1), Some(&5));
    ///
    /// // The change is reflected in the matrix
    /// assert_eq!(matrix.get(1, 2), Some(&5));
    /// ```
    pub fn column_mut(&mut self, column: usize) -> Option<VectorMut<T>> {
        if column < self.ncols {
            Some(VectorMut {
                len: self.nrows,
                inc: self.ncols,
                data: &mut self.data[column..=(column + (self.nrows - 1) * self.ncols)],
            })
        } else {
            None
        }
    }

    /// Returns an iterator over the rows of the matrix.
    ///
    /// Each row is represented as a [`Vector`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// // Create a 2x2 matrix with values 0..4
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// let mut row_iter = matrix.rows();
    ///
    /// let first_row = row_iter.next().unwrap();
    /// assert_eq!(first_row.len(), 2);
    /// assert_eq!(first_row.get(0), Some(&0));
    /// assert_eq!(first_row.get(1), Some(&1));
    /// assert!(first_row.get(2).is_none());
    ///
    /// let second_row = row_iter.next().unwrap();
    /// assert_eq!(second_row.len(), 2);
    /// assert_eq!(second_row.get(0), Some(&2));
    /// assert_eq!(second_row.get(1), Some(&3));
    /// assert!(second_row.get(2).is_none());
    ///
    /// // There are no more rows
    /// assert!(row_iter.next().is_none());
    /// ```
    pub fn rows(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }

    /// Returns an iterator over the columns of the matrix.
    ///
    /// Each column is represented as a [`Vector`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// // Create a 2x2 matrix with values 0..4
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// let mut column_iter = matrix.columns();
    ///
    /// let first_column = column_iter.next().unwrap();
    /// assert_eq!(first_column.len(), 2);
    /// assert_eq!(first_column.get(0), Some(&0));
    /// assert_eq!(first_column.get(1), Some(&2));
    /// assert!(first_column.get(2).is_none());
    ///
    /// let second_column = column_iter.next().unwrap();
    /// assert_eq!(second_column.len(), 2);
    /// assert_eq!(second_column.get(0), Some(&1));
    /// assert_eq!(second_column.get(1), Some(&3));
    /// assert!(second_column.get(2).is_none());
    ///
    /// // There are no more columns
    /// assert!(column_iter.next().is_none());
    /// ```
    pub fn columns(&self) -> ColumnIter<'_, T> {
        ColumnIter::new(self)
    }

    /// Returns an iterator over the position and value of each cell in the
    /// matrix in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::{Matrix, Position};
    /// // Create a 2x2 matrix with values 0..4
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// let mut iter = matrix.enumerate();
    ///
    /// assert_eq!(iter.next(), Some((Position::new(0, 0), &0)));
    /// assert_eq!(iter.next(), Some((Position::new(0, 1), &1)));
    /// assert_eq!(iter.next(), Some((Position::new(1, 0), &2)));
    /// assert_eq!(iter.next(), Some((Position::new(1, 1), &3)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn enumerate(&self) -> MatrixEnumerate<'_, T> {
        MatrixEnumerate::new(self)
    }

    /// Returns a slice containing all the elements in the matrix in row-major order.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// assert_eq!(matrix.as_slice(), &[0, 1, 2, 3]);
    /// ```
    ///
    /// The slice can be used to iterate over the elements of the matrix:
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// let mut iter = matrix.as_slice().iter();
    ///
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }
}

impl<T> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    /// Returns a reference to the value at the index, given as an array of
    /// `[row, col]`.
    ///
    /// # Panics
    ///
    /// If either `row` or `col` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// assert_eq!(matrix[[0, 0]], 0);
    /// assert_eq!(matrix[[0, 1]], 1);
    /// ```
    ///
    /// The following panics because `row` is out of bounds:
    ///
    /// ```should_panic
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// matrix[[2, 0]];
    /// ```
    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        &self.data[col + row * self.ncols]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[col + row * self.ncols]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    /// Returns a reference to the value at the index, given as a tuple of
    /// `(row, col)`.
    ///
    /// # Panics
    ///
    /// If either `row` or `col` are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// assert_eq!(matrix[(0, 0)], 0);
    /// assert_eq!(matrix[(0, 1)], 1);
    /// ```
    ///
    /// The following panics because `row` is out of bounds:
    ///
    /// ```should_panic
    /// # use aoc_lib::matrix::Matrix;
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// matrix[(2, 0)];
    /// ```
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[col + row * self.ncols]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[col + row * self.ncols]
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    /// Returns a reference to the value at the given [`Position`].
    ///
    /// # Panics
    ///
    /// If either `row` or `col` of the position are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::matrix::{Matrix, Position};
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// assert_eq!(matrix[Position::new(0, 0)], 0);
    /// assert_eq!(matrix[Position::new(0, 1)], 1);
    /// ```
    ///
    /// The following panics because `row` is out of bounds:
    ///
    /// ```should_panic
    /// # use aoc_lib::matrix::{Matrix, Position};
    /// let matrix: Matrix<u32> = Matrix::from_iter(2, 2, 0..);
    /// matrix[Position::new(2, 0)];
    /// ```
    fn index(&self, pos: Position) -> &Self::Output {
        &self[pos.as_tuple()]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self[pos.as_tuple()]
    }
}

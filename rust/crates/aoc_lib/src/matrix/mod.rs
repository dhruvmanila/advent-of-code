mod direction;
mod index;
mod iter;
mod position;
mod square;
mod vector;

use std::fmt;
use std::ops::{Index, IndexMut};

pub use direction::{CardinalDirection, Direction};
pub use iter::{
    ColumnIter, ColumnIterMut, MatrixEnumerate, PositionsInDirectionIter, RowIter, RowIterMut,
};
pub use position::Position;
pub use square::SquareMatrix;
pub use vector::{ColumnVector, ColumnVectorMut, RowVector, RowVectorMut, Vector, VectorMut};

/// A type-level marker for the row dimension.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RowDim;

/// A type-level marker for the column dimension.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColumnDim;

/// A generic implementation of a dynamically sized matrix.
///
/// The matrix is stored in row-major order, meaning that the first row is stored first, then the
/// second row, etc. The data is backed by a [`Vec`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Constructs a new matrix of given size (`rows * cols`) filled with the given `value`.
    #[inline]
    pub fn new_with(rows: usize, cols: usize, value: T) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            nrows: rows,
            ncols: cols,
            data: vec![value; rows * cols],
        }
    }

    /// Constructs a new matrix of given size (`rows * cols`) filled with the `data`.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in `data` is not equal to `rows * cols`.
    #[inline]
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(rows * cols, data.len());

        Matrix {
            nrows: rows,
            ncols: cols,
            data,
        }
    }

    /// Constructs a new, non-empty matrix of given size filled with the values returned by the
    /// given iterator.
    ///
    /// The matrix cells are set in a row-major order. The iterator can be infinite as this method
    /// only consumes `rows * cols` values from the iterator.
    ///
    /// # Panics
    ///
    /// Panics if either `rows` or `cols` are equal to `0` or if the iterator does not have `rows *
    /// cols` values.
    #[inline]
    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            nrows: rows,
            ncols: cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                // This is required to ensure that the iterator had enough values to fill the
                // matrix as `take` will stop as soon as it reaches the end of the iterator.
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    /// Constructs a matrix in a similar way to [`Matrix::from_iter`] but expects the iterator to
    /// return [`Result`]s instead of values.
    ///
    /// If the iterator returns an [`Err`] value, the matrix construction will stop and the error
    /// will be returned, otherwise the matrix will be created using all the [`Ok`] values returned
    /// by the iterator.
    #[inline]
    pub fn try_from_iter<E>(
        rows: usize,
        cols: usize,
        data: impl IntoIterator<Item = Result<T, E>>,
    ) -> Result<Matrix<T>, E> {
        assert!(rows > 0 && cols > 0);

        Ok(Matrix {
            nrows: rows,
            ncols: cols,
            data: {
                let data: Vec<_> = data
                    .into_iter()
                    .take(rows * cols)
                    .collect::<Result<_, _>>()?;
                // This is required to ensure that the iterator had enough values to fill the
                // matrix as `take` will stop as soon as it reaches the end of the iterator.
                assert_eq!(data.len(), rows * cols);
                data
            },
        })
    }
}

/// Methods
impl<T> Matrix<T> {
    /// Returns the number of rows in the matrix.
    #[inline]
    pub const fn nrows(&self) -> usize {
        self.nrows
    }

    /// Returns the number of columns in the matrix.
    #[inline]
    pub const fn ncols(&self) -> usize {
        self.ncols
    }

    /// Returns the shape of the matrix as a tuple of `(rows, cols)`.
    #[inline]
    pub const fn shape(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Returns a view into the i-th row of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the row index is out of bounds.
    #[inline]
    pub fn row(&self, i: usize) -> RowVector<'_, T> {
        self.index((i, ..))
    }

    /// Returns a mutable view into the i-th row of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the row index is out of bounds.
    #[inline]
    pub fn row_mut(&mut self, i: usize) -> RowVectorMut<'_, T> {
        self.index_mut((i, ..))
    }

    /// Returns a view into the i-th column of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the column index is out of bounds.
    #[inline]
    pub fn column(&self, i: usize) -> ColumnVector<'_, T> {
        self.index((.., i))
    }

    /// Returns a mutable view into the i-th column of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the column index is out of bounds.
    #[inline]
    pub fn column_mut(&mut self, i: usize) -> ColumnVectorMut<'_, T> {
        self.index_mut((.., i))
    }

    /// Returns an iterator over all the rows of the matrix.
    #[inline]
    pub fn row_iter(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }

    /// Returns an iterator over all the rows of the matrix, allowing mutable access to each row.
    #[inline]
    pub fn row_iter_mut(&mut self) -> RowIterMut<'_, T> {
        RowIterMut::new(self)
    }

    /// Returns an iterator over all the columns of the matrix.
    #[inline]
    pub fn column_iter(&self) -> ColumnIter<'_, T> {
        ColumnIter::new(self)
    }

    /// Returns an iterator over all the columns of the matrix, allowing mutable access to each
    /// column.
    #[inline]
    pub fn column_iter_mut(&mut self) -> ColumnIterMut<'_, T> {
        ColumnIterMut::new(self)
    }

    /// Returns an iterator over the [`Position`] and value of each cell in the matrix in row-major
    /// order.
    #[inline]
    pub fn enumerate(&self) -> MatrixEnumerate<'_, T> {
        MatrixEnumerate::new(self)
    }

    /// Returns an iterator over the [`Position`]s in the given [`Direction`].
    ///
    /// The iterator starts from the next position after the given `start` position along the given
    /// `direction` and continues until the end of the matrix.
    #[inline]
    pub fn positions_in_direction(
        &self,
        start: Position,
        direction: Direction,
    ) -> PositionsInDirectionIter {
        PositionsInDirectionIter::new(self.shape(), start, direction)
    }

    /// Finds the [`Position`] of the first occurrence of the given `expected` value in the matrix,
    /// [`None`] if not found.
    #[inline]
    pub fn find_position(&self, expected: &T) -> Option<Position>
    where
        T: PartialEq,
    {
        let index = self.data.iter().position(|item| item == expected)?;
        Some(Position::new(index / self.ncols, index % self.ncols))
    }

    /// Returns a slice containing all the elements in the matrix in row-major order.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Swaps two elements in the matrix.
    ///
    /// # Panics
    ///
    /// Panics if either `pos1` or `pos2` are out of bounds.
    #[inline]
    pub fn swap(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        self.data
            .swap(pos1.0 * self.ncols + pos1.1, pos2.0 * self.ncols + pos2.1);
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &T {
        self.get((row, col)).expect("row or column out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.get_mut((row, col))
            .expect("row or column out of bounds")
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, pos: Position) -> &T {
        &self[pos.as_tuple()]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, pos: Position) -> &mut T {
        &mut self[pos.as_tuple()]
    }
}

impl<T> Index<&Position> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Position) -> &T {
        &self[index.as_tuple()]
    }
}

impl<T> IndexMut<&Position> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, index: &Position) -> &mut T {
        &mut self[index.as_tuple()]
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.row_iter() {
            for cell in &row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

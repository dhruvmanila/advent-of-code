mod direction;
mod index;
mod iter;
mod position;
mod square;
mod vector;

use std::fmt;
use std::ops::{Index, IndexMut};

pub use direction::{CardinalDirection, Direction};
pub use iter::{ColumnIter, ColumnIterMut, MatrixEnumerate, RowIter, RowIterMut};
pub use position::Position;
pub use square::SquareMatrix;
pub use vector::{ColumnVector, ColumnVectorMut, RowVector, RowVectorMut, Vector, VectorMut};

/// A type-level marker for the row dimension.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RowDim;

/// A type-level marker for the column dimension.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColumnDim;

#[derive(Debug, thiserror::Error)]
pub enum MatrixError {
    #[error("Empty matrix cannot be created")]
    Empty,

    #[error("Expected {} elements for a {nrows} x {nrows} matrix, got {received}", nrows * ncols)]
    LengthMismatch {
        nrows: usize,
        ncols: usize,
        received: usize,
    },

    #[error("{nrows} x {ncols} is not a square matrix")]
    NotSquare { nrows: usize, ncols: usize },

    #[error("Invalid character {0:?} in the matrix input")]
    InvalidCharacter(char),
}

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
    /// Constructs a new matrix of given size (`nrows * ncols`) filled with the given `value`.
    #[inline]
    pub fn new_with(nrows: usize, ncols: usize, value: T) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            nrows,
            ncols,
            data: vec![value; nrows * ncols],
        }
    }

    /// Constructs a new matrix of given size (`nrows * ncols`) filled with the `data`.
    ///
    /// Returns [`MatrixError::LengthMismatch`] if the length of `data` is not equal to `nrows *
    /// ncols`.
    #[inline]
    pub fn from_vec(nrows: usize, ncols: usize, data: Vec<T>) -> Result<Matrix<T>, MatrixError> {
        if nrows * ncols == data.len() {
            Ok(Matrix { nrows, ncols, data })
        } else {
            Err(MatrixError::LengthMismatch {
                nrows,
                ncols,
                received: data.len(),
            })
        }
    }

    /// Constructs a new, non-empty matrix from the given `rows`.
    ///
    /// The `rows` should be a type that can be converted into an iterator on each row which itself
    /// should be a type that can be converted into an iterator on each column of that row.
    pub fn from_rows<R, C>(rows: R) -> Result<Matrix<T>, MatrixError>
    where
        R: IntoIterator<Item = C>,
        C: IntoIterator<Item = T>,
    {
        // Delegate the construction by converting each item into an `Ok` value.
        Matrix::try_from_rows(
            rows.into_iter()
                .map(|row| row.into_iter().map(Ok::<_, MatrixError>)),
        )
    }

    /// Constructs a new, non-empty matrix from the given `rows`.
    ///
    /// The `rows` should be a type that can be converted into an iterator on each row which itself
    /// should be a type that can be converted into an iterator on each column of that row.
    ///
    /// The difference between this method and [`Matrix::from_rows`] is that this method expects
    /// the inner iterators to return [`Result`]s instead of values. The error type of the inner
    /// iterator should be convertible into a [`MatrixError`].
    #[inline]
    pub fn try_from_rows<R, C, E>(rows: R) -> Result<Matrix<T>, MatrixError>
    where
        R: IntoIterator<Item = C>,
        C: IntoIterator<Item = Result<T, E>>,
        E: Into<MatrixError>,
    {
        let mut row_iter = rows.into_iter();
        let Some(first_row) = row_iter.next() else {
            return Err(MatrixError::Empty);
        };
        let mut data = first_row
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)?;
        let ncols = data.len();
        let mut nrows = 1;
        for row in row_iter {
            nrows += 1;
            for item in row {
                data.push(item.map_err(Into::into)?);
            }
        }
        // Delegate length checking to `from_vec` to avoid code duplication. This avoids checking
        // the length of each row individually and ensures that all rows have the same length.
        Matrix::from_vec(nrows, ncols, data)
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

    /// Returns the number of elements in the matrix.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the matrix contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns `true` if this is a square matrix.
    #[inline]
    pub const fn is_square(&self) -> bool {
        self.nrows == self.ncols
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
    ) -> impl Iterator<Item = Position> + '_ {
        std::iter::successors(start.checked_neighbor(direction), move |current| {
            if current.row < self.nrows && current.col < self.ncols {
                current.checked_neighbor(direction)
            } else {
                None
            }
        })
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
        let a = self.linear_index(pos1);
        let b = self.linear_index(pos2);
        self.data.swap(a, b);
    }

    /// Compute the index corresponding to the given `(row, col)` pair of this matrix.
    const fn linear_index(&self, (row, col): (usize, usize)) -> usize {
        row * self.ncols + col
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

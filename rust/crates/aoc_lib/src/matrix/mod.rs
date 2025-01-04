mod direction;
mod index;
mod iter;
mod position;
mod vector;

use std::fmt;
use std::ops::{Index, IndexMut};

pub use direction::{CardinalDirection, Direction};
pub use iter::{ColumnIter, MatrixEnumerate, PositionsInDirectionIter, RowIter};
pub use position::Position;
pub use vector::{Vector, VectorMut};

/// A generic implementation of a dynamically sized matrix.
///
/// The matrix is stored in row-major order, meaning that the first row is stored first, then the
/// second row, etc. The data is backed by a [`Vec`].
#[derive(Debug, PartialEq, Eq)]
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

    /// Returns a view into a `row` of the matrix, [`None`] if out of bounds.
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

    /// Returns a mutable view into a `row` of the matrix, [`None`] if out of bounds.
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

    /// Returns a view into a `column` of the matrix, [`None`] if out of bounds.
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

    /// Returns a mutable view into a `column` of the matrix, [`None`] if out of bounds.
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
    pub fn rows(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }

    /// Returns an iterator over the columns of the matrix.
    ///
    /// Each column is represented as a [`Vector`].
    pub fn columns(&self) -> ColumnIter<'_, T> {
        ColumnIter::new(self)
    }

    /// Returns an iterator over the [`Position`] and value of each cell in the matrix in row-major
    /// order.
    pub fn enumerate(&self) -> MatrixEnumerate<'_, T> {
        MatrixEnumerate::new(self)
    }

    /// Finds the [`Position`] of the first occurrence of the given `expected` value in the matrix,
    /// [`None`] if not found.
    pub fn find_position(&self, expected: &T) -> Option<Position>
    where
        T: PartialEq,
    {
        let index = self.data.iter().position(|item| item == expected)?;
        Some(Position::new(index / self.ncols, index % self.ncols))
    }

    /// Returns a slice containing all the elements in the matrix in row-major order.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns an iterator over the [`Position`]s in the given [`Direction`].
    ///
    /// The iterator starts from the next position after the given `start` position along the given
    /// `direction` and continues until the end of the matrix.
    pub fn positions_in_direction(
        &self,
        start: Position,
        direction: Direction,
    ) -> PositionsInDirectionIter {
        PositionsInDirectionIter::new(self.shape(), start, direction)
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &T {
        self.get((row, col)).expect("row or column out of bounds")
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut T {
        self.get_mut((row, col))
            .expect("row or column out of bounds")
    }
}

impl<T> Index<Position> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: Position) -> &T {
        &self[pos.as_tuple()]
    }
}

impl<T> IndexMut<Position> for Matrix<T> {
    fn index_mut(&mut self, pos: Position) -> &mut T {
        &mut self[pos.as_tuple()]
    }
}

impl<T> Index<&Position> for Matrix<T> {
    type Output = T;

    fn index(&self, index: &Position) -> &T {
        &self[index.as_tuple()]
    }
}

impl<T> IndexMut<&Position> for Matrix<T> {
    fn index_mut(&mut self, index: &Position) -> &mut T {
        &mut self[index.as_tuple()]
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for cell in row.iter() {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

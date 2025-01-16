use std::iter::{FusedIterator, StepBy};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use super::{ColumnDim, RowDim};

/// A view into a row of a matrix.
pub type RowVector<'a, T> = Vector<'a, T, RowDim>;

/// A view into a column of a matrix.
pub type ColumnVector<'a, T> = Vector<'a, T, ColumnDim>;

/// A mutable view into a row of a matrix.
pub type RowVectorMut<'a, T> = VectorMut<'a, T, RowDim>;

/// A mutable view into a column of a matrix.
pub type ColumnVectorMut<'a, T> = VectorMut<'a, T, ColumnDim>;

/// A view into a column or row of a matrix.
///
/// This is a read-only view. If you need to mutate the matrix, use [`VectorMut`].
///
/// This struct is created by the indexing operations on [`Matrix`].
///
/// [`Matrix`]: crate::matrix::Matrix
#[derive(Debug)]
pub struct Vector<'a, T, D> {
    len: usize,
    inc: usize,
    data: &'a [T],
    dim: PhantomData<D>,
}

impl<'a, T, D> Vector<'a, T, D> {
    /// Returns the number of elements in the vector.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns [`None`] whenever
    /// equivalent indexing operation would panic.
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index * self.inc])
        } else {
            None
        }
    }

    /// Returns an iterator over the elements of the vector.
    #[inline]
    pub fn iter(&self) -> VectorIter<'a, T, D> {
        VectorIter {
            iter: self.data.iter().step_by(self.inc),
            dim: self.dim,
        }
    }
}

impl<'a, T, D> IntoIterator for &'a Vector<'a, T, D> {
    type Item = &'a T;
    type IntoIter = VectorIter<'a, T, D>;

    #[inline]
    fn into_iter(self) -> VectorIter<'a, T, D> {
        self.iter()
    }
}

impl<T, D> Index<usize> for Vector<'_, T, D> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<T, D> PartialEq for Vector<'_, T, D>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vector<'_, T, D>) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl<T, D> Eq for Vector<'_, T, D> where T: Eq {}

/// An iterator over the elements of a [`Vector`].
#[derive(Clone, Debug)]
pub struct VectorIter<'a, T, D> {
    iter: StepBy<Iter<'a, T>>,
    dim: PhantomData<D>,
}

impl<'a, T, D> Iterator for VectorIter<'a, T, D> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T, D> DoubleEndedIterator for VectorIter<'a, T, D> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<T, D> ExactSizeIterator for VectorIter<'_, T, D> {}
impl<T, D> FusedIterator for VectorIter<'_, T, D> {}

/// A mutable view into a column or row of a matrix.
///
/// The backing data storage is shared with the original matrix. This means that changes to the
/// vector will be reflected in the matrix and vice versa.
///
/// This struct is created by the indexing operations on [`Matrix`].
///
/// [`Matrix`]: crate::matrix::Matrix
pub struct VectorMut<'a, T, D> {
    len: usize,
    inc: usize,
    data: &'a mut [T],
    dim: PhantomData<D>,
}

impl<T, D> VectorMut<'_, T, D> {
    /// Returns the number of elements in the vector.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns [`None`] whenever
    /// equivalent indexing operation would panic.
    #[inline]
    pub const fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.data[index * self.inc])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value at given `index`.
    ///
    /// This is the non-panicking alternative to indexing the vector. Returns [`None`] whenever
    /// equivalent indexing operation would panic.
    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.data[index * self.inc])
        } else {
            None
        }
    }

    /// Reverse the order of elements in the vector in-place.
    ///
    /// This in turn reverses the order of elements in the original matrix.
    #[inline]
    pub fn reverse(&mut self) {
        for i in 0..self.len / 2 {
            self.data.swap(i * self.inc, (self.len - i - 1) * self.inc);
        }
    }

    /// Returns an iterator over the elements of the vector.
    #[inline]
    pub fn iter(&self) -> VectorIter<'_, T, D> {
        VectorIter {
            iter: self.data.iter().step_by(self.inc),
            dim: self.dim,
        }
    }

    /// Returns a mutable iterator over the elements of the vector.
    #[inline]
    pub fn iter_mut(&mut self) -> VectorMutIter<'_, T, D> {
        VectorMutIter {
            iter: self.data.iter_mut().step_by(self.inc),
            dim: self.dim,
        }
    }
}

impl<'a, T, D> IntoIterator for &'a VectorMut<'a, T, D> {
    type Item = &'a T;
    type IntoIter = VectorIter<'a, T, D>;

    #[inline]
    fn into_iter(self) -> VectorIter<'a, T, D> {
        self.iter()
    }
}

impl<'a, T, D> IntoIterator for &'a mut VectorMut<'a, T, D> {
    type Item = &'a mut T;
    type IntoIter = VectorMutIter<'a, T, D>;

    #[inline]
    fn into_iter(self) -> VectorMutIter<'a, T, D> {
        self.iter_mut()
    }
}

impl<T, D> Index<usize> for VectorMut<'_, T, D> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<T, D> IndexMut<usize> for VectorMut<'_, T, D> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<T, D> PartialEq for VectorMut<'_, T, D>
where
    T: PartialEq,
{
    fn eq(&self, other: &VectorMut<'_, T, D>) -> bool {
        self.len == other.len && self.iter().eq(other.iter())
    }
}

impl<T, D> Eq for VectorMut<'_, T, D> where T: Eq {}

/// An iterator over the elements of a [`VectorMut`].
#[derive(Debug)]
pub struct VectorMutIter<'a, T, D> {
    iter: StepBy<IterMut<'a, T>>,
    dim: PhantomData<D>,
}

impl<'a, T, D> Iterator for VectorMutIter<'a, T, D> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T, D> DoubleEndedIterator for VectorMutIter<'a, T, D> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.iter.next_back()
    }
}

impl<T, D> ExactSizeIterator for VectorMutIter<'_, T, D> {}
impl<T, D> FusedIterator for VectorMutIter<'_, T, D> {}

/// Generate internal constructor methods for individual row or column for both [`Vector`] and
/// [`VectorMut`].
macro_rules! impl_vector_constructors {
    ($vector:ident$(, $mutable:tt)?) => {
        impl<'a, T> $vector<'a, T, $crate::matrix::RowDim> {
            #[inline]
            pub(super) unsafe fn new_unchecked_row(
                matrix: &'a $($mutable)? $crate::matrix::Matrix<T>,
                row: usize,
            ) -> $vector<'a, T, $crate::matrix::RowDim> {
                let (_, ncols) = matrix.shape();
                $vector {
                    len: ncols,
                    inc: 1,
                    data: &$($mutable)? matrix.data[row * ncols..(row + 1) * ncols],
                    dim: ::std::marker::PhantomData,
                }
            }
        }

        impl<'a, T> $vector<'a, T, $crate::matrix::ColumnDim> {
            #[inline]
            pub(super) unsafe fn new_unchecked_column(
                matrix: &'a $($mutable)? $crate::matrix::Matrix<T>,
                column: usize,
            ) -> $vector<'a, T, $crate::matrix::ColumnDim> {
                let (nrows, ncols) = matrix.shape();
                $vector {
                    len: nrows,
                    inc: ncols,
                    data: &$($mutable)? matrix.data[column..=(column + (nrows - 1) * ncols)],
                    dim: ::std::marker::PhantomData,
                }
            }
        }
    };
}

impl_vector_constructors!(Vector);
impl_vector_constructors!(VectorMut, mut);

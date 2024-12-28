use std::ops::{Index, IndexMut};

/// A view into a column or row of a matrix.
///
/// This is a read-only view. If you need to mutate the matrix, use [`VectorMut`].
///
/// This struct is created by the [`column`] and [`row`] methods on [`Matrix`].
///
/// [`Matrix`]: crate::matrix::Matrix
/// [`column`]: crate::matrix::Matrix::column
/// [`row`]: crate::matrix::Matrix::row
#[derive(Debug)]
pub struct Vector<'a, T> {
    pub(super) len: usize,
    pub(super) inc: usize,
    pub(super) data: &'a [T],
}

impl<T> Vector<'_, T> {
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
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().step_by(self.inc)
    }
}

impl<T> Index<usize> for Vector<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

/// A mutable view into a column or row of a matrix.
///
/// The backing data storage is shared with the original matrix. This means that changes to the
/// vector will be reflected in the matrix and vice versa.
///
/// This struct is created by the [`get_column_mut`] and [`get_row_mut`] methods on [`Matrix`].
///
/// [`Matrix`]: crate::matrix::Matrix
/// [`get_column_mut`]: crate::matrix::Matrix::column_mut
/// [`get_row_mut`]: crate::matrix::Matrix::row_mut
pub struct VectorMut<'a, T> {
    pub(super) len: usize,
    pub(super) inc: usize,
    pub(super) data: &'a mut [T],
}

impl<T> VectorMut<'_, T> {
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
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
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
    /// This is the non-panicking alternative to indexing the vector. Returns
    /// [`None`] whenever equivalent indexing operation would panic.
    #[inline]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            Some(&mut self.data[index * self.inc])
        } else {
            None
        }
    }

    /// Returns an iterator over the elements of the vector.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().step_by(self.inc)
    }

    /// Returns a mutable iterator over the elements of the vector.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().step_by(self.inc)
    }
}

impl<T> Index<usize> for VectorMut<'_, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> IndexMut<usize> for VectorMut<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("index out of bounds")
    }
}

use std::iter::FusedIterator;

/// An iterator that skips the `n`th element of `iter`.
///
/// This `struct` is created by the [`skip_nth`] method on [`Iterator`] using the [`SkipNthExt`]
/// trait. See its documentation for more.
///
/// [`skip_nth`]: SkipNthExt::skip_nth
/// [`Iterator`]: trait.Iterator.html
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SkipNth<I> {
    iter: I,
    n: usize,
}

impl<I> SkipNth<I> {
    pub fn new(iter: I, n: usize) -> Self {
        // `n` is the 0-based index into the iterator but we use 1-based indexing internally
        // to simplify the implementation.
        Self { iter, n: n + 1 }
    }
}

impl<I> Iterator for SkipNth<I>
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            self.iter.next()
        } else {
            self.n -= 1;
            if self.n == 0 {
                self.iter.next()?;
            }
            self.iter.next()
        }
    }
}

impl<I> FusedIterator for SkipNth<I> where I: FusedIterator {}

pub trait SkipNthExt: Iterator + Sized {
    /// Creates an iterator that skips the `n`th element.
    ///
    /// `skip_nth(n)` skips the nth element while yielding all other elements where
    /// `n` is the 0-based index of the element to skip. The first element is at index 0.
    /// If the iterator is shorter than `n`, then it yields all elements until the end
    /// of the iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::SkipNthExt;
    /// let a = [1, 2, 3];
    ///
    /// let mut iter = a.iter().skip_nth(1);
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn skip_nth(self, nth: usize) -> SkipNth<Self>;
}

impl<I: Iterator> SkipNthExt for I {
    fn skip_nth(self, nth: usize) -> SkipNth<Self> {
        SkipNth::new(self, nth)
    }
}

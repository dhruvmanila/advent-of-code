use std::iter::FusedIterator;

/// An iterator that skips the `n`th element of `iter`.
///
/// This `struct` is created by the `skip_nth` method on `Iterator` using the `IteratorExt`
/// trait. See its documentation for more.
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SkipNth<I> {
    iter: I,
    n: usize,
}

impl<I> SkipNth<I> {
    pub(super) fn new(iter: I, n: usize) -> Self {
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

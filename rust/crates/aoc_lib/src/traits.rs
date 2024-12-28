use std::hint::assert_unchecked;

use crate::SkipNth;

pub trait IteratorExt: Iterator {
    /// Searches for an element in the iterator from the right, returning its position and value.
    ///
    /// `find_rposition()` takes a closure that returns `true` or `false`. It applies this closure
    /// to each element of the iterator, starting from the end, and if one of them returns `true`,
    /// then `find_rposition()` returns [`Some((index, item))`]. If all of them return `false`, it
    /// returns [`None`].
    ///
    /// `find_rposition()` is short-circuiting; in other words, it will stop processing as soon as
    /// it finds a `true`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use aoc_lib::IteratorExt;
    ///
    /// let v = vec![1, 2, 3];
    ///
    /// assert_eq!(v.iter().find_rposition(|&&x| x == 2), Some((1, &2)));
    /// assert_eq!(v.iter().find_rposition(|&&x| x == 5), None);
    /// ```
    #[inline]
    fn find_rposition<P>(&mut self, mut predicate: P) -> Option<(usize, Self::Item)>
    where
        P: FnMut(&Self::Item) -> bool,
        Self: Sized + ExactSizeIterator + DoubleEndedIterator,
    {
        // The implementation is the same as `rposition()` but the return value includes both the
        // index and the item. This is similar to `find_position()` in the `itertools` crate.
        let length = self.len();
        let mut index = length;
        while let Some(item) = self.next_back() {
            index -= 1;
            if predicate(&item) {
                // SAFETY: `index` must be lower than `length` since it starts at `length`
                // and is only decreasing.
                unsafe { assert_unchecked(index < length) };
                return Some((index, item));
            }
        }
        None
    }

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
    /// # use aoc_lib::IteratorExt;
    /// let a = [1, 2, 3];
    ///
    /// let mut iter = a.iter().skip_nth(1);
    ///
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    fn skip_nth(self, nth: usize) -> SkipNth<Self>
    where
        Self: Sized,
    {
        SkipNth::new(self, nth)
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}

pub trait Gcd {
    /// Determine the [greatest common divisor] of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc_lib::Gcd;
    ///
    /// assert_eq!(0, 0u8.gcd(0));
    /// assert_eq!(10, 10u8.gcd(0));
    /// assert_eq!(10, 0u8.gcd(10));
    /// assert_eq!(10, 10u8.gcd(20));
    /// assert_eq!(44, 2024u32.gcd(748));
    /// ```
    ///
    /// [greatest common divisor]: https://en.wikipedia.org/wiki/Greatest_common_divisor
    #[must_use]
    fn gcd(self, other: Self) -> Self;
}

macro_rules! gcd_impl {
    ($($t:ty),*) => {
        $(
            impl Gcd for $t {
                #[inline]
                fn gcd(self, other: Self) -> Self {
                    let (mut a, mut b) = (self, other);
                    while b != 0 {
                        let t = b;
                        b = a % b;
                        a = t;
                    }
                    a
                }
            }
        )*
    };
}

gcd_impl!(u8, u16, u32, u64, u128, usize);

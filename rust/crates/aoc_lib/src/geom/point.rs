use std::fmt;

/// A 2-dimensional point with coordinates of type `T`.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    /// The x-coordinate of the point.
    pub x: T,
    /// The y-coordinate of the point.
    pub y: T,
}

impl<T> Point2D<T> {
    /// Create a new 2-dimensional point with the given `x` and `y` coordinates.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> fmt::Debug for Point2D<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

/// Shorthand for `Point2D::new(x, y)`.
#[inline]
pub const fn point2<T>(x: T, y: T) -> Point2D<T> {
    Point2D { x, y }
}

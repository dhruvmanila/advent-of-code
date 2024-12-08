use std::fmt;

use super::{point2, Point2D};

/// A 2-dimensional vector with components of type `T`.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Vector2D<T> {
    /// The delta in the x direction.
    pub dx: T,
    /// The delta in the y direction.
    pub dy: T,
}

impl<T> Vector2D<T> {
    /// Create a new 2-dimensional vector with the given `dx` and `dy` components.
    #[inline]
    pub const fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

impl<T: Copy> Vector2D<T> {
    /// Create a vector that points from the start to the end point.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::geom::{vec2, point2, Vector2D};
    /// let start = point2(1, 2);
    /// let end = point2(3, 4);
    ///
    /// assert_eq!(Vector2D::between_points(&start, &end), vec2(2, 2));
    /// ```
    pub fn between_points(start: &Point2D<T>, end: &Point2D<T>) -> Vector2D<T::Output>
    where
        T: std::ops::Sub,
    {
        vec2(end.x - start.x, end.y - start.y)
    }

    /// Translate the given point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use aoc_lib::geom::{vec2, point2};
    /// let p = point2(1, 2);
    /// let vec = vec2(3, 4);
    ///
    /// assert_eq!(vec.transform_point(&p), point2(4, 6));
    /// ```
    pub fn transform_point(&self, p: &Point2D<T>) -> Point2D<T::Output>
    where
        T: std::ops::Add,
    {
        point2(p.x + self.dx, p.y + self.dy)
    }
}

impl<T> fmt::Debug for Vector2D<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.dx).field(&self.dy).finish()
    }
}

/// Shorthand for `Vector2D::new(x, y)`.
#[inline]
pub const fn vec2<T>(dx: T, dy: T) -> Vector2D<T> {
    Vector2D { dx, dy }
}

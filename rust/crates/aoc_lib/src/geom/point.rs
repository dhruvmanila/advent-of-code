use std::fmt;
use std::ops::{Mul, Sub};

/// A 2-dimensional point with coordinates of type `T`.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point2D<T> {
    /// The x-coordinate of the point.
    pub x: T,
    /// The y-coordinate of the point.
    pub y: T,
}

impl<T> Point2D<T> {
    /// Create a new 2-dimensional point with the given `x` and `y` coordinates.
    #[inline]
    pub const fn new(x: T, y: T) -> Point2D<T> {
        Self { x, y }
    }

    /// Returns the determinant of the two points.
    #[inline]
    pub fn determinant(&self, other: &Point2D<T>) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Copy,
    {
        self.x * other.y - self.y * other.x
    }

    /// Returns the Euclidean distance between this point and another.
    #[inline]
    pub fn distance(&self, other: &Point2D<T>) -> f64
    where
        T: Sub<Output = T> + Into<f64> + Copy,
    {
        self.squared_distance(other).sqrt()
    }

    /// Returns the squared Euclidean distance between this point and another.
    #[inline]
    pub fn squared_distance(&self, other: &Point2D<T>) -> f64
    where
        T: Sub<Output = T> + Into<f64> + Copy,
    {
        let dx: f64 = (self.x - other.x).into();
        let dy: f64 = (self.y - other.y).into();
        dx * dx + dy * dy
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

/// A 3-dimensional point with coordinates of type `T`.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Point3D<T> {
    /// The x-coordinate of the point.
    pub x: T,
    /// The y-coordinate of the point.
    pub y: T,
    /// The z-coordinate of the point.
    pub z: T,
}

impl<T> Point3D<T> {
    /// Create a new 3-dimensional point with the given `x`, `y`, and `z` coordinates.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Point3D<T> {
        Self { x, y, z }
    }

    /// Returns the Euclidean distance between this point and another.
    #[inline]
    pub fn distance(&self, other: &Point3D<T>) -> f64
    where
        T: Sub<Output = T> + Into<f64> + Copy,
    {
        self.squared_distance(other).sqrt()
    }

    /// Returns the squared Euclidean distance between this point and another.
    #[inline]
    pub fn squared_distance(&self, other: &Point3D<T>) -> f64
    where
        T: Sub<Output = T> + Into<f64> + Copy,
    {
        let dx: f64 = self.x.into() - other.x.into();
        let dy: f64 = self.y.into() - other.y.into();
        let dz: f64 = self.z.into() - other.z.into();
        dx * dx + dy * dy + dz * dz
    }
}

impl<T> fmt::Debug for Point3D<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

/// Shorthand for `Point3D::new(x, y, z)`.
#[inline]
pub const fn point3<T>(x: T, y: T, z: T) -> Point3D<T> {
    Point3D { x, y, z }
}

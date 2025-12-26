//! Rational number type for exact arithmetic.
//!
//! This module provides a `Rational` type that represents fractions with exact arithmetic,
//! avoiding floating-point errors. Useful for Gaussian elimination and other algorithms
//! that require precise division.

use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::Gcd;

/// A rational number represented as a fraction `num/den`.
///
/// Invariants maintained:
/// - Denominator is always positive
/// - Fraction is always in lowest terms (GCD of numerator and denominator is 1)
/// - Zero is represented as 0/1
#[derive(Clone, Copy)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    /// The additive identity (0/1).
    pub const ZERO: Rational = Rational {
        numerator: 0,
        denominator: 1,
    };

    /// The multiplicative identity (1/1).
    pub const ONE: Rational = Rational {
        numerator: 1,
        denominator: 1,
    };

    /// The maximum representable rational number (``i64::MAX`` / 1).
    pub const MAX: Rational = Rational {
        numerator: i64::MAX,
        denominator: 1,
    };

    /// Creates a new rational number from a numerator and denominator.
    ///
    /// The result is automatically reduced to lowest terms with a positive denominator.
    ///
    /// # Panics
    ///
    /// Panics if `denominator` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aoc_lib::Rational;
    /// assert_eq!(Rational::new(4, 6).into_raw_parts(), (2, 3));
    /// assert_eq!(Rational::new(3, -6).into_raw_parts(), (-1, 2));
    /// assert_eq!(Rational::new(-3, 6).into_raw_parts(), (-1, 2));
    /// assert_eq!(Rational::new(-3, -6).into_raw_parts(), (1, 2));
    /// assert_eq!(Rational::new(0, 5).into_raw_parts(), (0, 1));
    /// assert_eq!(Rational::new(5, 5).into_raw_parts(), (1, 1));
    /// ```
    #[must_use]
    pub fn new(numerator: i64, denominator: i64) -> Rational {
        assert!(denominator != 0, "denominator cannot be zero");

        if numerator == 0 {
            return Rational::ZERO;
        }

        if numerator == denominator {
            return Rational::ONE;
        }

        let gcd = numerator.gcd(denominator);
        let sign = if denominator < 0 { -1 } else { 1 };

        Rational {
            numerator: sign * (numerator / gcd),
            denominator: sign * (denominator / gcd),
        }
    }

    /// Creates a rational number from an integer.
    #[must_use]
    pub const fn from_int(n: i64) -> Rational {
        Rational {
            numerator: n,
            denominator: 1,
        }
    }

    /// Decomposes the rational into its raw numerator and denominator.
    #[must_use]
    pub const fn into_raw_parts(self) -> (i64, i64) {
        (self.numerator, self.denominator)
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(&self) -> i64 {
        self.numerator
    }

    /// Returns the denominator (always positive).
    #[must_use]
    pub const fn denominator(&self) -> i64 {
        self.denominator
    }

    /// Returns true if this rational represents an integer (denominator is 1).
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        self.denominator == 1
    }

    /// Converts to an [i64] if this rational is an integer.
    ///
    /// Returns `None` if the denominator is not 1.
    #[must_use]
    pub const fn to_i64(&self) -> Option<i64> {
        if self.denominator == 1 {
            Some(self.numerator)
        } else {
            None
        }
    }

    /// Returns true if this rational is zero.
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    /// Returns true if this rational is positive (> 0).
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    /// Returns true if this rational is negative (< 0).
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.numerator < 0
    }

    /// Returns the absolute value of this rational.
    #[must_use]
    pub const fn abs(&self) -> Rational {
        Rational {
            numerator: self.numerator.abs(),
            denominator: self.denominator,
        }
    }

    /// Returns the reciprocal (1 / self).
    ///
    /// # Panics
    ///
    /// Panics if self is zero.
    #[must_use]
    pub fn reciprocal(&self) -> Rational {
        assert!(!self.is_zero(), "cannot take reciprocal of zero");
        let sign = if self.numerator < 0 { -1 } else { 1 };
        Rational {
            numerator: sign * self.denominator,
            denominator: self.numerator.abs(),
        }
    }
}

impl Default for Rational {
    fn default() -> Self {
        Rational::ZERO
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Rational {
        Rational::from_int(n)
    }
}

impl From<i32> for Rational {
    fn from(n: i32) -> Rational {
        Rational::from_int(i64::from(n))
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        // a/b + c/d = (a*d + c*b) / (b*d)
        Rational::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Rational) {
        *self = *self + other;
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        // a/b - c/d = (a*d - c*b) / (b*d)
        Rational::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Rational) {
        *self = *self - other;
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        // a/b * c/d = (a*c) / (b*d)
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Rational) {
        *self = *self * other;
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        // a/b / c/d = (a*d) / (b*c)
        assert!(!other.is_zero(), "division by zero");
        Rational::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Rational) {
        *self = *self / other;
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Rational) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Rational) -> Ordering {
        // Compare a/b with c/d by comparing a*d with c*b (this could overflow for large values)
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Rational {}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl<'a> Sum<&'a Rational> for Rational {
    fn sum<I: Iterator<Item = &'a Rational>>(iter: I) -> Rational {
        iter.fold(Rational::ZERO, |acc, &x| acc + x)
    }
}

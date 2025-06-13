//! A simple crate for working with fractions

use std::fmt;

/// Error types for fraction operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractionError {
    /// Attempted to create a fraction with zero denominator
    ZeroDenominator,
    /// Division by zero fraction
    DivisionByZero,
}

impl fmt::Display for FractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FractionError::ZeroDenominator => write!(f, "denominator cannot be zero"),
            FractionError::DivisionByZero => write!(f, "cannot divide by zero"),
        }
    }
}

impl std::error::Error for FractionError {}

/// A fraction with numerator and denominator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    /// Creates a new fraction.
    pub fn new(numerator: i64, denominator: i64) -> Result<Self, FractionError> {
        if denominator == 0 {
            return Err(FractionError::ZeroDenominator);
        }

        // Normalize sign to numerator
        let (num, den) = if denominator < 0 {
            (-numerator, -denominator)
        } else {
            (numerator, denominator)
        };

        Ok(Self {
            numerator: num,
            denominator: den,
        })
    }

    /// Returns the reciprocal of the fraction.
    pub fn reciprocal(&self) -> Result<Self, FractionError> {
        if self.numerator == 0 {
            return Err(FractionError::DivisionByZero);
        }
        Self::new(self.denominator, self.numerator)
    }

    /// Adds two fractions.
    pub fn add(&self, other: &Self) -> Self {
        Self {
            numerator: self.numerator * other.denominator + other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
        }
    }

    /// Subtracts two fractions.
    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            numerator: self.numerator * other.denominator - other.numerator * self.denominator,
            denominator: self.denominator * other.denominator,
        }
    }

    /// Multiplies two fractions.
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            numerator: self.numerator * other.numerator,
            denominator: self.denominator * other.denominator,
        }
    }

    /// Divides two fractions.
    pub fn divide(&self, other: &Self) -> Result<Self, FractionError> {
        let recip = other.reciprocal()?;
        Ok(self.multiply(&recip))
    }

    /// Reduces the fraction to lowest terms.
    pub fn reduce(&self) -> Self {
        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        Self {
            numerator: self.numerator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

/// Calculates the greatest common divisor using Euclid's algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reduced = self.reduce();
        if reduced.denominator == 1 {
            write!(f, "{}", reduced.numerator)
        } else {
            write!(f, "{}/{}", reduced.numerator, reduced.denominator)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(Fraction::new(1, 2).is_ok());
        assert_eq!(Fraction::new(1, 0), Err(FractionError::ZeroDenominator));
    }

    #[test]
    fn test_negative_denominator() {
        let frac = Fraction::new(1, -2).unwrap();
        assert_eq!(frac.numerator, -1);
        assert_eq!(frac.denominator, 2);
    }

    #[test]
    fn test_reduce() {
        let frac = Fraction::new(12, 8).unwrap();
        let reduced = frac.reduce();
        assert_eq!(reduced.numerator, 3);
        assert_eq!(reduced.denominator, 2);
    }

    #[test]
    fn test_display() {
        assert_eq!(Fraction::new(3, 4).unwrap().to_string(), "3/4");
        assert_eq!(Fraction::new(4, 2).unwrap().to_string(), "2");
        assert_eq!(Fraction::new(-3, 4).unwrap().to_string(), "-3/4");
    }

    #[test]
    fn test_arithmetic() {
        let half = Fraction::new(1, 2).unwrap();
        let third = Fraction::new(1, 3).unwrap();

        let sum = half.add(&third).reduce();
        assert_eq!(sum.numerator, 5);
        assert_eq!(sum.denominator, 6);

        let diff = half.subtract(&third).reduce();
        assert_eq!(diff.numerator, 1);
        assert_eq!(diff.denominator, 6);

        let product = half.multiply(&third).reduce();
        assert_eq!(product.numerator, 1);
        assert_eq!(product.denominator, 6);

        let quotient = half.divide(&third).unwrap().reduce();
        assert_eq!(quotient.numerator, 3);
        assert_eq!(quotient.denominator, 2);
    }
}

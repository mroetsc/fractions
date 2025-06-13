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
}

use std::ops::{Mul, MulAssign};

use crate::{baseunit, unit::prefix::Prefix, Number, Unit};

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUnit {
    unit: Unit,
    /// the extra number of the unit.
    ///
    /// for example, if we're parsing hr, then this would be 3600 and unit would be s.
    /// if we're parsing min, then this would be 60 and unit would be s.
    /// etc. etc.
    n: f64,
}

#[derive(Debug)]
pub enum ParserError {
    NotFound(String),
}

impl ParsedUnit {
    pub fn parse(unit_str: &str) -> Result<Self, ParserError> {
        let (exp, prefixless_unit_str) = match Prefix::parse_prefix(unit_str) {
            Some((prefix, unit)) => (prefix, unit),
            None => (0, unit_str),
        };

        let mut parsed_unit = match baseunit::parse(prefixless_unit_str) {
            Ok(unit) => unit,
            Err(ParserError::NotFound(_)) => baseunit::parse(unit_str)?,
        };

        parsed_unit.mult(10f64.powi(exp.into()));

        Ok(parsed_unit)
    }

    fn mult(&mut self, n: f64) {
        self.n *= n;
    }

    pub fn new(unit: Unit, n: f64) -> Self {
        Self { unit, n }
    }
}

impl Number {
    pub fn parse_and_add_unit(&mut self, unit: &str) -> Result<(), ParserError> {
        let parsed = ParsedUnit::parse(unit)?;
        self.value *= parsed.n;
        self.unit *= parsed.unit;

        Ok(())
    }
}

impl Mul<ParsedUnit> for Number {
    type Output = Self;

    fn mul(mut self, rhs: ParsedUnit) -> Self::Output {
        self.value *= rhs.n;
        self.unit *= rhs.unit;
        self
    }
}

impl MulAssign<ParsedUnit> for Number {
    fn mul_assign(&mut self, rhs: ParsedUnit) {
        self.value *= rhs.n;
        self.unit *= rhs.unit;
    }
}

#[cfg(test)]
mod tests {
    use crate::BaseUnits;

    use super::*;
    #[test]
    fn parsing_units() {
        assert_eq!(
            ParsedUnit::parse("s").unwrap(),
            ParsedUnit {
                unit: Unit::from_map(&[(BaseUnits::Time, 1)]),
                n: 1.0,
            }
        );

        assert_eq!(
            ParsedUnit::parse("kg").unwrap(),
            ParsedUnit {
                unit: Unit::from_map(&[(BaseUnits::Mass, 1)]),
                n: 1e3,
            }
        );

        assert_eq!(
            ParsedUnit::parse("um").unwrap(),
            ParsedUnit {
                unit: Unit::from_map(&[(BaseUnits::Length, 1)]),
                n: 1e-6,
            }
        );

        assert_eq!(
            ParsedUnit::parse("N").unwrap(),
            ParsedUnit {
                unit: Unit::from_map(&[
                    (BaseUnits::Length, 1),
                    (BaseUnits::Mass, 1),
                    (BaseUnits::Time, -2)
                ]),
                n: 1e3,
            }
        );

        assert_eq!(
            ParsedUnit::parse("mN").unwrap(),
            ParsedUnit {
                unit: Unit::from_map(&[
                    (BaseUnits::Length, 1),
                    (BaseUnits::Mass, 1),
                    (BaseUnits::Time, -2)
                ]),
                n: 1.0,
            }
        );
    }
}

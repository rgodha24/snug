use std::{
    ops::{Mul, MulAssign},
    str::FromStr,
};

use crate::{Number, Unit};

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
    NotFound,
}

impl FromStr for ParsedUnit {
    type Err = ParserError;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl ParsedUnit {
    fn combine(_: &[Self]) -> Self {
        todo!()
    }
}

impl Number {
    pub fn parse_and_add_unit(&mut self, unit: &str) -> Result<(), ParserError> {
        let parsed = unit.parse::<ParsedUnit>()?;
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

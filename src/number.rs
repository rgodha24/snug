use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::{
    unit::parse::{ParsedUnit, ParserError},
    Unit,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Number {
    /// the number of the unit
    pub value: f64,
    /// the unit of the number
    pub unit: Unit,
}

impl Number {
    pub fn new(value: f64, unit: &str) -> Result<Self, ParserError> {
        let base_unit = ParsedUnit::parse(unit)?;

        Ok(Self {
            value: value * base_unit.n,
            unit: base_unit.unit,
        })
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, other: Number) -> Number {
        if self.unit == other.unit {
            Number {
                value: self.value + other.value,
                unit: self.unit,
            }
        } else {
            panic!(
                "Cannot add two numbers with different units ({} and {})",
                self.unit, other.unit
            );
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        if self.unit == other.unit {
            Number {
                value: self.value - other.value,
                unit: self.unit,
            }
        } else {
            panic!(
                "Cannot subtract two numbers with different units ({} and {})",
                self.unit, other.unit
            );
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        Number {
            value: self.value * other.value,
            unit: self.unit * other.unit,
        }
    }
}

impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        Number {
            value: self.value / other.value,
            unit: self.unit / other.unit,
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

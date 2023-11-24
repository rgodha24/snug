use std::str::FromStr;

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

impl FromStr for ParsedUnit {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Number {
    pub fn parse_and_add_unit(&mut self, unit: &str) -> Result<(), ()> {
        let parsed = unit.parse::<ParsedUnit>()?;
        self.value *= parsed.n;
        self.unit *= parsed.unit;

        Ok(())
    }

    pub fn add_unit(&mut self, unit: Unit) {
        self.unit *= unit;
    }
}

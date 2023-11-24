use crate::{
    unit::parse::{ParsedUnit, ParserError},
    Unit,
};

use super::macros::UnitType;

pub fn parse(input: &str) -> Result<ParsedUnit, ParserError> {
    let ut = UnitType::new(input)?;

    let num = ut.num();
    let den = ut.den();

    let unit = Unit::from_frac(&num, &den);

    Ok(ParsedUnit::new(unit, ut.val()))
}

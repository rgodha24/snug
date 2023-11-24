#[macro_use]
mod macros;
mod parse;

pub use parse::parse;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseUnits {
    /// second
    Time,
    /// meter
    Length,
    /// technically kg, but we're using g bc prefixes are stored elsewhere
    Mass,
    /// technically SI uses Current, but Amps are current / time anyways ...
    Charge,
    /// celsius (technically should be Kelvin, but we don't like Kelvin here)
    Temp,
    /// candela (fake unit)
    Luminosity,
    /// degrees
    Angle,
}

impl Into<String> for &BaseUnits {
    fn into(self) -> String {
        match self {
            BaseUnits::Time => "s".to_string(),
            BaseUnits::Length => "m".to_string(),
            BaseUnits::Mass => "g".to_string(),
            BaseUnits::Charge => "C".to_string(),
            BaseUnits::Temp => "°C".to_string(),
            BaseUnits::Luminosity => "cd".to_string(),
            BaseUnits::Angle => "°".to_string(),
        }
    }
}

impl Display for BaseUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.into();
        write!(f, "{}", s)
    }
}

impl From<usize> for BaseUnits {
    fn from(i: usize) -> Self {
        match i {
            0 => BaseUnits::Time,
            1 => BaseUnits::Length,
            2 => BaseUnits::Mass,
            3 => BaseUnits::Charge,
            4 => BaseUnits::Temp,
            5 => BaseUnits::Luminosity,
            6 => BaseUnits::Angle,
            _ => panic!("Invalid base unit index"),
        }
    }
}

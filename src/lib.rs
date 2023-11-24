use std::{
    fmt::Display,
    ops::{Add, Div, Index, Mul, Sub},
};

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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Unit {
    /// the units indexed by their base unit
    units: [i8; 7],
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Number {
    /// the number of the unit
    pub value: f64,
    /// the unit of the number
    pub unit: Unit,
}

impl Unit {
    pub fn from_map(map: &[(BaseUnits, i8)]) -> Self {
        let mut units = [0; 7];

        for (base, power) in map {
            units[*base as usize] = *power;
        }

        Self { units }
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

impl Mul for Unit {
    type Output = Unit;

    fn mul(mut self, other: Unit) -> Unit {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit += other.units[i];
        }

        self
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

impl Div for Unit {
    type Output = Unit;

    fn div(mut self, other: Unit) -> Unit {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit -= other.units[i];
        }

        self
    }
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

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = String::new();
        let mut den = String::new();

        for (i, unit) in self.units.iter().enumerate() {
            match unit {
                0 => continue,
                1 => num.push_str(&format!("{}", BaseUnits::from(i))),
                -1 => den.push_str(&format!("{}", BaseUnits::from(i))),
                i8::MIN..=-2 => den.push_str(&format!("{}^{}", BaseUnits::from(i), unit)),
                2..=i8::MAX => num.push_str(&format!("{}^{}", BaseUnits::from(i), unit)),
            }
        }

        if num.is_empty() {
            num.push('1');
        }

        if den.is_empty() {
            write!(f, "{}", num)
        } else {
            write!(f, "{} / {}", num, den)
        }
    }
}

impl Index<&BaseUnits> for Unit {
    type Output = i8;

    fn index(&self, index: &BaseUnits) -> &Self::Output {
        &self.units[*index as usize]
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

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub numerator: Vec<BaseUnits>,
    pub denominator: Vec<BaseUnits>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Number {
    /// the number of the unit
    pub value: f64,
    /// the unit of the number
    pub unit: Unit,
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
        self.numerator.extend(other.numerator);
        self.denominator.extend(other.denominator);

        self.collapse();

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
        self.numerator.extend(other.denominator);
        self.denominator.extend(other.numerator);

        self.collapse();

        self
    }
}

impl Unit {
    fn collapse(&mut self) {
        for i in (0..self.numerator.len()).rev() {
            if self.denominator.contains(&self.numerator[i]) {
                self.numerator.remove(i);
                self.denominator.remove(i);
            }
        }
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
        for unit in &self.numerator {
            write!(f, "{unit}")?;
        }

        if self.numerator.is_empty() {
            write!(f, "1")?;
        }

        if !self.denominator.is_empty() {
            write!(f, "/")?;
        }

        for unit in &self.denominator {
            write!(f, "{unit}")?;
        }

        Ok(())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

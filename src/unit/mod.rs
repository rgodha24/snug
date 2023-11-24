mod parse;
mod prefix;

use std::{
    fmt::Display,
    ops::{Div, DivAssign, Index, IndexMut, Mul, MulAssign},
};

use crate::BaseUnits;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Unit {
    /// the units indexed by their base unit
    units: [i8; 7],
}

impl Unit {
    pub fn from_map(map: &[(BaseUnits, i8)]) -> Self {
        let mut units = [0; 7];

        for (base, power) in map {
            units[*base as usize] = *power;
        }

        Self { units }
    }

    pub fn from_frac(num: &[BaseUnits], denom: &[BaseUnits]) -> Self {
        let mut unit = Self::default();

        for n in num {
            unit[n] += 1;
        }
        for d in denom {
            unit[d] -= 1;
        }

        unit
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(mut self, rhs: Unit) -> Unit {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit += rhs.units[i];
        }

        self
    }
}

impl MulAssign for Unit {
    fn mul_assign(&mut self, rhs: Self) {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit += rhs.units[i]
        }
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(mut self, rhs: Unit) -> Unit {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit -= rhs.units[i];
        }

        self
    }
}

impl DivAssign for Unit {
    fn div_assign(&mut self, rhs: Self) {
        for (i, unit) in self.units.iter_mut().enumerate() {
            *unit -= rhs.units[i];
        }
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

impl IndexMut<&BaseUnits> for Unit {
    fn index_mut(&mut self, index: &BaseUnits) -> &mut Self::Output {
        &mut self.units[*index as usize]
    }
}

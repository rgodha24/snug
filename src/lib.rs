//! # Snug
//! a unit creation and manipulation library for rust
//!
//! ```
//! use snug::{BaseUnits, Number, Unit};
//! fn main() {
//!
//!    let force = Number::new(43.213, "N").unwrap();
//!    let mass = Number::new(12.0, "kg").unwrap();
//!    let time = Number::new(452.42, "ms").unwrap();
//!
//!    let acceleration = force / mass;
//!    println!("acceleration: {}", acceleration); // acceleration: 3.6010833333333334 m / s^-2
//!    assert_eq!(
//!        acceleration.unit,
//!        Unit::parse("m / s * s").unwrap()
//!    );
//!    assert!((acceleration.value - 3.6010833333333334).abs() < 1e-10);
//!
//!    let velocity = acceleration * time;
//!    println!("velocity: {}", velocity); // velocity: 1.6292021216666668 m / s
//!    assert_eq!(
//!        velocity.unit,
//!        Unit::parse("m / s").unwrap()
//!    );
//!    assert!((velocity.value - 1.6292021216666668).abs() < 1e-10);
//! }
//! ```
mod baseunit;

mod number;
pub mod unit;

pub use baseunit::BaseUnits;
pub use number::Number;
pub use unit::Unit;

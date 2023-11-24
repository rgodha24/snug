use snug::{unit::parse::ParserError, Number, Unit};

fn main() -> Result<(), ParserError> {
    let force = Number::new(43.213, "N")?;
    let mass = Number::new(12.0, "kg")?;
    let time = Number::new(452.42, "ms")?;

    let acceleration = force / mass;
    println!("acceleration: {}", acceleration); // acceleration: 3.6010833333333334 m / s^-2
    assert_eq!(acceleration.unit, Unit::parse("m / s * s")?);
    assert!((acceleration.value - 3.6010833333333334).abs() < 1e-10);

    let velocity = acceleration * time;
    println!("velocity: {}", velocity); // velocity: 1.6292021216666668 m / s
    assert_eq!(velocity.unit, Unit::parse("m / s")?);
    assert!((velocity.value - 1.6292021216666668).abs() < 1e-10);

    Ok(())
}

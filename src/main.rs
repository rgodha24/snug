use snug::{BaseUnits, Number, Unit};

fn main() {
    let force = Number::new(43.213, "N").unwrap();
    let mass = Number::new(12.0, "kg").unwrap();
    let time = Number::new(452.42, "ms").unwrap();

    let acceleration = force / mass;
    println!("acceleration: {}", acceleration); // acceleration: 3.6010833333333334 m / s^-2
    assert_eq!(
        acceleration.unit,
        Unit::from_frac(&[BaseUnits::Length], &[BaseUnits::Time, BaseUnits::Time])
    );
    assert!((acceleration.value - 3.6010833333333334).abs() < 1e-10);

    let velocity = acceleration * time;
    println!("velocity: {}", velocity); // velocity: 1.6292021216666668 m / s
    assert_eq!(
        velocity.unit,
        Unit::from_frac(&[BaseUnits::Length], &[BaseUnits::Time])
    );
    assert!((velocity.value - 1.6292021216666668).abs() < 1e-10);
}

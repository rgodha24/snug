use snug::{BaseUnits, Number, Unit};

fn main() {
    let len = Number {
        value: 10.0,
        unit: Unit {
            numerator: vec![BaseUnits::Length],
            denominator: vec![],
        },
    };

    let time = Number {
        value: 4.,
        unit: Unit {
            numerator: vec![BaseUnits::Time],
            denominator: vec![],
        },
    };

    let times_done = Number {
        value: 2.,
        unit: Default::default(),
    };

    let width = Number {
        value: 2.,
        unit: Unit {
            numerator: vec![BaseUnits::Length],
            denominator: vec![],
        },
    };

    let speed = dbg!(len / time * times_done * width);

    println!("speed: {}", speed);
}

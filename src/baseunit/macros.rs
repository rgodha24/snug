macro_rules! impl_unit_type {
    ($($name:ident, $($num:ident),*; $($den:ident),*; $val:expr, $($symbol:expr),+;);+) => {
        use crate::baseunit::BaseUnits;
        use crate::unit::parse::ParserError;

        pub enum UnitType {
            $($name),*
        }

        impl UnitType {
            pub fn new(input: &str) -> Result<Self, ParserError> {
                match input.trim() {
                    $($($symbol => Ok(UnitType::$name),)*)*
                    _ => Err(ParserError::NotFound(input.to_string())),
                }
            }
            pub fn num(&self) -> Vec<BaseUnits> {
                match self {
                    $(UnitType::$name => vec![$(BaseUnits::$num),*]),*
                }
            }
            pub fn den(&self) -> Vec<BaseUnits> {
                match self {
                    $(UnitType::$name => vec![$(BaseUnits::$den),*]),*
                }
            }
            pub fn val(&self) -> f64 {
                match self {
                    $(UnitType::$name => $val),*
                }
            }
        }


    };
}

impl_unit_type! {
    Second, Time; ; 1.0, "s";;
    Meter, Length; ; 1.0, "m";;
    Gram, Mass; ; 1.0, "g";;
    Coulomb, Charge; ; 1.0, "C";;
    Celsius, Temp; ; 1.0, "°C", "degC", "deg C";;
    Candela, Luminosity; ; 1.0, "cd";;
    Degrees, Angle; ; 1.0, "°", "deg", "deg.";;
    Newton, Mass, Length; Time, Time; 1000.0, "N", "Newton";
}

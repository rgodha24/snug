#[macro_use]
mod macros;

impl_prefix!(
    Quetta, "Q"; 30;
    Ronna, "R"; 27;
    Yotta, "Y"; 24;
    Zetta, "Z"; 21;
    Exa, "E"; 18;
    Peta, "P"; 15;
    Tera, "T"; 12;
    Giga, "G"; 9;
    Mega, "M"; 6;
    Kilo, "k"; 3;
    Hecto, "h"; 2;
    Deka, "da"; 1;
    Deci, "d"; -1;
    Centi, "c"; -2;
    Milli, "m"; -3;
    Micro, "μ", "u"; -6;
    Nano, "n"; -9;
    Pico, "p"; -12;
    Femto, "f"; -15;
    Atto, "a"; -18;
    Zepto, "z"; -21;
    Yocto, "y"; -24;
    Ronto, "r"; -27;
    Quonto, "q"; -30
);

impl<'a> Prefix {
    pub(crate) fn parse_prefix(s: &'a str) -> Option<(i8, &'a str)> {
        Self::parse_prefix_unchecked(s).and_then(|(p, s)| {
            if s.len() == 0 {
                // we parsed an actual unit as a prefix (e.g. m as only `Milli`,
                // not as PrefixLess Meter)
                None
            } else {
                Some((p.exp_of(), s))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() {
        assert_eq!(Prefix::parse_prefix("cm"), Some((-2, "m")));
        assert_eq!(Prefix::parse_prefix("m"), None);
    }
}

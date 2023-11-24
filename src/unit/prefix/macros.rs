macro_rules! impl_prefix {
    ($($prefix:ident, $($symbol:expr),+; $exp:expr);+) => {

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum Prefix {
            $($prefix),+
        }


        impl<'a> Prefix {
            pub fn exp_of(&self) -> i8 {
                match self {
                    $(Prefix::$prefix => $exp),+
                }
            }

            fn parse_prefix_unchecked(s: &'a str) -> Option<(Prefix, &'a str)> {
                $($(
                    if s.starts_with($symbol) {
                        return Some((Prefix::$prefix, &s[$symbol.len()..]));
                    }
                );+);+

                 None
            }
        }
    };
}

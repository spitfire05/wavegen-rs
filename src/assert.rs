macro_rules! assert_value {
    ($x:ident, $assert:ident) => {
        #[cfg(not(feature = "no_assert"))] {
            assert!($x.$assert(), "{} does not satisfy {}", stringify!($x), stringify!($assert))
        }
    };
}

macro_rules! assert_not_value {
    ($x:ident, $assert:ident) => {
        #[cfg(not(feature = "no_assert"))] {
            assert!(!$x.$assert(), "{} satisfies {}, while it should not", stringify!($x), stringify!($assert))
        }
    };
}

macro_rules! assert_periodic_params {
    ($frequency:ident, $amplitude:ident, $phase:ident) => {
            assert_value!($frequency, is_normal);
            assert_value!($frequency, is_sign_positive);

            assert_not_value!($amplitude, is_nan);
            assert_value!($amplitude, is_sign_positive);
            
            assert_not_value!($phase, is_nan);
            assert_value!($phase, is_finite);
    };
}

pub(crate) use assert_value;
pub(crate) use assert_not_value;
pub(crate) use assert_periodic_params;
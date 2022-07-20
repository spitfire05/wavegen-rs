use alloc::boxed::Box;

use crate::{PeriodicFunction, assert::{assert_not_value}};

pub fn _dc_bias(bias: f64) -> PeriodicFunction {
    assert_not_value!(bias, is_nan);

    Box::new(move |_| bias)
}

/// Builder macro for DC Bias [PeriodicFunction].
///
/// Takes just one argument - the bias value.
///
/// # Examples
///
/// Defines bias of amplitude +10
/// ```
/// use wavegen::dc_bias;
///
/// let bias = dc_bias!(10);
///
/// assert!((0..100000).all(|x| bias(x as f64) == 10.0))
/// ```
#[macro_export]
macro_rules! dc_bias {
    ($bias:expr) => {
        $crate::periodic_functions::bias::_dc_bias($bias as f64)
    };
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use crate::periodic_functions::test_utils::test_panic;

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y);
        for x in (0..10000000).map(|x| x as f64) {
            assert_eq!(dc(x), y);
        }
    }

    test_panic!{
        nan, amplitude, dc_bias!(f64::NAN)
    }
}

use alloc::boxed::Box;

use crate::{PeriodicFunction, assert::assert_periodic_params};
use crate::assert::{assert_value, assert_not_value};

pub fn _square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    assert_periodic_params!(frequency, amplitude, phase);

    _square_internal(frequency, amplitude, phase)
}

#[inline(always)]
#[cfg(all(not(feature = "libm"), feature = "std"))]
pub fn _square_internal(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    Box::new(move |t| {
        let power = (2.0 * (t - phase) * frequency).floor() as i32;

        amplitude * (-1f64).powi(power)
    })
}

#[inline(always)]
#[cfg(feature = "libm")]
pub fn _square_internal(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    use libm::{floor, pow};
    Box::new(move |t| amplitude * pow(-1.0, floor(2.0 * (t - phase) * frequency)))
}

/// Builder macro for Square [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
/// 
/// # Panics
/// 
/// This macro will cause panic if:
/// 
/// * `frequency` is not non-zero, positive, finite number.
/// * `amplitude` is `NaN` or negative.
/// * `phase` is `NaN` or non finite.
#[macro_export]
macro_rules! square {
    (frequency: $frequency:expr) => {
        square!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        square!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr) => {
        square!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::square::_square(
            $frequency as f64,
            $amplitude as f64,
            $phase as f64,
        )
    };
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::periodic_functions::test_utils::test_panic;
    use paste::paste;

    const EPS: f64 = 1e-3;

    #[test]
    fn default_square_has_amplitude_of_one() {
        let square = square!(1);

        for x in [0.0, 0.1, 0.2, 0.3, 0.4] {
            assert!(approx_eq!(f64, square(x), 1.0, epsilon = EPS))
        }

        for x in [0.5, 0.6, 0.7, 0.8, 0.9] {
            assert!(approx_eq!(f64, square(x), -1.0, epsilon = EPS))
        }
    }

    test_panic!{
        nan, frequency, square!(f64::NAN)
        nan, amplitude, square!(1, f64::NAN)
        nan, phase, square!(1, 1, f64::NAN)

        negative, frequency, square!(-1)
        negative, amplitude, square!(1, -1)

        zero, frequency, square!(0)

        infinite, frequency, square!(f64::INFINITY)
        infinite, phase, square!(1, 1, f64::INFINITY)

        neg_infinite, frequency, square!(f64::NEG_INFINITY)
        neg_infinite, phase, square!(1, 1, f64::NEG_INFINITY)
    }
}

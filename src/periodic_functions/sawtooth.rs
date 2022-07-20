use alloc::boxed::Box;

use crate::{PeriodicFunction, assert::assert_periodic_params};
use crate::assert::{assert_value, assert_not_value};

#[inline(always)]
#[cfg(feature = "std")]
fn frac(x: f64) -> f64 {
    // this is actually slower than `x - ((x as i64) as f64)` on x86_64-pc-windows-msvc target,
    // but faster than the "casting hack" when `target-cpu=native` (tested on skylake)
    x.fract()
}

#[inline(always)]
#[cfg(all(not(feature = "std"), feature = "libm"))]
fn frac(x: f64) -> f64 {
    use libm::modf;
    let (frac, _) = modf(x);

    frac
}

pub fn _sawtooth(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    assert_periodic_params!(frequency, amplitude, phase);

    Box::new(move |t| 2.0 * amplitude * frac(t * frequency + phase) - amplitude)
}

/// Builder macro for Sawtooth [PeriodicFunction].
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
macro_rules! sawtooth {
    ($frequency:expr) => {
        sawtooth!($frequency, 1.0, 0.0)
    };
    (frequency: $frequency:expr) => {
        sawtooth!($frequency)
    };
    ($frequency:expr, $amplitude:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sawtooth!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::sawtooth::_sawtooth(
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

    use super::frac;

    const EPS: f64 = 1e-3;

    #[test]
    fn frac_of_non_integer() {
        assert!(approx_eq!(f64, frac(1.5), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, frac(21.37), 0.37, epsilon = EPS));
        assert!(approx_eq!(f64, frac(42.69), 0.69, epsilon = EPS));
        assert!(approx_eq!(f64, frac(-5.55), -0.55, epsilon = EPS));
    }

    #[test]
    fn default_sawtooth_has_amplitude_of_one() {
        let f = sawtooth!(2.0);

        assert!(approx_eq!(f64, f(0.49999), 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, f(0.5), -1.0, epsilon = EPS));
    }

    test_panic!{
        nan, frequency, sawtooth!(f64::NAN)
        nan, amplitude, sawtooth!(1, f64::NAN)
        nan, phase, sawtooth!(1, 1, f64::NAN)

        negative, frequency, sawtooth!(-1)
        negative, amplitude, sawtooth!(1, -1)

        zero, frequency, sawtooth!(0)

        infinite, frequency, sawtooth!(f64::INFINITY)
        infinite, phase, sawtooth!(1, 1, f64::INFINITY)

        neg_infinite, frequency, sawtooth!(f64::NEG_INFINITY)
        neg_infinite, phase, sawtooth!(1, 1, f64::NEG_INFINITY)
    }
}

use core::f64::consts::PI;

use alloc::boxed::Box;

use crate::PeriodicFunction;
use crate::assert::{assert_value, assert_not_value};
use crate::assert::assert_periodic_params;

pub fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    assert_periodic_params!(frequency, amplitude, phase);

    _sine_internal(frequency, amplitude, phase)
}

#[inline(always)]
#[cfg(all(not(feature = "libm"), feature = "std"))]
pub fn _sine_internal(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| {
        let radians = (2.0 * PI * frequency * t) + (phase * 2.0 * PI);
        let sine = radians.sin();

        sine * amplitude
    })
}

#[inline(always)]
#[cfg(feature = "libm")]
pub fn _sine_internal(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    use libm::sin;
    Box::new(move |t| sin((2.0 * PI * frequency * t) + (phase * 2.0 * PI)) * amplitude)
}

/// Builder macro for Sine [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
///
/// # Examples
///
/// 50 Hz sine of amplitude 1 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(50);
/// ```
///
/// 50 Hz sine of amplitude 20 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(frequency: 50, amplitude: 20);
/// ```
///
/// 50 Hz sine of amplitude 20 and phase shift of half a turn
/// ```
/// use core::f64::consts::PI;
/// use wavegen::sine;
///
/// let sine = sine!(50, 20, 0.5);
/// ```
#[macro_export]
macro_rules! sine {
    (frequency: $frequency:expr) => {
        sine!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sine!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sine!($frequency, $amplitude, $phase)
    };
    ($frequency:expr) => {
        sine!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        sine!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::periodic_functions::sine::_sine($frequency as f64, $amplitude as f64, $phase as f64)
    };
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use paste::paste;

    use crate::periodic_functions::test_utils::test_panic;

    const EPS: f64 = 1e-3;

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = sine!(1);

        let max = sine(0.25);
        let min = sine(0.75);
        let zero = sine(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = sine!(1, 1, 0.5);

        let max = sine(0.75);
        let min = sine(0.25);
        let zero = sine(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    test_panic!{
        nan, frequency, sine!(f64::NAN)
        nan, amplitude, sine!(1, f64::NAN)
        nan, phase, sine!(1, 1, f64::NAN)

        negative, frequency, sine!(-1)
        negative, amplitude, sine!(1, -1)

        zero, frequency, sine!(0)
        
        infinite, frequency, sine!(f64::INFINITY)
        infinite, phase, sine!(1, 1, f64::INFINITY)

        neg_infinite, frequency, sine!(f64::NEG_INFINITY)
        neg_infinite, phase, sine!(1, 1, f64::NEG_INFINITY)
    }

}

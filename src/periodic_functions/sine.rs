use core::f64::consts::PI;

use alloc::boxed::Box;
use libm::sin;

use crate::PeriodicFunction;

pub fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
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
/// use wavy::sine;
///
/// let sine = sine!(50);
/// ```
///
/// 50 Hz sine of amplitude 20 and no phase shift
/// ```
/// use wavy::sine;
///
/// let sine = sine!(50, 20);
/// ```
///
/// 50 Hz sine of amplitude 20 and phase shift of half a turn
/// ```
/// use core::f64::consts::PI;
/// use wavy::sine;
///
/// let sine = sine!(50, 20, PI);
/// ```
#[macro_export]
macro_rules! sine {
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
}

use core::f32::consts::PI;

use alloc::boxed::Box;
use libm::sinf;

use crate::PeriodicFunction;

pub fn sine_builder(frequency: f32, amplitude: f32, phase: f32) -> PeriodicFunction {
    Box::new(move |t| sinf((2.0 * PI * frequency * t) + phase) * amplitude)
}

/// Builder macro for Sine [PeriodicFunction].
/// 
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
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
/// use core::f32::consts::PI;
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
        $crate::periodic_functions::sine::sine_builder($frequency as f32, $amplitude as f32, $phase as f32)
    };
}

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    use float_cmp::approx_eq;

    const EPS: f32 = 1e-3;

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = sine!(1);

        let max = sine(0.25);
        let min = sine(0.75);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = sine!(1, 1, PI);

        let max = sine(0.75);
        let min = sine(0.25);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }
}

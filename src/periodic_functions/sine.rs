use core::f32::consts::PI;

use alloc::boxed::Box;
use libm::sinf;

use crate::PeriodicFunction;

/// Builder for sine trygonometric function. Uses `libm::sinf` as backend generating sine.
pub struct Sine {
    frequency: f32,
    amplitude: f32,
    phase: f32,
}

impl Sine {
    /// Constructor method for the builder. Takes frequency in *Hz* as an input.
    /// Returns a builder for sine wave with amplitude of 1.0 and no phase shift.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use wavy::Sine;
    /// 
    /// // Simple 50 Hz sine wave
    /// let sine = Sine::new(50.0).build();
    /// ```
    pub fn new(frequency: f32) -> Self {
        Sine {
            frequency,
            amplitude: 1.0,
            phase: 0.0
        }
    }

    /// Adds amplitude to the sine builder.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use wavy::Sine;
    /// 
    /// // 50 Hz sine with an amplitude of 10.0
    /// let sine = Sine::new(50.0).with_amplitude(10.0).build();
    /// ```
    pub fn with_amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude;

        self
    }

    /// Adds phase shift in *rad* to the sine builder.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use wavy::Sine;
    /// use core::f32::consts::PI;
    /// 
    /// // 50 Hz sine with phase shift of half a turn (π *rad*)
    /// let sine = Sine::new(50.0).with_phase_shift(PI).build();
    /// ```
    pub fn with_phase_shift(mut self, phase: f32) -> Self {
        self.phase = phase;

        self
    }

    /// Builds and returns the [PeriodicFunction]. Consumes the builder.
    pub fn build(self) -> PeriodicFunction {
        Box::new(move |t| {
            sinf((2.0 * PI * self.frequency * t) + self.phase) * self.amplitude
        })
    }
}

/// Builder function for sine [PeriodicFunction]
pub fn sinef(frequency: f32, amplitude: f32, phase: f32) -> PeriodicFunction {
    Box::new(move |t| sinf((2.0 * PI * frequency * t) + phase) * amplitude)
}

/// Macro simplyfying [sinef] calls.
#[macro_export]
macro_rules! sine {
    ($frequency:expr) => {
        sinef($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        sinef($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        sinef($frequency, $amplitude, $phase)
    };
}



#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    use float_cmp::approx_eq;

    use crate::periodic_functions::sine::sinef;

    use super::Sine;

    const EPS: f32 = 1e-3;

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = sinef(1.0, 0.0, 0.0);

        let max = sine(0.25);
        let min = sine(0.75);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = Sine::new(1.0).with_phase_shift(PI).build();

        let max = sine(0.75);
        let min = sine(0.25);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }
}

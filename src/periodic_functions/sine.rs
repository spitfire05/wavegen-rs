use core::f32::consts::PI;

use alloc::boxed::Box;
use libm::sinf;

use crate::PeriodicFunction;

pub struct Sine {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    dc_bias: f32,
}

impl Sine {
    pub fn new(frequency: f32) -> Self {
        Sine {
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            dc_bias: 0.0,
        }
    }

    pub fn with_amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude;

        self
    }

    pub fn with_phase(mut self, phase: f32) -> Self {
        self.phase = phase;

        self
    }

    pub fn with_dc_bias(mut self, dc_bias: f32) -> Self {
        self.dc_bias = dc_bias;

        self
    }

    pub fn build(self) -> PeriodicFunction {
        Box::new(move |t| {
            (sinf((2.0 * PI * self.frequency * t) + self.phase) * self.amplitude) + self.dc_bias
        })
    }
}

#[cfg(test)]
mod tests {
    use core::{f32::consts::PI, ops::Deref};

    use float_cmp::{ApproxEq, approx_eq};

    use super::Sine;

    const EPS: f32 = 1e-3;

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = Sine::new(1.0).build();

        let max = sine(0.25);
        let min = sine(0.75);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }

    #[test]
    fn dc_bias_affects_min_max_amplitude() {
        let sine = Sine::new(1.0).with_dc_bias(1.0).build();

        let max = sine(0.25);
        let min = sine(0.75);

        assert_eq!(max, 2.0);
        assert_eq!(min, 0.0);
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = Sine::new(1.0).with_phase(PI).build();

        let max = sine(0.75);
        let min = sine(0.25);
        let zero = sine(0.5);

        assert!(approx_eq!(f32, max, 1.0, epsilon=EPS));
        assert!(approx_eq!(f32, min, -1.0, epsilon=EPS));
        assert!(approx_eq!(f32, zero, 0.0, epsilon=EPS));
    }
}

use core::f32::consts::PI;

use libm::sinf;

use super::PeriodicFunction;

pub struct Sine {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    dc_bias: f32,
}

impl Sine {
    pub fn new(frequency: f32, amplitude: f32, phase: f32, dc_bias: f32) -> Self {
        Sine { frequency, amplitude, phase, dc_bias }
    }

    pub fn with_frequency(frequency: f32) -> Self {
        Sine {
            frequency,
            amplitude: 1.0,
            phase: 0.0,
            dc_bias: 0.0
        }
    }
}

impl PeriodicFunction for Sine {
    fn sample(&self, t: f32) -> f32 {
        (sinf((2.0 * PI * self.frequency * t) + self.phase) * self.amplitude) + self.dc_bias
    }
}

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    use super::{Sine, PeriodicFunction};

    #[test]
    fn create_sine() {
        let _sine = Sine::with_frequency(0.0);
    }

    #[test]
    fn default_sine_has_amplitude_of_one() {
        let sine = Sine::with_frequency(1.0);
        
        let max = sine.sample(0.25);
        let min = sine.sample(0.75);
        
        assert_eq!(max, 1.0);
        assert_eq!(min, -1.0);
    }

    #[test]
    fn dc_bias_affects_min_max_amplitude() {
        let sine = Sine::new(1.0, 1.0, 0.0, 1.0);

        let max = sine.sample(0.25);
        let min = sine.sample(0.75);

        assert_eq!(max, 2.0);
        assert_eq!(min, 0.0);
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = Sine::new(1.0, 1.0, PI, 0.0);

        let max = sine.sample(0.75);
        let min = sine.sample(0.25);

        assert_eq!(max, 1.0);
        assert_eq!(min, -1.0);
    }
}
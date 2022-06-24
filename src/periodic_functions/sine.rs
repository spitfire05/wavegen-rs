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
        (sinf(2.0 * PI * self.frequency * t + self.phase) * self.amplitude) + self.dc_bias
    }
}

#[cfg(test)]
mod tests {
    use super::{Sine, PeriodicFunction};

    #[test]
    fn create_sine() {
        let _sine = Sine::with_frequency(0.0);
    }

    #[test]
    fn default_sine_has_amplitude_of_one() {
        let sine = Sine::with_frequency(1.0);
        
        let sample = sine.sample(0.25);
        assert_eq!(sample, 1.0);

        let sample = sine.sample(0.75);
        assert_eq!(sample, -1.0);
    }
}
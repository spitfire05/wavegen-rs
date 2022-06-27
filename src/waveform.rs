use core::marker::PhantomData;

use alloc::{vec, vec::Vec};
use num_traits::NumCast;

use crate::PeriodicFunction;

pub struct Waveform<T: Clone> {
    sample_rate: f32,
    components: Vec<PeriodicFunction>,
    _phantom: PhantomData<T>,
}

impl<T: Clone> Waveform<T> {
    pub fn new(sample_rate: f32) -> Self {
        Waveform {
            sample_rate,
            components: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn with_components(sample_rate: f32, components: Vec<PeriodicFunction>) -> Self {
        Waveform {
            sample_rate,
            components,
            _phantom: PhantomData,
        }
    }

    pub fn add_component(&mut self, component: PeriodicFunction) {
        self.components.push(component);
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

impl<'a, T: Clone + NumCast> IntoIterator for &'a Waveform<T> {
    type Item = T;

    type IntoIter = WaveformIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        WaveformIterator {
            inner: self,
            time: 0.0,
        }
    }
}

pub struct WaveformIterator<'a, T: Clone> {
    inner: &'a Waveform<T>,
    time: f32,
}

impl<'a, T: Clone + NumCast> Iterator for WaveformIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: normalize?
        let sample: f32 = self.inner.components.iter().map(|x| x(self.time)).sum();
        self.time += 1.0 / self.inner.sample_rate;
        NumCast::from(sample)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use super::Waveform;
    use crate::{sine, dc_bias};

    // TODO: needs more tests

    #[test]
    pub fn sine_waveform_has_default_amplitude_of_one() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1)]);

        let samples = wf.into_iter().take(100).collect::<Vec<f32>>();

        assert_eq!(samples[25], 1.0);
        assert_eq!(samples[75], -1.0);
    }

    #[test]
    pub fn sine_waveform_as_integers_has_amplitude_of_one() {
        let wf = Waveform::<i32>::with_components(100.0, vec![sine!(1)]);

        let samples = wf.into_iter().take(100).collect::<Vec<i32>>();

        assert_eq!(samples[25], 1);
        assert_eq!(samples[75], -1);
    }

    #[test]
    pub fn sine_waveform_with_bias_has_correct_amplitude() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1), dc_bias!(5)]);

        let samples = wf.into_iter().take(100).collect::<Vec<f32>>();

        assert_eq!(samples[25], 6.0);
        assert_eq!(samples[75], 4.0);
    }
}

use core::marker::PhantomData;

use alloc::{vec, vec::Vec, boxed::Box};
use num_traits::NumCast;

use crate::PeriodicFunction;

pub struct Waveform<BitDepth: Clone> {
    sample_rate: f32,
    components: Vec<Box<dyn PeriodicFunction>>,
    _phantom: PhantomData<BitDepth>
}

impl<BitDepth: Clone> Waveform<BitDepth> {
    pub fn new(sample_rate: f32) -> Self {
        Waveform { sample_rate, components: vec![], _phantom: PhantomData }
    }

    pub fn with_components(sample_rate: f32, components: Vec<Box<dyn PeriodicFunction>>) -> Self {
        Waveform { sample_rate, components, _phantom: PhantomData }
    }

    pub fn add_component(&mut self, component: Box<dyn PeriodicFunction>) {
        self.components.push(component);
    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }
}

impl<'a, BitDepth: Clone + NumCast> IntoIterator for &'a Waveform<BitDepth> {
    type Item = BitDepth;

    type IntoIter = WaveformIterator<'a, BitDepth>;

    fn into_iter(self) -> Self::IntoIter {
        WaveformIterator{
            inner: self,
            time: 0.0
        }
    }
}

pub struct WaveformIterator<'a, BitDepth: Clone> {
    inner: &'a Waveform<BitDepth>,
    time: f32,
}

impl<'a, BitDepth: Clone + NumCast> Iterator for WaveformIterator<'a, BitDepth> {
    type Item = BitDepth;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: normalize?
        let sample: f32 = self.inner.components.iter().map(|x| x.sample(self.time)).sum();
        self.time += 1.0 / self.inner.sample_rate;
        NumCast::from(sample)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec, boxed::Box};

    use super::Waveform;
    use crate::Sine;

    #[test]
    pub fn create_and_sample_sine_waveform() {
        let wf = Waveform::<f32>::with_components(100.0, vec![Box::new(Sine::with_frequency(1.0))]);

        let _samples = wf.into_iter().take(100).collect::<Vec<f32>>();

        let _foo = 0; // just a debug point
    }

    #[test]
    pub fn sine_waveform_as_integers_has_amplitude_of_one() {
        let wf = Waveform::<i32>::with_components(100.0, vec![Box::new(Sine::with_frequency(1.0))]);

        let samples = wf.into_iter().take(100).collect::<Vec<i32>>();

        assert_eq!(samples[25], 1)
    }
}
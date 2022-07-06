use core::marker::PhantomData;

use alloc::{vec, vec::Vec};

use num_traits::NumCast;

use crate::PeriodicFunction;

/// Struct representing a waveform, consisting of output numeric type, sampling rate and a vector of [PeriodicFunction]s.
pub struct Waveform<T: Clone> {
    sample_rate: f64,
    components: Vec<PeriodicFunction>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Clone> Waveform<T> {
    /// Initializes new empty [Waveform]
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::Waveform;
    ///
    /// let wf = Waveform::<f32>::new(500.0);
    ///
    /// assert!(wf.iter().take(100).all(|y| y == 0.0));
    /// ```
    pub fn new(sample_rate: f64) -> Self {
        Waveform {
            sample_rate,
            components: vec![],
            _phantom: PhantomData,
        }
    }

    /// Initializes new [Waveform] with predefined components
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1), dc_bias!(-50)]);
    /// ```
    pub fn with_components(sample_rate: f64, components: Vec<PeriodicFunction>) -> Self {
        Waveform {
            sample_rate,
            components,
            _phantom: PhantomData,
        }
    }

    /// Ads a new component to existing [Waveform].
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let mut wf = Waveform::<f32>::new(100.0);
    /// wf.add_component(sine!(10));
    /// wf.add_component(dc_bias!(5));
    ///
    /// assert_eq!(2, wf.get_components_len());
    /// ```
    pub fn add_component(&mut self, component: PeriodicFunction) {
        self.components.push(component);
    }

    /// Getter for sample rate of a [Waveform].
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::Waveform;
    ///
    /// let wf = Waveform::<f32>::new(42.0);
    ///
    /// assert_eq!(42.0, wf.get_sample_rate());
    /// ```
    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }

    /// Returns number of components this [Waveform] consists of.
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let wf = Waveform::<f32>::with_components(42.0, vec![sine!(1), dc_bias!(5)]);
    ///
    /// assert_eq!(2, wf.get_components_len());
    /// ```
    pub fn get_components_len(&self) -> usize {
        self.components.len()
    }

    /// Returns an iterator over this [Waveform] samples.
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let wf = Waveform::<f32>::with_components(42.0, vec![sine!(1)]);
    /// let samples = wf.iter().take(100).collect::<Vec<_>>();
    /// ```
    pub fn iter(&self) -> WaveformIterator<T> {
        WaveformIterator::<T> {
            inner: self,
            time: 0.0,
        }
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

#[derive(Clone, Copy)]
pub struct WaveformIterator<'a, T: Clone> {
    inner: &'a Waveform<T>,
    time: f64,
}

impl<'a, T: Clone + NumCast> Iterator for WaveformIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sample: f64 = self.inner.components.iter().map(|x| x(self.time)).sum();
        let new_time = self.time + 1.0 / self.inner.sample_rate;
        if new_time.is_finite() {
            self.time = new_time;
        } else {
            self.time = (1.0 / self.inner.sample_rate) - (f64::MAX - self.time);
        }

        NumCast::from(sample)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use float_cmp::approx_eq;

    use super::Waveform;
    use crate::{dc_bias, sawtooth, sine};

    const EPS: f32 = 1e-3;

    #[test]
    fn sine_waveform_has_default_amplitude_of_one() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1)]);

        let samples = wf.iter().take(100).collect::<Vec<_>>();

        assert_eq!(samples[25], 1.0);
        assert_eq!(samples[75], -1.0);
    }

    #[test]
    fn sine_waveform_as_integers_has_amplitude_of_one() {
        let wf = Waveform::<i32>::with_components(100.0, vec![sine!(1)]);

        let samples = wf.iter().take(100).collect::<Vec<_>>();

        assert_eq!(samples[25], 1);
        assert_eq!(samples[75], -1);
    }

    #[test]
    fn sine_waveform_with_bias_has_correct_amplitude() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1), dc_bias!(5)]);

        let samples = wf.iter().take(100).collect::<Vec<_>>();

        assert_eq!(samples[25], 6.0);
        assert_eq!(samples[75], 4.0);
    }

    #[test]
    fn default_sine_wavefrom_has_no_bias() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(10)]);

        let bias = wf.iter().take(100).sum::<f32>() / 100.0;

        assert!(approx_eq!(f32, bias, 0.0, epsilon = EPS));
    }

    #[test]
    fn default_sawtooth_wavefrom_has_no_bias() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sawtooth!(1)]);

        let bias = wf.iter().take(100).sum::<f32>() / 100.0;

        assert!(approx_eq!(f32, bias, 0.0, epsilon = 1e-2));
    }

    #[test]
    fn waveform_iterator_is_infinite() {
        let wf = Waveform::<f64>::new(f64::MIN_POSITIVE);
        let mut iter = wf.iter();
        let samples = iter.take(1e5 as usize).collect::<Vec<_>>();

        assert_eq!(1e5 as usize, samples.len());
        assert_eq!(Some(0f64), iter.next());
    }
}

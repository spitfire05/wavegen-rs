use core::marker::PhantomData;

use alloc::{vec, vec::Vec};

use num_traits::{Bounded, NumCast};

use crate::PeriodicFunction;

/// Struct representing a waveform, consisting of output numeric type, sampling rate and a vector of [PeriodicFunction]s.
pub struct Waveform<T: Clone> {
    sample_rate: f64,
    components: Vec<PeriodicFunction>,
    _phantom: PhantomData<T>,
}

impl<T: Clone> Waveform<T> {
    /// Initializes new empty [Waveform]
    ///
    /// # Panics
    ///
    /// This method will panic if `sample_rate` is not a finite, positive, non-zero number.
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
        assert!(sample_rate.is_normal());
        assert!(sample_rate.is_sign_positive());

        Waveform {
            sample_rate,
            components: vec![],
            _phantom: PhantomData,
        }
    }

    /// Initializes new [Waveform] with predefined components
    ///
    /// # Panics
    ///
    /// This method will panic if `sample_rate` is not a finite, positive, non-zero number.
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1), dc_bias!(-50)]);
    /// ```
    pub fn with_components(sample_rate: f64, components: Vec<PeriodicFunction>) -> Self {
        assert!(sample_rate.is_normal());
        assert!(sample_rate.is_sign_positive());

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

impl<'a, T: Clone + NumCast + Bounded> IntoIterator for &'a Waveform<T> {
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

impl<'a, T: Clone + NumCast + Bounded> Iterator for WaveformIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sample: f64 = self.inner.components.iter().map(|x| x(self.time)).sum();
        let new_time = self.time + 1.0 / self.inner.sample_rate;
        if new_time.is_finite() {
            self.time = new_time;
        } else {
            self.time = (1.0 / self.inner.sample_rate) - (f64::MAX - self.time);
        }

        let result = NumCast::from(sample);

        match result {
            Some(_) => result,
            None => {
                if sample > 0.0 {
                    Some(T::max_value())
                }
                else if sample < 0.0 {
                    Some(T::min_value())
                }
                else {
                    panic!("Sample {} cannot be converted to waveform type.", sample);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use float_cmp::approx_eq;
    use paste::paste;

    use super::Waveform;
    use crate::{dc_bias, sine, square};

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

    macro_rules! test_no_default_bias {
        ($($name:ident: $func:expr)*) => {
            $(
                paste! {
                    #[test]
                    fn [<default_ $name _waveforom_has_no_bias>]() {
                        let wf = Waveform::<f32>::with_components(100.0, vec![$func]);

                        let bias = wf.iter().take(100).sum::<f32>() / 100.0;

                        assert!(approx_eq!(f32, bias, 0.0, epsilon = EPS));
                    }
                }
            )*
        };
    }

    test_no_default_bias! {
        sine: sine!(1)
        // sawtooth: sawtooth!(1) // does not pass currently, see https://github.com/spitfire05/wavegen-rs/issues/17
        square: square!(1)
    }

    #[test]
    fn waveform_iterator_is_infinite() {
        let wf = Waveform::<f64>::new(f64::MIN_POSITIVE);
        let mut iter = wf.iter();
        let samples = iter.take(1e5 as usize).collect::<Vec<_>>();

        assert_eq!(1e5 as usize, samples.len());
        assert_eq!(Some(0f64), iter.next());
    }

    #[test]
    fn oversaturated_amplitude_clips_to_max() {
        let wf = Waveform::<u8>::with_components(100.0, vec![dc_bias!(300)]);
        let samples = wf.iter().take(1).collect::<Vec<_>>();

        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0], u8::MAX);
    }

    #[test]
    fn undersaturated_amplitude_clips_to_min() {
        let wf = Waveform::<u8>::with_components(100.0, vec![dc_bias!(-300)]);
        let samples = wf.iter().take(1).collect::<Vec<_>>();

        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0], u8::MIN);
    }

    macro_rules! test_wavefrom_panic {
        ($($name:ident: $sample_rate:expr)*) => {
            $(
                paste! {
                    #[test]
                    #[should_panic]
                    fn [<waveform_new_panics_on_ $name>]() {

                        Waveform::<f64>::new($sample_rate);
                    }


                    #[test]
                    #[should_panic]
                    fn [<waveform_with_components_panics_on_ $name>]() {
                        Waveform::<f64>::with_components($sample_rate, vec![]);
                    }
                }
            )*
        };
    }

    test_wavefrom_panic! {
        nan: f64::NAN
        negative: -1f64
        zero: 0.0
        infinity: f64::INFINITY
        negative_infinity: f64::NEG_INFINITY
    }
}

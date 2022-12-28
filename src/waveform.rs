use crate::PeriodicFunction;
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use num_traits::{float::FloatCore, Bounded, NumCast};

/// Helper trait defining all the types that can be used as [Waveform]'s sample type.
pub trait SampleType: NumCast + Bounded {}

impl<T> SampleType for T where T: NumCast + Bounded {}

/// Struct representing a waveform, consisting of output numeric type, sampling rate and a vector of [PeriodicFunction]s.
pub struct Waveform<T: SampleType, P: FloatCore = f64> {
    sample_rate: P,
    components: Vec<PeriodicFunction>,
    _phantom: PhantomData<T>,
}

impl<T: SampleType, P: FloatCore> Waveform<T, P> {
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
    pub fn new(sample_rate: impl Into<P>) -> Self {
        let sample_rate = sample_rate.into();
        Self::assert_sane(sample_rate);

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
    pub fn with_components(sample_rate: impl Into<P>, components: Vec<PeriodicFunction>) -> Self {
        let sample_rate = sample_rate.into();
        Self::assert_sane(sample_rate);

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

    /// Gets sample rate of this [`Waveform`].
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
    pub fn sample_rate(&self) -> &P {
        &self.sample_rate
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
    /// use wavegen::{Waveform, sine};
    ///
    /// let wf = Waveform::<f32>::with_components(42.0, vec![sine!(1)]);
    /// let samples = wf.iter().take(100).collect::<Vec<_>>();
    /// ```
    pub fn iter(&self) -> WaveformIterator<T, P> {
        WaveformIterator::<T, P> {
            inner: self,
            time: P::zero(),
        }
    }

    #[inline(always)]
    fn assert_sane(x: P) {
        assert!(x.is_normal());
        assert!(x.is_sign_positive());
    }
}

impl<'a, T: SampleType, P: FloatCore> IntoIterator for &'a Waveform<T, P> {
    type Item = T;

    type IntoIter = WaveformIterator<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        WaveformIterator {
            inner: self,
            time: P::zero(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct WaveformIterator<'a, T: SampleType, P: FloatCore> {
    inner: &'a Waveform<T, P>,
    time: P,
}

impl<'a, T: SampleType, P: FloatCore> WaveformIterator<'a, T, P> {
    fn into_target_type_sanitized(sample: f64) -> Option<T> {
        let result = NumCast::from(sample);

        result.or_else(|| {
            if sample > 0.0 {
                Some(T::max_value())
            } else if sample < 0.0 {
                Some(T::min_value())
            } else {
                None
            }
        })
    }

    fn increment_time(&mut self, n: usize) {
        let new_time = self.time + (n as f64 * (1.0 / self.inner.sample_rate));
        if new_time.is_finite() {
            self.time = new_time;
        } else {
            self.time = (P::one() / self.inner.sample_rate) - (P::max_value() - self.time);
        }
    }

    fn raw_sample(&self) -> f64 {
        self.inner.components.iter().map(|x| x(self.time)).sum()
    }
}

impl<'a, T: SampleType, P: FloatCore> Iterator for WaveformIterator<'a, T, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sample: f64 = self.raw_sample();
        self.increment_time(1);

        Self::into_target_type_sanitized(sample)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.increment_time(n);

        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use float_cmp::approx_eq;
    use paste::paste;

    use super::Waveform;
    use crate::{dc_bias, sawtooth, sine, square};

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
    #[allow(clippy::iter_skip_next)]
    fn waveform_iterator_is_infinite() {
        let wf = Waveform::<f64>::new(f64::MIN_POSITIVE);
        let mut iter = wf.iter().skip(usize::MAX);

        assert_eq!(Some(0f64), iter.next());
        assert_eq!(Some(0f64), iter.skip(usize::MAX).next())
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

    macro_rules! test_size_hint {
        () => {
            let wf = Waveform::<f32>::new(44100.0);
            assert_eq!((usize::MAX, None), wf.iter().size_hint());
        };
        ($($component:expr),*) => {
            let mut wf = Waveform::<f32>::new(44100.0);
            $(
                wf.add_component($component);
            )*
            assert_eq!((usize::MAX, None), wf.iter().size_hint());
        };
    }

    #[test]
    fn test_size_hint() {
        test_size_hint!();
        test_size_hint!(sine!(50));
        test_size_hint!(sine!(1), sawtooth!(2), square!(3), dc_bias!(4));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::unwrap_used)]
    fn nth_and_next_give_same_results() {
        let wf = Waveform::<i32>::with_components(44100.0, vec![sine!(3000, i32::MAX)]);
        let mut i1 = wf.iter();
        let mut i2 = wf.iter();

        for _ in 0..1000 {
            assert_eq!(i1.next().unwrap(), i2.nth(0).unwrap());
        }
    }

    #[test]
    fn waveform_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Waveform<f64>>();
    }

    #[test]
    fn waveform_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Waveform<f64>>();
    }
}

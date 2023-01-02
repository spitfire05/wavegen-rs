//! Rust waveform generator, with [`no_std` support](https://github.com/spitfire05/wavegen-rs#how-to-use-it).
//!
//! # Quickstart
//!
//! ```
//! use wavegen::{wf, sine, dc_bias, sawtooth};
//!
//! // Define a Waveform with 200Hz sampling rate and three function components,
//! // choosing f32 as the output type:
//! let waveform = wf!(f32, 200., sine!(50., 10.), sawtooth!(20.), dc_bias!(-5.));
//!
//! // Use Waveform as an infinite iterator:
//! let two_seconds_of_samples: Vec<f32> = waveform.iter().take(400).collect();
//! ```
//!
//! Look into macros section for a complete list of defined periodic functions and their constructors.
//!
//! # Periodic function macros
//! The macros for building predefined [`PeriodicFunction`]s generally have a form of:
//!
//! `function!(frequency, [amplitude, [phase]])`
//!
//! (Square braces "[]" indicate optional argument).
//!
//! They come in an annotated and non-annotated form, so for example a Sine function can be expressed in both ways:
//! ```
//! use wavegen::{wf, sine, PeriodicFunction};
//!
//! let _: PeriodicFunction<f32> = sine!(100., 20., 0.25);
//! ```
//!
//! ```
//! use wavegen::{wf, sine, PeriodicFunction};
//!
//! let _: PeriodicFunction<f32> = sine!(frequency: 100., amplitude: 20., phase: 0.25);
//! ```
//!
//! Refer to Macros section for more info.
//!
//! # Custom periodic functions
//! Supported, of course. Just define your custom function as `Box<Fn(f64) -> f64>` and use it with [`Waveform`].
//!
//! ```
//! use wavegen::{wf, PeriodicFunction};
//!
//! let waveform = wf!(f64, 100.0, PeriodicFunction::custom(|x| x % 2.0));
//! ```
//!
//! # Overflows
//!
//! As [`Waveform`] can be composed of multiple components, it is possible for it to overflow during samples collection.
//! If overflow occurs, the sample's value will be clamped to the largest possible representation of sample's type.
//!
//! That means `+/- Inf` for floating point types, and `MAX/MIN` for integers.
//!
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let wf = Waveform::<f64>::with_components(100.0, vec![dc_bias![f32::MAX], dc_bias![f32::MAX]]);
//! let sample = wf.iter().take(1).collect::<Vec<_>>()[0];
//!
//! assert_eq!(sample, f64::INFINITY);
//! ```
//!
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let wf = Waveform::<i32>::with_components(100.0, vec![dc_bias![f32::MAX], dc_bias![f32::MAX]]);
//! let sample = wf.iter().take(1).collect::<Vec<_>>()[0];
//!
//! assert_eq!(sample, i32::MAX);
//! ```
//!
//! # Calculation precision
//!
//! By default, all calculations in [`Waveform`] use single floating point precision [`f32`]. This can be set to [`f64`] if needed, possibly in case of very high frequencies. To do so, set the `P` type parameter to [`f64`]:
//!
//! ```
//! let double_precision_waveform = wavegen::Waveform::<f64, f64>::new(1e100);
//! ```
//!
//! # Iterator infinity
//!
//! [`WaveformIterator`] is a *mostly* infinite iterator, with one exception:
//!
//! The `WaveformIterator::next` method can return [`None`] in some rare cases if it is not able to convert the inner sample type [`f64`] into the target sample type.
//!
//! `f64::NAN` cannot be represented as [`i32`]:
//! ```
//! use wavegen::{Waveform, PeriodicFunction};
//!
//! let mut wf = Waveform::<i32, f64>::new(100.0);
//! wf.add_component(PeriodicFunction::dc_bias(f64::NAN));
//!
//! assert_eq!(None, wf.iter().next())
//! ```
//!
//! This however is fine, as `f64::NAN` can be represented as `f32::NAN`:
//! ```
//! use wavegen::{Waveform, PeriodicFunction};
//!
//! let mut wf = Waveform::<f32, f64>::new(100.0);
//! wf.add_component(PeriodicFunction::dc_bias(f64::NAN));
//!
//! assert!(wf.iter().next().unwrap().is_nan())
//! ```
//!
//! It is probably a good practice to sanitize the parameters of the periodic function before it is constructed.
//!
//! # Note about Nyquist-Shannon rule enforcement
//!
//! As a rule of thumb in signal processing, the sampling frequency should be *at least* 2 times bigger than the highest frequency of sampled continous signal.
//!
//! This lib will **not** enforce the Nyquist-Shannon rule on the waveforms you create, therefore abominations like this are possible (altough not recommended):
//!
//! ```
//! use wavegen::{Waveform, sine};
//!
//! // 100 Hz sampling of 80 Hz sine... will not yield realistic results.
//! let wf = Waveform::<f32>::with_components(100.0, vec![sine!(80.)]);
//! ```
//!
//! As it is often a case, it is you, the programmer, who's left in charge of making sure the input data makes sense.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
compile_error!("at least one of \"libm\", \"std\" features has to be enabled");

extern crate alloc;

mod macros;

use alloc::{boxed::Box, vec, vec::Vec};
use core::iter::Sum;
use core::marker::PhantomData;
use core::ops::Add;
use num_traits::{Bounded, Float, FloatConst, NumCast, One};

/// Defines precision of inner [`Waveform`] and [`PeriodicFunction`] calcualtions.
pub trait Precision: Float + FloatConst + Sum + Send + Sync + 'static {}

impl<T> Precision for T where T: Float + FloatConst + Sum + Send + Sync + 'static {}

trait Two {
    fn two() -> Self;
}

impl<T> Two for T
where
    T: One + Add<Output = T>,
{
    #[inline]
    fn two() -> Self {
        T::one() + T::one()
    }
}

/// Helper trait defining all the types that can be used as [`Waveform`]'s sample type.
pub trait SampleType: NumCast + Bounded {}

impl<T> SampleType for T where T: NumCast + Bounded {}

/// Struct representing a waveform, consisting of output numeric type, sampling rate and a vector of [`PeriodicFunction`]s.
pub struct Waveform<T: SampleType, P: Precision = f32> {
    sample_rate: P,
    components: Vec<PeriodicFunction<P>>,
    _phantom: PhantomData<T>,
}

impl<T: SampleType, P: Precision> Waveform<T, P> {
    /// Initializes new empty [`Waveform`]
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

    /// Initializes new [`Waveform`] with predefined components
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
    /// let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1.), dc_bias!(-50.)]);
    /// ```
    pub fn with_components(
        sample_rate: impl Into<P>,
        components: Vec<PeriodicFunction<P>>,
    ) -> Self {
        let sample_rate = sample_rate.into();
        Self::assert_sane(sample_rate);

        Waveform {
            sample_rate,
            components,
            _phantom: PhantomData,
        }
    }

    /// Ads a new component to existing [`Waveform`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let mut wf = Waveform::<f32>::new(100.0);
    /// wf.add_component(sine!(10.));
    /// wf.add_component(dc_bias!(5.));
    ///
    /// assert_eq!(2, wf.components().len());
    /// ```
    pub fn add_component(&mut self, component: PeriodicFunction<P>) {
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
    /// assert_eq!(42.0, *wf.sample_rate());
    /// ```
    pub fn sample_rate(&self) -> &P {
        &self.sample_rate
    }

    /// Returns list of components this [`Waveform`] consists of.
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine, dc_bias};
    ///
    /// let wf = Waveform::<f32>::with_components(42.0, vec![sine!(1.), dc_bias!(5.)]);
    ///
    /// assert_eq!(2, wf.components().len());
    /// ```
    pub fn components(&self) -> &Vec<PeriodicFunction<P>> {
        &self.components
    }

    /// Returns an iterator over this [`Waveform`] samples.
    ///
    /// # Examples
    ///
    /// ```
    /// use wavegen::{Waveform, sine};
    ///
    /// let wf = Waveform::<f32>::with_components(42.0, vec![sine!(1.)]);
    /// let samples = wf.iter().take(100).collect::<Vec<_>>();
    /// ```
    pub fn iter(&self) -> WaveformIterator<T, P> {
        WaveformIterator::<T, P> {
            inner: self,
            time: P::zero(),
        }
    }

    #[inline]
    fn assert_sane(x: P) {
        assert!(x.is_normal());
        assert!(x.is_sign_positive());
    }
}

impl<'a, T: SampleType, P: Precision> IntoIterator for &'a Waveform<T, P> {
    type Item = T;

    type IntoIter = WaveformIterator<'a, T, P>;

    fn into_iter(self) -> Self::IntoIter {
        WaveformIterator {
            inner: self,
            time: P::zero(),
        }
    }
}

/// An iterator that allows to sample a [`Waveform`].
#[derive(Clone, Copy)]
pub struct WaveformIterator<'a, T: SampleType, P: Precision> {
    inner: &'a Waveform<T, P>,
    time: P,
}

impl<'a, T: SampleType, P: Precision> WaveformIterator<'a, T, P> {
    fn into_target_type_sanitized(sample: P) -> Option<T> {
        let result = NumCast::from(sample);

        result.or_else(|| {
            if sample > P::zero() {
                Some(T::max_value())
            } else if sample < P::zero() {
                Some(T::min_value())
            } else {
                None
            }
        })
    }

    fn increment_time(&mut self, n: usize) -> Result<(), ()> {
        let new_time = self.time + (P::from(n).ok_or(())? * (P::one() / self.inner.sample_rate));
        if new_time.is_finite() {
            self.time = new_time;
        } else {
            self.time = (P::one() / self.inner.sample_rate) - (P::max_value() - self.time);
        }

        Ok(())
    }

    fn raw_sample(&self) -> P {
        self.inner
            .components
            .iter()
            .map(|x| x.sample(self.time))
            .sum()
    }
}

impl<'a, T: SampleType, P: Precision> Iterator for WaveformIterator<'a, T, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.raw_sample();
        self.increment_time(1).ok()?;

        Self::into_target_type_sanitized(sample)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.increment_time(n).ok()?;

        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

/// Wrapper struct for a periodic function (in most cases a `f32 -> f32` or `f64 -> f64` map).
pub struct PeriodicFunction<P: Precision = f32> {
    inner: Box<dyn Fn(P) -> P + Send + Sync>,
}

impl<P: Precision + 'static> PeriodicFunction<P> {
    /// Initializes new [`PeriodicFunction`] with function defined by `f` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// let _ = wavegen::PeriodicFunction::new(Box::new(|x: f32| x.cos()));
    /// ```
    #[must_use]
    pub fn new(f: Box<dyn Fn(P) -> P + Send + Sync>) -> Self {
        Self { inner: f }
    }

    /// Helper for defining custom functions. Same as `PeriodicFunction::new` but with implicit Boxing.
    ///
    /// # Examples
    ///
    /// ```
    /// let _ = wavegen::PeriodicFunction::custom(|x: f32| x.cos());
    /// ```
    #[inline]
    pub fn custom<F: Fn(P) -> P + Send + Sync + 'static>(f: F) -> Self {
        Self::new(Box::new(f))
    }

    /// DC Bias function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.dc_bias.html
    #[inline]
    pub fn dc_bias(bias: impl Into<P>) -> Self {
        let bias = bias.into();

        Self::new(Box::new(move |_| bias))
    }

    /// Sawtooth function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.sawtooth.html
    #[inline]
    pub fn sawtooth(frequency: impl Into<P>, amplitude: impl Into<P>, phase: impl Into<P>) -> Self {
        let frequency = frequency.into();
        let amplitude = amplitude.into();
        let phase = phase.into();

        Self::new(Box::new(move |t| {
            P::two() * amplitude * (t * frequency + phase).fract() - amplitude
        }))
    }

    /// Sine function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.sine.html
    #[inline]
    pub fn sine(frequency: impl Into<P>, amplitude: impl Into<P>, phase: impl Into<P>) -> Self {
        let frequency = frequency.into();
        let amplitude = amplitude.into();
        let phase = phase.into();

        Self::new(Box::new(move |t| {
            let radians = (P::two() * P::PI() * frequency * t) + (phase * P::two() * P::PI());
            let sine = radians.sin();

            sine * amplitude
        }))
    }

    /// Square function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.square.html
    #[inline]
    pub fn square(frequency: impl Into<P>, amplitude: impl Into<P>, phase: impl Into<P>) -> Self {
        let frequency = frequency.into();
        let amplitude = amplitude.into();
        let phase = phase.into();

        Self::new(Box::new(move |t| {
            let power = (P::two() * (t - phase) * frequency).floor();

            amplitude * (P::one().neg()).powf(power)
        }))
    }

    /// Gets the inner function.
    pub fn inner(&self) -> &(impl Fn(P) -> P + Send + Sync) {
        &self.inner
    }

    /// Returns the sample value for given input.
    pub fn sample(&self, t: P) -> P {
        self.inner()(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dc_bias, sawtooth, sine, square};
    use alloc::{vec, vec::Vec};
    use float_cmp::approx_eq;
    use paste::paste;

    const EPS: f32 = 1e-3;

    #[test]
    fn square_of_high_frequency() {
        let square = PeriodicFunction::<f64>::square(u32::MAX, 1.0, 0.0);
        assert!(square.sample(1.0).is_finite());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn sine_waveform_has_default_amplitude_of_one() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1.)]);

        let samples = wf.iter().take(100).collect::<Vec<_>>();

        assert_eq!(samples[25], 1.0);
        assert_eq!(samples[75], -1.0);
    }

    #[test]
    fn sine_waveform_as_integers_has_amplitude_of_one() {
        let wf = Waveform::<i32>::with_components(100.0, vec![sine!(1.)]);

        let samples = wf.iter().take(100).collect::<Vec<_>>();

        assert_eq!(samples[25], 1);
        assert_eq!(samples[75], -1);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn sine_waveform_with_bias_has_correct_amplitude() {
        let wf = Waveform::<f32>::with_components(100.0, vec![sine!(1.), dc_bias!(5.)]);

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
                        let wf = Waveform::<f32, f64>::with_components(100.0, vec![$func]);

                        let bias = wf.iter().take(100).sum::<f32>() / 100.0;

                        assert!(approx_eq!(f32, bias, 0.0, epsilon = EPS));
                    }
                }
            )*
        };
    }

    test_no_default_bias! {
        sine: sine!(1.)
        // sawtooth: sawtooth!(1) // does not pass currently, see https://github.com/spitfire05/wavegen-rs/issues/17
        square: square!(1.)
    }

    #[test]
    #[allow(clippy::iter_skip_next)]
    fn waveform_iterator_is_infinite_single() {
        let wf = Waveform::<f64>::new(f32::MIN_POSITIVE);
        let mut iter = wf.iter().skip(usize::MAX);

        assert_eq!(Some(0f64), iter.next());
        assert_eq!(Some(0f64), iter.skip(usize::MAX).next());
    }

    #[test]
    #[allow(clippy::iter_skip_next)]
    fn waveform_iterator_is_infinite_double() {
        let wf = Waveform::<f64, f64>::new(f64::MIN_POSITIVE);
        let mut iter = wf.iter().skip(usize::MAX);

        assert_eq!(Some(0f64), iter.next());
        assert_eq!(Some(0f64), iter.skip(usize::MAX).next());
    }

    #[test]
    fn oversaturated_amplitude_clips_to_max() {
        let wf = Waveform::<u8>::with_components(100.0, vec![dc_bias!(300.)]);
        let samples = wf.iter().take(1).collect::<Vec<_>>();

        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0], u8::MAX);
    }

    #[test]
    fn undersaturated_amplitude_clips_to_min() {
        let wf = Waveform::<u8>::with_components(100.0, vec![dc_bias!(-300.)]);
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
        nan: f32::NAN
        negative: -1f32
        zero: 0.0
        infinity: f32::INFINITY
        negative_infinity: f32::NEG_INFINITY
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
        test_size_hint!(sine!(50.));
        test_size_hint!(sine!(1.), sawtooth!(2.), square!(3.), dc_bias!(4.));
    }

    #[test]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::unwrap_used)]
    fn nth_and_next_give_same_results() {
        let wf = Waveform::<i32>::with_components(44100.0, vec![sine!(3000., i16::MAX)]);
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

//! Definitions of periodic functions.

use crate::{Precision, Two};
use alloc::boxed::Box;

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
    /// let _ = wavegen::PeriodicFunction::new(Box::new(|x| x.cos()));
    /// ```
    pub fn new(f: Box<dyn Fn(P) -> P + Send + Sync>) -> Self {
        Self { inner: f }
    }

    /// Helper for defining custom functions. Same as `PeriodicFunction::new` but with implicit Boxing.
    ///
    /// # Examples
    ///
    /// ```
    /// let _ = wavegen::PeriodicFunction::new(|x| x.cos());
    /// ```
    pub fn custom<F: Fn(P) -> P + Send + Sync + 'static>(f: F) -> Self {
        Self::new(Box::new(f))
    }

    /// DC Bias function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.dc_bias.html
    pub fn dc_bias(bias: impl Into<P>) -> Self {
        let bias = bias.into();

        Self::new(Box::new(move |_| bias))
    }

    /// Sawtooth function builder. See the [`macro`] for more info.
    ///
    /// [`macro`]: ../macro.sawtooth.html
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
    #[inline(always)]
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
    #[inline(always)]
    pub fn square(frequency: impl Into<P>, amplitude: impl Into<P>, phase: impl Into<P>) -> Self {
        let frequency = frequency.into();
        let amplitude = amplitude.into();
        let phase = phase.into();

        Self::new(Box::new(move |t| {
            let power = (P::two() * (t - phase) * frequency)
                .floor()
                .to_i32()
                .unwrap_or(1); // TODO: is this safe enough?

            amplitude * (P::one().neg()).powi(power)
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

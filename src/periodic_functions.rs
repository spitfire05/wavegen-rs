//! Definitions of periodic functions.

use crate::PeriodicFunction;
use alloc::boxed::Box;
use core::f64::consts::PI;

/// Helper wrapping a custom periodic function
/// See: [Custom periodic functions]
///
/// # Examples
/// ```
/// let custom_func = wavegen::periodic_functions::custom(|t| t % 2.0);
/// ```
///
/// [Custom periodic functions]: ../index.html#custom-periodic-functions
#[inline(always)]
pub fn custom<F: Fn(f64) -> f64 + Send + Sync + 'static>(f: F) -> PeriodicFunction {
    Box::new(f)
}

/// DC Bias function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.dc_bias.html
pub fn dc_bias(bias: f64) -> PeriodicFunction {
    Box::new(move |_| bias)
}

#[cfg(feature = "std")]
fn frac(x: f64) -> f64 {
    // this is actually slower than `x - ((x as i64) as f64)` on x86_64-pc-windows-msvc target,
    // but faster than the "casting hack" when `target-cpu=native` (tested on skylake)
    x.fract()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn frac(x: f64) -> f64 {
    use libm::modf;
    let (frac, _) = modf(x);

    frac
}

/// Sawtooth function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.sawtooth.html
pub fn sawtooth(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| 2.0 * amplitude * frac(t * frequency + phase) - amplitude)
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    Box::new(move |t| {
        let radians = (2.0 * PI * frequency * t) + (phase * 2.0 * PI);
        let sine = radians.sin();

        sine * amplitude
    })
}

#[cfg(feature = "libm")]
fn _sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    use libm::sin;
    Box::new(move |t| sin((2.0 * PI * frequency * t) + (phase * 2.0 * PI)) * amplitude)
}

/// Sine function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.sine.html
#[inline(always)]
pub fn sine(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    _sine(frequency, amplitude, phase)
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn _square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    Box::new(move |t| {
        let power = (2.0 * (t - phase) * frequency).floor() as i32;

        amplitude * (-1f64).powi(power)
    })
}

#[cfg(feature = "libm")]
fn _square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    // TODO: implement duty cycle control
    use libm::{floor, pow};
    Box::new(move |t| amplitude * pow(-1.0, floor(2.0 * (t - phase) * frequency)))
}

/// Square function builder. See the [`macro`] for more info.
///
/// [`macro`]: ../macro.square.html
#[inline(always)]
pub fn square(frequency: f64, amplitude: f64, phase: f64) -> PeriodicFunction {
    _square(frequency, amplitude, phase)
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const EPS: f64 = 1e-3;

    #[test]
    fn frac_of_non_integer() {
        assert!(approx_eq!(f64, frac(1.5), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, frac(21.37), 0.37, epsilon = EPS));
        assert!(approx_eq!(f64, frac(42.69), 0.69, epsilon = EPS));
        assert!(approx_eq!(f64, frac(-5.55), -0.55, epsilon = EPS));
    }
}

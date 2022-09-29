use crate::PeriodicFunction;
use alloc::boxed::Box;

#[doc(hidden)]
pub mod bias;

#[doc(hidden)]
pub mod sawtooth;

#[doc(hidden)]
pub mod sine;

#[doc(hidden)]
pub mod square;

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

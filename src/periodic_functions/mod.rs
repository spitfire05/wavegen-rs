//! Definitions of periodic functions.

use crate::PeriodicFunction;
use alloc::boxed::Box;

pub mod bias;
pub mod sawtooth;
pub mod sine;
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

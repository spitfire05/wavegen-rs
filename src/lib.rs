//! Rust waveform generator, with no_std support.
//!
//! # Quickstart
//!
//! ```
//! use wavegen::{Waveform, sine, dc_bias, sawtooth};
//!
//! // Define a Waveform with 200Hz sampling rate and three function components,
//! // choosing f32 as the ouput type:
//! let wf = Waveform::<f32>::with_components(
//!     200.0,
//!     vec![sine!(50, 10), sawtooth!(20), dc_bias!(-5)]
//! );
//!
//! // Use Waveform as an infinite iterator:
//! let two_seconds_of_samples: Vec<f32> = wf.iter().take(400).collect();
//! ```
//!
//! Look into macros section for a complete list of defined periodic functions and their constructors.
//!
//! # Periodic function macros
//! The macros for building predefined [PeriodicFunction]s generally have a form of:
//!
//! `function!(frequency, [amplitude, [phase]])`
//!
//! (Square braces "[]" indicate optional argument).
//!
//! They come in an annotated and non-annotated form, so for example a Sine function can be expressed in both ways:
//! ```
//! use wavegen::sine;
//!
//! let sine_f = sine!(100, 20, 0.25);
//! ```
//!
//! ```
//! use wavegen::sine;
//!
//! let sine_f = sine!(frequency: 100, amplitude: 20, phase: 0.25);
//! ```
//!
//! Refer to Macros section for more info.
//!
//! # Custom periodic functions
//! Supported, of course. Just define your custom function as `Box<Fn(f64) -> f64>` and use it with [Waveform].
//!
//! ```
//! use wavegen::Waveform;
//!
//! let wf = Waveform::<f64>::with_components(100.0, vec![Box::new(|x| x % 2 as f64)]);
//! ```
//!
//! # Overflows
//!
//! As [Waveform] can be composed of multiple components, it is possible for it to overflow during samples collection.
//! If overflow occurs, the sample's value will be clamped to the largest possible representation of sample's type.
//!
//! That means `+/- Inf` for floating point types, and `MAX/MIN` for integers.
//!
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let wf = Waveform::<f64>::with_components(100.0, vec![dc_bias![f64::MAX], dc_bias![f64::MAX]]);
//! let sample = wf.iter().take(1).collect::<Vec<_>>()[0];
//!
//! assert_eq!(sample, f64::INFINITY);
//! ```
//!
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let wf = Waveform::<i32>::with_components(100.0, vec![dc_bias![f64::MAX], dc_bias![f64::MAX]]);
//! let sample = wf.iter().take(1).collect::<Vec<_>>()[0];
//!
//! assert_eq!(sample, i32::MAX);
//! ```
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
//! let wf = Waveform::<f32>::with_components(100.0, vec![sine!(80)]);
//! ```
//!
//! As it is often a case, it is you, the programmer, who's left in charge of making sure the input data makes sense.

#![no_std]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
compile_error!("at least one of \"libm\", \"std\" features has to be enabled");

extern crate alloc;

#[doc(hidden)]
pub mod periodic_functions;

mod waveform;
mod assert;

use alloc::boxed::Box;

pub use waveform::SampleType;
pub use waveform::Waveform;

/// Type alias defining a periodic function (f64 -> f64 map)
pub type PeriodicFunction = Box<dyn Fn(f64) -> f64 + Send + Sync>;

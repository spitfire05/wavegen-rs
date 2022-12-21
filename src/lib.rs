//! Rust waveform generator, with [no_std support](https://github.com/spitfire05/wavegen-rs#how-to-use-it).
//!
//! # Quickstart
//!
//! ```
//! use wavegen::{wf, sine, dc_bias, sawtooth};
//!
//! // Define a Waveform with 200Hz sampling rate and three function components,
//! // choosing f32 as the output type:
//! let waveform = wf!(f32, 200, sine!(50, 10), sawtooth!(20), dc_bias!(-5));
//!
//! // Use Waveform as an infinite iterator:
//! let two_seconds_of_samples: Vec<f32> = waveform.iter().take(400).collect();
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
//! use wavegen::{wf, periodic_functions::custom};
//!
//! let waveform = wf!(f64, 100.0, custom(|x| x % 2.0));
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
//! # Iterator infinity
//!
//! `WaveformIterator` is a *mostly* infinite iterator, with one exception:
//!
//! The `WaveformIterator::next()` method can return `None` in some rare cases if it is not able to convert the inner sample type `f64` into the target sample type.
//!
//! `f64::NAN` cannot be represented as `i32`:
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let mut wf = Waveform::<i32>::new(100.0);
//! wf.add_component(dc_bias!(f64::NAN));
//!
//! assert_eq!(None, wf.iter().next())
//! ```
//!
//! This however is fine, as `f64::NAN` can be represented as `f32::NAN`:
//! ```
//! use wavegen::{Waveform, dc_bias};
//!
//! let mut wf = Waveform::<f32>::new(100.0);
//! wf.add_component(dc_bias!(f64::NAN));
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

pub mod periodic_functions;

mod macros;
mod sampling_rate;
mod waveform;

use alloc::boxed::Box;

pub use sampling_rate::SamplingRate;
pub use sampling_rate::SamplingRateValueError;
pub use waveform::SampleType;
pub use waveform::Waveform;

/// Type alias defining a periodic function (f64 -> f64 map)
pub type PeriodicFunction = Box<dyn Fn(f64) -> f64 + Send + Sync>;

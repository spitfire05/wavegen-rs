//! 'wavy' is a pure rust waveform generator.

#![no_std]

extern crate alloc;

mod periodic_functions;
mod waveform;

use alloc::boxed::Box;
pub use periodic_functions::sine::Sine;
pub use periodic_functions::bias::dc_bias;
pub use periodic_functions::sine::sinef;

pub use waveform::Waveform;

/// Type alias defining a periodic function (f32 -> f32 map)
pub type PeriodicFunction = Box<dyn Fn(f32) -> f32>;

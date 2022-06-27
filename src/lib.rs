//! 'wavy' is a pure rust waveform generator.

#![no_std]

extern crate alloc;

#[doc(hidden)]
pub mod periodic_functions;

mod waveform;

use alloc::boxed::Box;

pub use waveform::Waveform;

/// Type alias defining a periodic function (f64 -> f64 map)
pub type PeriodicFunction = Box<dyn Fn(f64) -> f64>;

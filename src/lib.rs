#![no_std]

extern crate alloc;

// Use wee_alloc as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod periodic_functions;
mod waveform;

use alloc::boxed::Box;
pub use periodic_functions::sine::Sine;

pub use waveform::Waveform;

pub type PeriodicFunction = Box<dyn Fn(f32) -> f32>;

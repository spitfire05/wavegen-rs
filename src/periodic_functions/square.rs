use alloc::boxed::Box;
use libm::{pow, floor};

use crate::PeriodicFunction;

pub fn _square(frequency: f64, amplitude: f64, phase: f64, duty_cycle: f64) -> PeriodicFunction {
    Box::new(move |t| amplitude * pow(-1.0, floor((2.0 * (t - phase)) / (1.0 / frequency))))
}

#[macro_export]
macro_rules! square {
    ($frequency:expr, $amplitude:expr, $phase:expr, $duty_cycle:expr) => {
        $crate::periodic_functions::square::_square($frequency as f64, $amplitude as f64, $phase as f64, $duty_cycle as f64)
    };
}
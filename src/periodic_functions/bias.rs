use alloc::boxed::Box;

use crate::PeriodicFunction;

#[derive(Debug, Clone, Copy)]
pub struct Bias {
    bias: f64,
}

impl Bias {
    pub fn new(bias: f64) -> Box<Self> {
        Box::new(Bias { bias })
    }
}

impl PeriodicFunction for Bias {
    fn sample(&self, _: f64) -> f64 {
        self.bias
    }
}

/// Builder macro for DC Bias [PeriodicFunction].
///
/// Takes just one argument - the bias value.
///
/// # Examples
///
/// Defines bias of amplitude +10
/// ```
/// use wavegen::{dc_bias, PeriodicFunction};
///
/// let bias = dc_bias!(10);
///
/// assert!((0..100000).all(|x| bias.sample(x as f64) == 10.0))
/// ```
#[macro_export]
macro_rules! dc_bias {
    ($bias:expr) => {
        $crate::periodic_functions::bias::Bias::new($bias as f64)
    };
}

#[cfg(test)]
mod tests {
    use crate::PeriodicFunction;

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y);
        for x in (0..10000000).map(|x| x as f64) {
            assert_eq!(dc.sample(x), y);
        }
    }
}

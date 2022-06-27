use alloc::boxed::Box;

use crate::PeriodicFunction;

pub fn dc_bias_builder(bias: f32) -> PeriodicFunction {
    Box::new(move |_| bias)
}

/// Builder macro for DC Bias [PeriodicFunction].
/// 
/// Takes just one argument - the bias value.
/// 
/// # Examples
/// 
/// Defines bias of amplitude +10
/// ```
/// use wavy::dc_bias;
/// 
/// let bias = dc_bias!(10);
/// ```
#[macro_export]
macro_rules! dc_bias {
    ($bias:expr) => {
        $crate::periodic_functions::bias::dc_bias_builder($bias as f32)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y);
        for x in (0..10000000).map(|x| x as f32) {
            assert_eq!(dc(x), y);
        }
    }
}
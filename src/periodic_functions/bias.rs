use alloc::boxed::Box;

use crate::PeriodicFunction;

/// Builder for a DC Bias [PeriodicFunction]
pub fn dc_bias(bias: f32) -> PeriodicFunction {
    Box::new(move |_| bias)
}

#[cfg(test)]
mod tests {
    use super::dc_bias;

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias(y);
        for x in (0..10000000).map(|x| x as f32) {
            assert_eq!(dc(x), y);
        }
    }
}
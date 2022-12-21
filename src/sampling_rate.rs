use core::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
/// Defines sampling rate value - aka non-zero, positive, finite `f64`.
pub struct SamplingRate(pub(crate) f64);

impl SamplingRate {
    /// Initializes new `SamplingRate` value.
    ///
    /// # Examples
    ///
    /// ```
    /// let s = wavegen::SamplingRate::new(44100.0);
    /// assert!(s.is_ok());
    /// ````
    pub fn new(value: impl Into<f64>) -> Result<Self, SamplingRateValueError> {
        let value = value.into();
        if !Self::is_sane(value) {
            return Err(SamplingRateValueError(value));
        }

        Ok(SamplingRate(value))
    }

    fn is_sane(x: f64) -> bool {
        x.is_normal() && x.is_sign_positive()
    }
}

#[derive(Error, Debug, Clone, Copy)]
#[error("Invalid SamplingRate value: `{0}`. SamplingRate has to be positive, non-zero and finite.")]
/// Error raised when cosntructing [`SamplingRate`] of invalid value.
pub struct SamplingRateValueError(f64);

macro_rules! impl_tryfrom {
    ($($target:ty),+) => {
        $(
            impl TryFrom<$target> for SamplingRate {
                type Error = SamplingRateValueError;

                fn try_from(value: $target) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        )+
    };
}

impl_tryfrom! {
    f64, f32, u32, i32, u16, i16, u8, i8
}

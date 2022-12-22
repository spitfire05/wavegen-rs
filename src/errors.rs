use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
#[error("Invalid SamplingRate value: `{0}`. SamplingRate has to be positive, non-zero and finite.")]
/// Error raised when cosntructing [`SamplingRate`] of invalid value.
pub struct InvalidSampleRate(pub(crate) f64);

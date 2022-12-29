use core::fmt::Display;

#[derive(Debug, Clone, Copy)]
/// Error raised when cosntructing [`Waveform`] with sample rate of invalid value.
pub struct InvalidSampleRate(pub(crate) f64);

impl Display for InvalidSampleRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid SamplingRate value: `{}`. SamplingRate has to be positive, non-zero and finite.", self.0)
    }
}

impl std::error::Error for InvalidSampleRate {}

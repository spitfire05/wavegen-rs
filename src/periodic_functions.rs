use alloc::rc::Rc;

#[cfg(feature = "std")]
fn frac(x: f64) -> f64 {
    // this is actually slower than `x - ((x as i64) as f64)` on x86_64-pc-windows-msvc target,
    // but faster than the "casting hack" when `target-cpu=native` (tested on skylake)
    x.fract()
}

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn frac(x: f64) -> f64 {
    use libm::modf;
    let (frac, _) = modf(x);

    frac
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn square(pfd: &PeriodicFunctionData, t: f64) -> f64 {
    let power = (2.0 * (t - pfd.phase) * pfd.frequency).floor() as i32;

    pfd.amplitude * (-1f64).powi(power)
}

#[cfg(feature = "libm")]
fn square(pfd: &PeriodicFunctionData, t: f64) -> f64 {
    // TODO: implement duty cycle control
    use libm::{floor, pow};
    pfd.amplitude * pow(-1.0, floor(2.0 * (t - pfd.phase) * pfd.frequency))
}

fn sawtooth(pfd: &PeriodicFunctionData, t: f64) -> f64 {
    2.0 * pfd.amplitude * frac(t * pfd.frequency + pfd.phase) - pfd.amplitude
}

#[cfg(all(not(feature = "libm"), feature = "std"))]
fn sine(pfd: &PeriodicFunctionData, t: f64) -> f64 {
    use core::f64::consts::PI;

    let radians = (2.0 * PI * pfd.frequency * t) + (pfd.phase * 2.0 * PI);
    let sine = radians.sin();

    sine * pfd.amplitude
}

#[cfg(feature = "libm")]
fn sine(pfd: &PeriodicFunctionData, t: f64) -> f64 {
    use libm::sin;
    sin((2.0 * PI * pfdfrequency * t) + (pfdphase * 2.0 * PI)) * pfd.amplitude
}

/// Data struct for [PeriodicFunction]. You probably don't need to use this directly.
#[derive(Debug, Clone, Copy)]
pub struct PeriodicFunctionData {
    frequency: f64,
    amplitude: f64,
    phase: f64,
}

impl PeriodicFunctionData {
    /// Creates new instance of [PeriodicFunctionData].
    /// You probably don't need to use this, see the macros section instead.
    pub fn new(frequency: f64, amplitude: f64, phase: f64) -> Self {
        Self {
            frequency,
            amplitude,
            phase,
        }
    }
}

/// Defines a periodic function to use with [crate::Waveform].
#[derive(Clone)]
pub enum PeriodicFunction {
    /// Sine wave
    Sine(PeriodicFunctionData),

    /// Square wave
    Square(PeriodicFunctionData),

    /// Sawtooth wave
    Sawtooth(PeriodicFunctionData),

    /// DC bias
    Bias(f64),

    /// Custom function
    Custom(Rc<dyn Fn(f64) -> f64>),
}

impl PeriodicFunction {
    /// Returns the sample value at point `t`
    pub fn sample(&self, t: f64) -> f64 {
        match self {
            PeriodicFunction::Sine(pfd) => sine(pfd, t),
            PeriodicFunction::Square(pfd) => square(pfd, t),
            PeriodicFunction::Sawtooth(pfd) => sawtooth(pfd, t),
            PeriodicFunction::Bias(b) => *b,
            PeriodicFunction::Custom(f) => f(t),
        }
    }

    /// Returns a custom periodic function, defined by `f`
    ///
    /// # Examples
    /// ```
    /// use wavegen::PeriodicFunction;
    ///
    /// let f = PeriodicFunction::custom(|t| t % 2.0);
    /// ```
    pub fn custom<T: Fn(f64) -> f64 + 'static>(f: T) -> Self {
        Self::Custom(Rc::new(f))
    }
}

impl std::fmt::Debug for PeriodicFunction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sine(arg0) => f.debug_tuple("Sine").field(arg0).finish(),
            Self::Square(arg0) => f.debug_tuple("Square").field(arg0).finish(),
            Self::Sawtooth(arg0) => f.debug_tuple("Sawtooth").field(arg0).finish(),
            Self::Bias(arg0) => f.debug_tuple("Bias").field(arg0).finish(),
            Self::Custom(_) => f.debug_tuple("Custom").finish(),
        }
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
        $crate::PeriodicFunction::Bias($bias as f64)
    };
}

/// Builder macro for Sawtooth [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
#[macro_export]
macro_rules! sawtooth {
    ($frequency:expr) => {
        sawtooth!($frequency, 1.0, 0.0)
    };
    (frequency: $frequency:expr) => {
        sawtooth!($frequency)
    };
    ($frequency:expr, $amplitude:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sawtooth!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sawtooth!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::Sawtooth($crate::PeriodicFunctionData::new(
            $frequency as f64,
            $amplitude as f64,
            $phase as f64,
        ))
    };
}

/// Builder macro for Sine [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
///
/// # Examples
///
/// 50 Hz sine of amplitude 1 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(50);
/// ```
///
/// 50 Hz sine of amplitude 20 and no phase shift
/// ```
/// use wavegen::sine;
///
/// let sine = sine!(frequency: 50, amplitude: 20);
/// ```
///
/// 50 Hz sine of amplitude 20 and phase shift of half a turn
/// ```
/// use core::f64::consts::PI;
/// use wavegen::sine;
///
/// let sine = sine!(50, 20, 0.5);
/// ```
#[macro_export]
macro_rules! sine {
    (frequency: $frequency:expr) => {
        sine!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        sine!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        sine!($frequency, $amplitude, $phase)
    };
    ($frequency:expr) => {
        sine!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        sine!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::Sine($crate::PeriodicFunctionData::new(
            $frequency as f64,
            $amplitude as f64,
            $phase as f64,
        ))
    };
}

/// Builder macro for Square [PeriodicFunction].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
#[macro_export]
macro_rules! square {
    (frequency: $frequency:expr) => {
        square!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        square!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr) => {
        square!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::Square($crate::PeriodicFunctionData::new(
            $frequency as f64,
            $amplitude as f64,
            $phase as f64,
        ))
    };
}

#[cfg(test)]
mod tests {
    use super::frac;
    use float_cmp::approx_eq;

    const EPS: f64 = 1e-3;

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y);
        for x in (0..10000000).map(|x| x as f64) {
            assert_eq!(dc.sample(x), y);
        }
    }

    #[test]
    fn frac_of_non_integer() {
        assert!(approx_eq!(f64, frac(1.5), 0.5, epsilon = EPS));
        assert!(approx_eq!(f64, frac(21.37), 0.37, epsilon = EPS));
        assert!(approx_eq!(f64, frac(42.69), 0.69, epsilon = EPS));
        assert!(approx_eq!(f64, frac(-5.55), -0.55, epsilon = EPS));
    }

    #[test]
    fn default_sawtooth_has_amplitude_of_one() {
        let f = sawtooth!(2.0);

        assert!(approx_eq!(f64, f.sample(0.49999), 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, f.sample(0.5), -1.0, epsilon = EPS));
    }

    #[test]
    fn default_sine_has_amplitude_of_one_and_no_phase_shift() {
        let sine = sine!(1);

        let max = sine.sample(0.25);
        let min = sine.sample(0.75);
        let zero = sine.sample(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    #[test]
    fn phase_affects_min_max_amplitude_position() {
        let sine = sine!(1, 1, 0.5);

        let max = sine.sample(0.75);
        let min = sine.sample(0.25);
        let zero = sine.sample(0.5);

        assert!(approx_eq!(f64, max, 1.0, epsilon = EPS));
        assert!(approx_eq!(f64, min, -1.0, epsilon = EPS));
        assert!(approx_eq!(f64, zero, 0.0, epsilon = EPS));
    }

    #[test]
    fn default_square_has_amplitude_of_one() {
        let square = square!(1);

        for x in [0.0, 0.1, 0.2, 0.3, 0.4] {
            assert!(approx_eq!(f64, square.sample(x), 1.0, epsilon = EPS))
        }

        for x in [0.5, 0.6, 0.7, 0.8, 0.9] {
            assert!(approx_eq!(f64, square.sample(x), -1.0, epsilon = EPS))
        }
    }
}

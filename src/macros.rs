/// Helper macro to construct [`Waveform`] instance.
///
/// # Panics
///
/// This macro will cause panic if sampling rate is not a finite, positive, non-zero number.
///
/// # Examples
///
/// ```
/// use wavegen::{wf, sine, square};
///
/// let empty_waveform = wf!(f32, 16000);
/// assert_eq!(0, empty_waveform.get_components_len());
///
/// let sine_waveform = wf!(f64, 44100, sine!(50));
/// assert_eq!(1, sine_waveform.get_components_len());
///
/// let some_other_waveform = wf!(i64, 22000, sine!(100), square!(200));
/// assert_eq!(2, some_other_waveform.get_components_len());
/// ```
///
/// [`Waveform`]: struct.waveform.html
#[macro_export]
macro_rules! wf {
    ($sample_type:ty, $sample_rate:expr) => {
        $crate::wf!($sample_type, $sample_rate, f32)
    };
    ($sample_type:ty, $sample_rate:expr, $precision:ty) => {
        $crate::Waveform::<$sample_type>::new($sample_rate)
    };
    ($sample_type:ty, $sample_rate:expr, $($comp:expr),+) => {
        $crate::wf!($sample_type, $sample_rate, f32, $($comp,)+)
    };
    ($sample_type:ty, $sample_rate:expr, $precision:ty, $($comp:expr),+ $(,)?) => {
        {
            extern crate alloc;
            let __wf = $crate::Waveform::<$sample_type>::with_components($sample_rate, alloc::vec![$($comp,)+]);

            __wf
        }
    };
}

/// Builder macro for DC Bias [`PeriodicFunction`].
///
/// Takes just one argument - the bias value.
///
/// # Examples
///
/// Defines bias of amplitude +10
/// ```
/// let bias = wavegen::dc_bias!(10);
///
/// assert!((0..100000).all(|x| bias(x.into()) == 10.0))
/// ```
///
/// [`PeriodicFunction`]: type.periodicfunction.html
#[macro_export]
macro_rules! dc_bias {
    ($bias:expr) => {
        $crate::dc_bias!($bias, f32)
    };
    ($bias:expr, $t:ty) => {
        $crate::PeriodicFunction::<$t>::dc_bias($bias)
    };
}

/// Builder macro for Sawtooth [`PeriodicFunction`].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
///
/// [`PeriodicFunction`]: type.periodicfunction.html
#[macro_export]
macro_rules! sawtooth {
    ($frequency:expr) => {
        $crate::sawtooth!($frequency, 1.0, 0.0)
    };
    (frequency: $frequency:expr) => {
        $crate::sawtooth!($frequency)
    };
    ($frequency:expr, $amplitude:expr) => {
        $crate::sawtooth!($frequency, $amplitude, 0.0)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        $crate::sawtooth!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        $crate::sawtooth!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::sawtooth($frequency, $amplitude, $phase)
    };
}

/// Builder macro for Sine [`PeriodicFunction`].
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
/// let sine = wavegen::sine!(50);
/// ```
///
/// 50 Hz sine of amplitude 20 and no phase shift
/// ```
/// let sine = wavegen::sine!(frequency: 50, amplitude: 20);
/// ```
///
/// 50 Hz sine of amplitude 20 and phase shift of half a turn
/// ```
/// let sine = wavegen::sine!(50, 20, 0.5);
/// ```
///
/// [`PeriodicFunction`]: type.periodicfunction.html
#[macro_export]
macro_rules! sine {
    (frequency: $frequency:expr) => {
        $crate::sine!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        $crate::sine!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        $crate::sine!($frequency, $amplitude, $phase)
    };
    ($frequency:expr) => {
        $crate::sine!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        $crate::sine!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::sine($frequency, $amplitude, $phase)
    };
}

/// Builder macro for Square [`PeriodicFunction`].
///
/// Takes up to 3 arguments - frequency {amplitude, {phase}}
///
/// | argument | unit | notes |
/// | -------- | ---- | ----- |
/// | frequency | Hz | Frequecy of the periodic function. Also: 1 / period |
/// | amplitude | *arbitrary* | The amplitude of the function in 0-peak notation. |
/// | phase | *periods* | The phase shift of the function. Value of 1 means full shift around.
///
/// [`PeriodicFunction`]: type.periodicfunction.html
#[macro_export]
macro_rules! square {
    (frequency: $frequency:expr) => {
        $crate::square!($frequency)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr) => {
        $crate::square!($frequency, $amplitude)
    };
    (frequency: $frequency:expr, amplitude: $amplitude:expr, phase: $phase:expr) => {
        $crate::square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr) => {
        $crate::square!($frequency, 1.0, 0.0)
    };
    ($frequency:expr, $amplitude:expr) => {
        $crate::square!($frequency, $amplitude, 0.0)
    };
    ($frequency:expr, $amplitude:expr, $phase:expr) => {
        $crate::PeriodicFunction::square($frequency, $amplitude, $phase)
    };
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    const EPS: f64 = 1e-3;

    #[test]
    fn empty_waveform_has_zero_components() {
        let wf = wf!(f64, 44100);
        assert_eq!(0, wf.get_components_len());
    }

    #[test]
    fn wavefrom_with_one_component() {
        let wf = wf!(f64, 44100, sine!(500));
        assert_eq!(1, wf.get_components_len());
    }
    #[test]
    fn wavefrom_with_three_components() {
        let wf = wf!(f64, 44100, sine!(500), square!(1000), sawtooth!(1500));
        assert_eq!(3, wf.get_components_len());
    }

    #[test]
    fn dc_bias_is_const_for_any_input() {
        let y = 42.0;
        let dc = dc_bias!(y, f64);
        for x in (0..10000000).map(|x| x.into()) {
            assert_eq!(dc.sample(x), y);
        }
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
    fn sine_phase_affects_min_max_amplitude_position() {
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

# wavegen

## 0.4.0

- `Waveform` has a parametrized precision now. By default `f32`, can be set to `f64` if needed.
- Renamed `Waveform::get_sample_rate` to `Waveform::sample_rate`.
- Removed `Waveform::get_components_len`.
- Added `Waveform::components` getter.

## 0.3.0

- `wf` helper macro added.
- `WaveformIterator` no longer will panic on failed conversion to output type.
- Switch to using `Into` instead of directly casting numeric types in numerous places.

# 0.2.2

- Implemented `size_hint`.

# 0.2.1

- Added `Sync` and `Send` constraints on `PeriodicFunction`.
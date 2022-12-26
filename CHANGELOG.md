# wavegen

## 0.4.0

- `Waveform` constructors now return `Result<Waveform, InvalidSampleRate>` to allow for handling the errors instead of outright panicking.

## 0.3.0

- `wf` helper macro added.
- `WaveformIterator` no longer will panic on failed conversion to output type.
- Switch to using `Into` instead of directly casting numeric types in numerous places.

## 0.2.2

- Implemented `size_hint`.

## 0.2.1

- Added `Sync` and `Send` constraints on `PeriodicFunction`.
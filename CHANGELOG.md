# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [0.4.1]

### Fixed

- Square function giving wrong samples for very high frequencies.


## [0.4.0]

### Added

- `Waveform::components` getter.

### Changed

- `Waveform` has a parametrized precision now. By default `f32`, can be set to `f64` if needed.
- Renamed `Waveform::get_sample_rate` to `Waveform::sample_rate`.

### Removed

- `Waveform::get_components_len`.



## [0.3.0]

### Added

- `wf!` macro.

### Changed

- `WaveformIterator` no longer will panic on failed conversion to output type.
- Switch to using `Into` instead of directly casting numeric types in numerous places.

## [0.2.2]

### Added

- `size_hint` implementation on `WaveformIterator`.

## [0.2.1]

### Added

- `Sync` and `Send` constraints on `PeriodicFunction`.


## [0.2.0]

### Changed

- Make `no_std` optional. Add separate math backend for std.
- Macro annotations to `frequency:` style.

## [0.1.0]

- initial release

<!-- Links -->
[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html

<!-- Versions -->
[unreleased]: https://github.com/spitfire05/wavegen-rs/compare/0.4.1...HEAD
[0.4.1]: https://github.com/spitfire05/wavegen-rs/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/spitfire05/wavegen-rs/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/spitfire05/wavegen-rs/compare/0.2.2...0.3.0
[0.2.2]: https://github.com/spitfire05/wavegen-rs/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/spitfire05/wavegen-rs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/spitfire05/wavegen-rs/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/spitfire05/wavegen-rs/releases/tag/0.1.0
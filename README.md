# wavegen

[![Crates.io](https://img.shields.io/crates/v/wavegen)](https://crates.io/crates/wavegen)
[![docs](https://img.shields.io/docsrs/wavegen)](https://docs.rs/wavegen)
[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/46467)

`wavegen` is a wavefrom generator made with 🦀

## How to use it?

1) Add `wavegen` to your project:

```toml
[dependencies]
wavegen = "0.2"
```
Or, to use the *no_std* version:

```toml
[dependencies]
wavegen = { version = "0.2", default-features = false, features = ["libm"] }
```

2) Define a waveform with sampling frequency and function components:

```rust
let wf = Waveform::<f64>::with_components(200.0, vec![
        sine!(frequency: 100, amplitude: 10),
        dc_bias!(20)
    ]);
```

3) Turn it into an iterator and sample:

```rust
let some_samples: Vec<f64> = wf.iter().take(200).collect();
```

Refer to [documentation](https://docs.rs/wavegen) for more exhaustive usage examples.

## Show me some examples!

### Interactive demo

Check out the demo at https://wavegen-demo.netlify.app

### Plot charts

* Simple sine

![Sine plot](img/sine.png)

* Two superposed phase-shifted sines

![Superposed sines plot](img/sine_double.png)

* "Real life" example: 300Hz sine signal with 50Hz interference noise

![300_50_hz_sines](img/sines_300_50_hz.png)

* Sawtooth

![Sawtooth plot](img/sawtooth.png)

* Superposition of sine + sawtooth

![Sine and sawtooth superposed](img/sawtooth_sinesised.png)

* Square wave

![Square wave](img/square.png)

* Superposition of Sine, Square and Sawtooth with different frequencies

![Something funky](img/funky.png)

All above examples are generated with simple program found in `examples/plot.rs`. Run `cargo run --example plot` to generate them yourself.

## MSRV

The *Minimum Supported Rust Version* is `1.56.1`.

## Similar crates
* [Waver](https://github.com/amrali/waver/) which was the inspiration for this crate

## Breaking changes

### 0.2

0.2 intorduces a braking change in how macros are annotated, changing the annotation form from `frequency = n` to `frequency: n`

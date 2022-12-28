use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wavegen::{dc_bias, sawtooth, sine, square, Waveform};

fn sample_waveform_single(n: usize) -> Vec<f64> {
    let wf = Waveform::<f64, f32>::with_components(
        44100.0,
        vec![
            sine!(2048.),
            sawtooth!(1024.),
            square!(512.),
            dc_bias!(0.1, f32),
        ],
    );

    wf.iter().take(n).collect::<Vec<f64>>()
}

fn sample_waveform_double(n: usize) -> Vec<f64> {
    let wf = Waveform::<f64, f64>::with_components(
        44100.0,
        vec![
            sine!(2048.),
            sawtooth!(1024.),
            square!(512.),
            dc_bias!(0.1, f64),
        ],
    );

    wf.iter().take(n).collect::<Vec<f64>>()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("44.1 kHz 25000 samples single precision", |b| {
        b.iter(|| sample_waveform_single(black_box(25000)))
    });

    c.bench_function("44.1 kHz 25000 samples double precision", |b| {
        b.iter(|| sample_waveform_double(black_box(25000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

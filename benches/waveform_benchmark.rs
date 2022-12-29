use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wavegen::{dc_bias, sawtooth, sine, square, Waveform};

fn sample_waveform(n: usize) -> Vec<f64> {
    let wf = Waveform::with_components(
        44100.0,
        vec![sine!(2048), sawtooth!(1024), square!(512), dc_bias!(0.1)],
    )
    .unwrap();

    wf.iter().take(n).collect::<Vec<f64>>()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("44.1 kHz 25000 samples", |b| {
        b.iter(|| sample_waveform(black_box(25000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

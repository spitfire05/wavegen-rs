use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wavegen::{dc_bias, sawtooth, sine, square, Precision, SampleType, Waveform};

fn sample_waveform<T: SampleType, P: Precision>(n: usize) -> Vec<T> {
    let wf = Waveform::<T, P>::with_components(
        P::from(44100.0).unwrap(),
        vec![
            sine!(
                P::from(2048.).unwrap(),
                P::from(1).unwrap(),
                P::from(0).unwrap()
            ),
            sawtooth!(
                P::from(1024.).unwrap(),
                P::from(1).unwrap(),
                P::from(0).unwrap()
            ),
            square!(
                P::from(512.).unwrap(),
                P::from(1).unwrap(),
                P::from(0).unwrap()
            ),
            dc_bias!(P::from(0.1).unwrap()),
        ],
    );

    wf.iter().take(n).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("44.1 kHz 25000 samples");
    group.bench_function("f32 sample @ f32 precision", |b| {
        b.iter(|| sample_waveform::<f32, f32>(black_box(25000)));
    });

    group.bench_function("f32 sample @ f64 precision", |b| {
        b.iter(|| sample_waveform::<f32, f64>(black_box(25000)));
    });
    group.bench_function("f64 sample @ f32 precision", |b| {
        b.iter(|| sample_waveform::<f64, f32>(black_box(25000)));
    });
    group.bench_function("f64 sample @ f64 precision", |b| {
        b.iter(|| sample_waveform::<f64, f64>(black_box(25000)));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

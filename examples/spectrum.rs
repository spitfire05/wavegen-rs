use plotters::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use std::path::Path;
use wavegen::{sine, square, wf};

const F_MAX: f32 = 350.0;
const SAMPLE_RATE: f32 = F_MAX * 2.56;
const N_SAMPLES: usize = 2048;
const SPECTRUM_RESOLUTION: f32 = SAMPLE_RATE / N_SAMPLES as f32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the waveform to perform FFT on
    let wf = wf!(
        f32,
        SAMPLE_RATE,
        sine!(300., 10.),
        sine!(50., 2.),
        square!(100., 5.)
    );
    let samples = wf.into_iter().take(N_SAMPLES);

    // Perform FFT
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(N_SAMPLES);
    let mut buffer: Vec<_> = samples.into_iter().map(|s| Complex::new(s, 0.)).collect();
    fft.process(&mut buffer);

    draw(
        "spectrum.png",
        "Spectrum",
        buffer
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as f32 * SPECTRUM_RESOLUTION, c.norm() / N_SAMPLES as f32))
            .take_while(|(f, _)| *f < F_MAX),
    )?;

    Ok(())
}

fn draw<I: IntoIterator<Item = (f32, f32)>, P: AsRef<Path>>(
    path: P,
    label: &str,
    iter: I,
) -> Result<(), Box<dyn std::error::Error>> {
    let img_path = Path::new("img").join(path);
    let root = BitMapBackend::new(&img_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        // .caption(label, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..F_MAX, -0f32..10f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(iter, RED))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}

use std::path::Path;

use libm::sqrtf;
use plotters::prelude::*;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use wavegen::{sine, Waveform};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 2560.0;
    let n = 2048;

    let samples = Waveform::<f32>::with_components(sample_rate, vec![sine!(300, 10), sine!(50, 2)])
        .into_iter()
        .take(n)
        .collect::<Vec<f32>>();

    let spectrum = samples_fft_to_spectrum(
            &samples,
            sample_rate as u32,
            FrequencyLimit::All,
            None,
        ).unwrap();

    draw("spectrum.png", "Spectrum", spectrum.data().iter().map(|(f, a)| (f.val(), a.val() * (1.0 / sqrtf(n as f32)))))?;

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
        .build_cartesian_2d(-0f32..1024f32, -0f32..350f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(iter, &RED))?
        .label(label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

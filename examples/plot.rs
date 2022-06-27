use std::path::Path;

use plotters::prelude::*;
use wavy::{sawtooth, sine, Waveform};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 150.0;

    draw(
        "sine.png",
        "Sine",
        Waveform::<f32>::with_components(sample_rate, vec![sine!(1.0)])
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as f32 / sample_rate as f32, x))
            .take(sample_rate as usize),
    )?;

    draw(
        "sine_double.png",
        "Sines",
        Waveform::<f32>::with_components(sample_rate, vec![sine!(1.0), sine!(1.0, 1.0, 0.25)])
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as f32 / sample_rate as f32, x))
            .take(sample_rate as usize),
    )?;

    draw(
        "sawtooth.png",
        "Sawtooth",
        Waveform::<f32>::with_components(sample_rate, vec![sawtooth!(2, 1, 0.0)])
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as f32 / sample_rate as f32, x))
            .take(sample_rate as usize),
    )?;

    draw(
        "sawtooth_sinesised.png",
        "Sawtooth with sine",
        Waveform::<f32>::with_components(sample_rate, vec![sawtooth!(2, 1, 0.0), sine!(50, 0.1)])
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as f32 / sample_rate as f32, x))
            .take(sample_rate as usize),
    )?;

    Ok(())
}

fn draw<I: IntoIterator<Item = (f32, f32)>, P: AsRef<Path>>(
    path: P,
    label: &str,
    iter: I,
) -> Result<(), Box<dyn std::error::Error>> {
    let img_path = Path::new("img").join(path);
    let root = BitMapBackend::new(&img_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        // .caption(label, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-0f32..1f32, -2f32..2f32)?;

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

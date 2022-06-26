use std::{f32::consts::PI, path::Path};

use plotters::prelude::*;
use wavy::{Sine, Waveform};

#[macro_export]
macro_rules! foo {
    ( $ ($label:expr, $iter:expr),* ) => {
        $(
            draw("foo.png", label, $x)?
        )*
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 150.0;

    draw(
        "sine.png",
        "Sine",
        Waveform::<f32>::with_components(sample_rate, vec![Sine::with_frequency(1.0).build()])
            .into_iter()
            .enumerate()
            .map(|(i, x)| (i as f32 / sample_rate, x))
            .take(sample_rate as usize),
    )?;

    draw(
        "sine_double.png",
        "Sines",
        Waveform::<f32>::with_components(
            sample_rate,
            vec![
                Sine::with_frequency(1.0).build(),
                Sine::new(1.0, 1.0, PI / 2.0, 0.0).build(),
            ],
        )
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i as f32 / sample_rate, x))
        .take(sample_rate as usize),
    )?;

    Ok(())
}

fn draw<I: IntoIterator<Item = (f32, f32)>, P: AsRef<Path>>(
    path: P,
    label: &str,
    iter: I,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(label, ("sans-serif", 50).into_font())
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

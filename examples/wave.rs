/* Use Wavegen + Hound to generate a WAVE audio file */
use wavegen::{dc_bias, sine, Waveform};

const SAMPLE_RATE: u16 = 44100; // sample rate
const FILENAME: &str = "sine.wav"; // output file name
const WAVE_TIME_S: f32 = 1.0; // audio length in seconds

fn main() {
    // Define waveform
    // 500 Hz sine spanned from 0 to i16::MAX
    let wf = Waveform::<i16>::with_components(
        SAMPLE_RATE as f64,
        vec![sine!(500, i16::MAX / 2), dc_bias!(i16::MAX / 2)],
    );

    // WAVE file specification
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Write waveform to file
    let mut writer = hound::WavWriter::create(FILENAME, spec).unwrap();
    for s in wf.iter().take((SAMPLE_RATE as f32 * WAVE_TIME_S) as usize) {
        writer.write_sample(s).unwrap();
    }
    println!("{}s of audio written to {}", WAVE_TIME_S, FILENAME);
}

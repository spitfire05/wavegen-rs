/* Use Wavegen + Hound to generate a WAVE audio file */
use wavegen::{sine, wf};

const SAMPLE_RATE: u16 = 44100; // sample rate
const FILENAME: &str = "sine.wav"; // output file name
const WAVE_TIME_S: f32 = 1.0; // audio length in seconds

fn main() {
    // Define waveform
    // 500 Hz sine spanned from i16::MIN to i16::MAX
    let wf = wf!(i16, SAMPLE_RATE, sine!(500., i16::MAX));

    // WAVE file specification

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: u32::from(SAMPLE_RATE),
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    // Write waveform to file
    let mut writer = hound::WavWriter::create(FILENAME, spec).unwrap();
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    for s in wf
        .iter()
        .take((f32::from(SAMPLE_RATE) * WAVE_TIME_S) as usize)
    {
        writer.write_sample(s).unwrap();
    }
    println!("{WAVE_TIME_S}s of audio written to {FILENAME}");
}

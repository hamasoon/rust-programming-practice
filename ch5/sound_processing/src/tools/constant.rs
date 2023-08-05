use hound;

pub const SAMPLE_RATE: f32 = 44100.0;
pub const BPM: f32 = 122.0;
const BITS: u16 = 16;

pub const SIN_WAVE: hound::WavSpec = hound::WavSpec {
    channels: 1,
    sample_rate: SAMPLE_RATE as u32,
    bits_per_sample: BITS,
    sample_format: hound::SampleFormat::Int,
};

pub const SAWTOOTH_WAVE: hound::WavSpec = hound::WavSpec {
    channels: 1,
    sample_rate: SAMPLE_RATE as u32,
    bits_per_sample: BITS * 2,
    sample_format: hound::SampleFormat::Float,
};
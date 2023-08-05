use std::{f32::consts::PI, io::BufWriter};
use std::fs::File;

use super::constant::{SAMPLE_RATE, SIN_WAVE, SAWTOOTH_WAVE};
use super::parse_input::{parse, SoundComp};
use hound::WavWriter;

fn write_sin_wave(fw: &mut WavWriter<BufWriter<File>>, tone: f32, length: u32) {
    for i in 0..length {
        let v = ((i as f32 / SAMPLE_RATE) * tone * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

fn write_sawtooth_wave(fw: &mut WavWriter<BufWriter<File>>, tone: f32, length: u32, gain: f32) {
    for i in 0..length {
        let v = if tone == 0.0 { 0.0 } else { (i as f32 / (SAMPLE_RATE / tone) % 1.0) * 2.0 - 1.0 };
        match fw.write_sample(v * gain) {
            Err(e) => println!("{}", e),
            _ => {}
        }
    }
}

pub fn create_wave(input: Vec<String>, filename: String, gain: f32) {
    let mut len = 0;

    let mut fw = WavWriter::create(filename, SAWTOOTH_WAVE).expect("File open error");
    let data = parse(input);

    for d in data {
        match d {
            SoundComp::Length(l) => { len = l; },
            SoundComp::Tone(t) => { write_sawtooth_wave(&mut fw, t, len, gain); }
        }
    }
}
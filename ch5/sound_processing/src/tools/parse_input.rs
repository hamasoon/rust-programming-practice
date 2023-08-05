use super::constant::{BPM, SAMPLE_RATE};

pub enum SoundComp {
    Length(u32),
    Tone(f32)
}

fn select_tone(input: String) -> Result<f32, String> {
    let chars = input.bytes().collect::<Vec<u8>>();
    let mut octave: i32 = 3;

    for (i, c) in chars.iter().enumerate() {
        if i == 0 { continue; }

        octave += match *c as char {
            '+' => 1,
            '-' => -1, 
            _ => 0,
        }
    }

    if octave < 0 || octave > 7 { return Result::Err("Wrong ocatave".to_string()); }

    let tone: f32 = match chars.get(0) {
        Some(b'C') | Some(b'c') => {
            match chars.get(1) {
                Some(b'#') => 2.0,
                _=> 1.0
            }
        }
        Some(b'D') | Some(b'd') => {
            match chars.get(1) {
                Some(b'#') => 4.0,
                _=> 3.0
            }
        },
        Some(b'E') | Some(b'e') => 5.0,
        Some(b'F') | Some(b'f') => {
            match chars.get(1) {
                Some(b'#') => 7.0,
                _=> 6.0
            }
        },
        Some(b'G') | Some(b'g') => {
            match chars.get(1) {
                Some(b'#') => 9.0,
                _=> 8.0
            }
        },
        Some(b'A') | Some(b'a') => {
            match chars.get(1) {
                Some(b'#') => 11.0,
                _ => 10.0
            }
        },
        Some(b'B') | Some(b'b') => 12.0,
        Some(b'R') | Some(b'r') => return Result::Ok(0.0),
        _ => {
            return Result::Err("Wrong tone type ".to_string());
        }
    };

    Result::Ok(f32::powi(2.0, octave) * 55.0 * f32::powf(2.0, (tone - 10.0)/12.0))
}

pub fn parse(input: Vec<String>) -> Vec<SoundComp>{
    let mut ret: Vec<SoundComp> = Vec::new();
    let calc_len = |l: u32| -> u32 {((30.0/BPM * SAMPLE_RATE) as u32) * l};

    for word in input {
        if word == "l8" {
            ret.push(SoundComp::Length(calc_len(1)));
        }
        else if word == "l4" {
            ret.push(SoundComp::Length(calc_len(2)));
        }
        else if word == "l3" {
            ret.push(SoundComp::Length(calc_len(3)));
        }
        else if word == "l2" {
            ret.push(SoundComp::Length(calc_len(4)));
        }
        else if word == "l1" {
            ret.push(SoundComp::Length(calc_len(8)));
        }
        else {
            match select_tone(word) {
                Ok(v) => { ret.push(SoundComp::Tone(v)); },
                Err(_) => { println!("Wrong input type found."); }
            }
        }
    }

    ret
}
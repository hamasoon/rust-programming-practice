mod utils;
mod tools;

use utils::input::read_text;
use tools::wave::create_wave;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    create_wave(read_text(&args[1]), "sample.wav".to_string(), 0.25);
}

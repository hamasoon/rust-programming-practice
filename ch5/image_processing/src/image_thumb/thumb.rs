use image::{self, imageops, GenericImageView};

pub fn resize(size: u32, input_filename: String, output_filename: String) {
    let mut img = image::open(input_filename).expect("Image open err.");

    let dim = img.dimensions();
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};

    img.resize(w, w);
}
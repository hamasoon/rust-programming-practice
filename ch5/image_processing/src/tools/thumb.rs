use image::{self, imageops, GenericImageView};

pub fn resize(size: u32, input_filename: String, output_filename: String) {
    let mut img = match image::open(input_filename) {
        Ok(v) => v,
        Err(_) => { panic!("No such file/dir"); }
    };

    let dim = img.dimensions();
    let w = if dim.0 > dim.1 {dim.1} else {dim.0};

    let mut cutted_img = imageops::crop(&mut img, (dim.0 - w)/2, (dim.1 - w)/2, w, w).to_image();
    let final_image = imageops::resize(&mut cutted_img, size, size, imageops::Lanczos3);

    let _ = final_image.save(output_filename);
}
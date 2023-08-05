use image::{self, GenericImage, GenericImageView, Rgba};

pub fn reverse(input: String, output: String) {
    let mut img = match image::open(input) {
        Ok(v) => v,
        Err(_) => { panic!("No such file/dir"); }
    };

    let img_size = img.dimensions();

    for x in 0..img_size.0 {
        for y in 1..img_size.1 {
            let mut pixel = img.get_pixel(x, y);

            //pixel.0.iter().map(|c| 255 - c);

            let pixel = Rgba([
                255 - pixel[0],
                255 - pixel[1],
                255 - pixel[2],
                255 - pixel[3],
            ]);

            img.put_pixel(x, y, pixel);
        }
    }

    let _ = img.save(output);
}
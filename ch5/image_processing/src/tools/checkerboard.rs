use image::{Rgb, ImageBuffer};

pub fn create_board(width: u32, height: u32, tile_size: u32) {
    const WHITE: Rgb::<u8> = Rgb::<u8>([255, 255, 255]);
    const GREY: Rgb::<u8> = Rgb::<u8>([90, 90, 90]);

    let draw = |x: u32, y: u32| {
        match ((x / tile_size + y / tile_size)) % 2 {
            0 => WHITE,
            1 => GREY,
            _ => panic!("Error!")
        }
    };

    let img = ImageBuffer::from_fn(width, height, draw);

    let _ = match img.save("checkerboard.png") {
        Ok(_) => { println!("Complete Task!"); },
        Err(_) => { println!("Error oocur during saving image."); }
    };
}
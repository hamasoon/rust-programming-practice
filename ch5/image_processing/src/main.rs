mod image_checkerboard;
mod utils;
mod image_thumb;

use image_checkerboard::checkerboard;
use utils::input::get_u32_input;


fn main() {
    let width: u32 = get_u32_input("image width".to_string());
    let height: u32 = get_u32_input("image height".to_string());
    let tile_size: u32 = get_u32_input("image tile size".to_string());

    checkerboard::create_board(width, height, tile_size);
}

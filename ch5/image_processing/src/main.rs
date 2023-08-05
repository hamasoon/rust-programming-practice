mod utils;
mod tools;

use tools::checkerboard;
use tools::thumb;
use tools::reverse_color;
use utils::input::{get_u32_input, get_string_input};

fn checkerboard_img() {
    let width: u32 = get_u32_input("image width".to_string());
    let height: u32 = get_u32_input("image height".to_string());
    let tile_size: u32 = get_u32_input("image tile size".to_string());

    checkerboard::create_board(width, height, tile_size);
}

fn create_thumbnail() {
    let input: String = get_string_input("Original filename".to_string());
    let size: u32 = get_u32_input("Output file size".to_string());
    let output: String = get_string_input("Output filename".to_string());

    thumb::resize(size, input, output);
}

fn reverse() {
    let input: String = get_string_input("Original filename".to_string());
    let output: String = get_string_input("Output filename".to_string());

    reverse_color::reverse(input, output);
}

fn main() {
    

    reverse();
}

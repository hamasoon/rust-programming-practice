static CODE_A: i32 = 'A' as i32;
static CODE_Z: i32 = 'Z' as i32;

fn main() {
    println!("{}", encrypt("I LOVE RUST.", 3));
}

fn encrypt(text: &str, shift: i32) -> String {
    let is_az = |code: i32| -> bool {
        code > CODE_Z || code < CODE_A
    };

    let conv = |code: i32| -> i32 {
        (code - CODE_A + shift + 26) % 26 + CODE_A
    };

    let enc = |code: i32| -> char {
        if is_az(code) { code as u8 as char } else { conv(code) as u8 as char} 
    };

    text.chars().map(|c| enc(c as i32)).collect()
}
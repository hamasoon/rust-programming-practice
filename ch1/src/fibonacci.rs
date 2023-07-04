fn main() {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut c: u128 = 0;
    for _ in 1..51 {
        println!("{}", a);
        c = a + b;
        a = b;
        b = c;
    }
}
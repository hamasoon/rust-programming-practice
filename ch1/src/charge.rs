fn main() {
    let price: u32 = 3950;

    for i500 in 0..11 {
        if i500 * 500 > price {
            break;
        }
        for i100 in 0..4 {
            if i500 * 500 + i100 * 100 > price {
                break;
            }
            else if (price - (i500 * 500 + i100 * 100)) % 50 != 0 || (price - (i500 * 500 + i100 * 100)) / 50 > 10 {
                continue;
            }
            else {
                println!("500원 {}개, 100원 {}개, 50원 {}개", i500, i100, (price - (i500 * 500 + i100 * 100)) / 50);
            }
        }
    }
}
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 0..6 {
        println!("Roll the dice: {}", rng.gen_range(1..7));
    }
}

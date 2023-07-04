fn main() {
    for i in 1..1001 {
        if i % 3 == 0 {
            println!("{}: A", i);
        }
        else {
            let mut x = 1;

            while x < i {
                if (i / x) % 10 == 3 {
                    println!("{}: B", i);
                    break;
                }

                x *= 10;
            }
        }
    }
}
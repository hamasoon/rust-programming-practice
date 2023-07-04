fn main() {
    for i in 1..101 {
        print!("{}: ", i);

        if i % 15 == 0 {
            println!("fizzbuzz");
        }
        else if i % 3 == 0 {
            println!("fizz");
        }
        else if i % 5 == 0 {
            println!("buzz");
        }
        else {
            println!();
        }
    }
}
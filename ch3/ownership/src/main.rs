fn ownership_bia_type() {
    let i1: i32 = 30;
    let i2 = i1;

    let s1: String = String::from("Hello, world!");
    let s2: String = s1;

    // primitive type just copy
    println!("i1: {}", i1);
    println!("i2: {}", i2);

    println!("");

    // non primtive type work as move!
    // rather than, using clone()
    // println!("s1: {}", s1); -> error!
    println!("s1: {}", s2.clone());
    println!("s2: {}", s2);
}

fn give_and_return(s1: String) -> String {
    println!("s1: {}", s1);

    s1
}

fn main() {
    ownership_bia_type();

    let mut s1 = String::from("String Number.1");
    s1 = give_and_return(s1);
    println!("s1: {}", s1);
}

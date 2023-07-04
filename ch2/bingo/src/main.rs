use rand::seq::SliceRandom;
use std::env; 
//_rng.gen_range(0..4);

fn rand_array() {
    let mut _rng = rand::thread_rng();
 
    let mut nums = [0; 100];
    for i in 1..100 { nums[i] = i as i32 };
    nums.shuffle(&mut _rng);


    for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2{
                print!("{:>3}, ", '*');
            }
            else {
                print!("{:>3}, ", nums[i * 5 + j]);
            }
        }
        println!("");
    }
}

fn rand_vec() {
    //let mut vec: Vec<i32> = vec![1, 2, 3];
    let mut rng = rand::thread_rng(); 
    
    let mut nums: Vec<i32> = (1..100).collect();

    nums.shuffle(&mut rng);


    for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2{
                print!("{:>3}, ", '*');
            }
            else {
                print!("{:>3}, ", nums[i * 5 + j]);
            }
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2);

    match args[1].to_lowercase().as_str() {
        "array" | "arr" | "a" => { rand_array(); },
        "vector" | "vec" | "v" =>  { rand_vec(); },
        _ => { println!{"Wrong Input!"}; }
    }
}

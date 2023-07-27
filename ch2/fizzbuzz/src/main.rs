use std::env;
use std::fs::{File};
use std::io::{Write, BufWriter};

const FILE_DIR: &str = "output.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let iterate_count = match args[1].parse::<usize>() {
        Ok(v) => v,
        Err(_) => { println!("Wrong input type!"); return; }
    };

    let fptr = match File::create(FILE_DIR) {
        Ok(fp) => fp,
        Err(_) => { println!("File open error."); return; }
    };

    let mut writer = BufWriter::new(fptr);

    for i in 1..=iterate_count {
        let mut output: String = format!("{}\n", i);

        if i % 15 == 0 {
            output = String::from("FizzBuzz\n");
        }
        else if i % 3 == 0 {
            output = String::from("Fizz\n");
        }
        else if i % 5 == 0 {
            output = String::from("Buzz\n");
        }

        match writer.write(output.as_bytes()) {
            Ok(_) => {},
            Err(_) => { println!("Error in write {}th output", i); return;}
        }
    }
}

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const DIC_FILE_LOC: &str = "dict.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dict: HashMap<String, String> = HashMap::new();

    assert_ne!(args.len(), 1);

    let fptr = match File::open(DIC_FILE_LOC) {
        Ok(fp) => fp,
        Err(_) => { println!("File doesn't exist"); return; }
    };

    let reader = BufReader::new(fptr);

    for item in reader.lines() {
        let line = match item {
            Ok(v) => v,
            Err(_) => { println!("Error when open file."); return; }
        };

        let words: Vec<&str> = line.split('\t').collect();
        
        dict.entry(words[0].trim().to_string())
            .or_insert(words[1].trim().to_string());
    }

    for (i, word) in args.iter().enumerate() {
        if i == 0 { continue; }

        println!("==================================================================");
        println!("Searching: {}", word);
        println!("==================================================================");

        for key in dict.keys() {
            if key.find(word) != None {
                match dict.get(key) {
                    None => { println!("Not in dictionary"); },
                    Some(v) => { println!("|{:<20}|{:<30}", key, v); }
                }
            }
        }

        println!("==================================================================");
    }
}

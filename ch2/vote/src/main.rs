use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vote_result: HashMap<String, i32> = HashMap::new();

    for (i, s) in args.iter().enumerate() {
        if i == 0 { continue; }

        match vote_result.get(&(s.to_uppercase())) {
            None => { vote_result.insert(s.to_string(), 1); },
            Some(value) => { vote_result.insert(s.to_string(), value + 1); }
        }
    }

    for item in vote_result.iter() {
        println!("{} : {}", item.0, item.1);
    }
}

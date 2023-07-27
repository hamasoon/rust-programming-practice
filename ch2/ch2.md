# Rust-prgramming-practice Chapter 2

## Dice

#### Let use crate
```sh
cargo add rand@0.8.0
```
- add package to project
 
#### Use crate
```rust
use rand::Rng;
```

#### Get random number
```rust
let mut rng = rand::thread_rng(); // kind of random generator

let ni = rng.gen_range(1..7); // get random number between 1 and 6
```

## Bingo
- now, using `rand::seq::SliceRandom` crate

#### Arguments
```rust
let args: Vec<String> = env::args().collect();

assert_eq!(args.len(), 2);

match args[1].to_lowercase().as_str() {
    "array" | "arr" | "a" => { rand_array(); },
    "vector" | "vec" | "v" =>  { rand_vec(); },
    _ => { println!{"Wrong Input!"}; }
}
```

#### Shuffle
```rust
nums.shuffle(&mut _rng);
```

#### Choose
```rust
println!("{:?}", choices.choose(&mut rng).unwrap());
```

#### formatted print
```rust
print!("{:>2}, ", '*');
print!("{:^2}, ", '*');
print!("{:>2}, ", '*');
```

#### Vector
```rust
let mut vec: Vec<i32> = vec![1, 2, 3];  // vector initialize macro
let mut nums: Vec<i32> = (1..100).collect(); // collect make iterator to vector
```

## BMI

#### Get input
```rust
let mut input_string = String::new();
std::io::stdin().read_line(&mut input_string).expect("Input Error");
```

#### Parse string to float
```rust
input_string.trim().parse::<f64>().expect("Not digit");
```
- originial form
```rust
str.parse::<TYPE_NAME>().expect("Error message");
```

#### Result type
- Result<DATA_TYPE, ERR_TYPE>
- expect("Error message") print error message when error occured(ERR_TPYE is not none)
  - e.g. `.parse::<f64>().expect("Not digit");` &rarr; `ParseFloatError`
- unwrap() is same as expect() but ignore error message
- but best ways to handle result is using `match` expression
```rust
fn main() {
    match num {
        Ok(result) => println!("num is {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```
```rust
fn main() {
    let s = "365";

    let i: i32 = match s.parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
}
```
```rust
fn main() {
    let s = "365";

    let i: i32 = match s.parse().unwarp();
}
```

## Vote

#### HashMap
- using `std::collections::HashMap`
```rust
let mut map: HashMap<KEY_TYPE, VALUE_TYPE> = HashMap::new();
```
- note that `HashMap` is not ordered and must specify `KEY_TYPE` and `VALUE_TYPE`
- insert
  - overwrite : `map.insert(KEY, VALUE)`
  - insert new: `map.entry(KEY).or_insert(VALUE)`
    - entry check key is exist or not
    - if key is not exist, insert new key-value pair
    - if key is exist, do nothing(return mutable reference)
  - update : `map.entry(KEY).and_modify(|v| {*v += VALUE})`
    - and_modify() update value if key is exist
    - v is mutable reference of value
  - another update : `map.entry(KEY).or_insert(VALUE)`
    - note that or_insert() return **mutable reference**
    - by using this reference, we can update value
    ```rust
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    ```
- get
  - indexing : `map[&key]`
    - simple but not safe &rarr; panic when key is not exist
  - get() : `map.get(&key)`
    - does not panic but return `Option<&VALUE_TYPE>`
    - so, when key is not exist, return `None`
    - then return `Some(&VALUE_TYPE)` when key is exist
    - usually use `match` expression to handle `Option` type
    - example
    ```rust
    match vote_result.get(&(s.to_uppercase())) {
        None => { vote_result.insert(s.to_string(), 1); },
        Some(value) => { vote_result.insert(s.to_string(), value + 1); }
    }
    ```

## Dictioanry

#### File input
- using `std::fs::File`
- using `std::io::{BufRead, BufReader}`
- open file with `File::open(FILE_PATH)`
  - open return `Result<File, Error>`
```rust
let fptr = match File::open(DIC_FILE_LOC) {
    Ok(fp) => fp,
    Err(_) => { println!("File doesn't exist"); return; }
};
```
- create `BufReader` with `BufReader::new(FILE)`
- read single line with `read_line(&mut File)`
  - return `Result<&str, Error>`
  - if success, return read bytes
  - if fail, return `Err(Error)`
```rust
for item in reader.lines() {
    let line = match item {
        Ok(v) => v,
        Err(_) => { println!("Error when open file."); return; }
    };
}
```
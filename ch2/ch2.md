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
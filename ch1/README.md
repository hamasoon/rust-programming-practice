## Compile rust coed
```bash
$ rustc [RUSTFILENAME].rs
```

## Start Rust

### print
```rust
println!("Hello, world!");
```
- why ***!*** needed?
  - this mean macro, not normal function

#### print with Variable
```rust
println!("Hello, {}!", "world");
```
- normally called as variable binding

## Variable 
- in rust, variable is immutable by default
- ex
```rust
let a = 1;
a = a + 1 // error
```
- if you want to mutable variable, use ***mut***
```rust
let mut a = 1;
a = a + 1 // ok
```

use std::io::Write;

pub fn get_u32_input(input_prompt: String) -> u32 {
    print!("{}: ", input_prompt);
    let _ = std::io::stdout().flush();

    let mut input: String = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Input err.");

    match input.trim().parse::<u32>() {
        Ok(v) => v,
        Err(_) => panic!("Input type err.")
    }
}

pub fn get_string_input(input_prompt: String) -> String {
    print!("{}: ", input_prompt);
    let _ = std::io::stdout().flush();

    let mut input: String = String::new();
    
    std::io::stdin().read_line(&mut input).expect("Input err.");

    input.trim().to_string()
}
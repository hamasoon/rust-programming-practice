fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    
    let mut input_string = String::new();

    std::io::stdin().read_line(&mut input_string).expect("Input Error");

    return input_string.trim().parse::<f64>().expect("Not digit");
}

fn main() {
    let height_cm: f64 = input("Height(cm): ");
    let weight: f64 = input("Weight(kg): ");

    let height = height_cm / 100.0;

    let bmi = weight / height.powi(2);

    if bmi < 18.5 {
        println!("저체중");
    }
    else if bmi < 23.0 {
        println!("정상");
    }
    else if bmi < 25.0 {
        println!("비만전 단계");
    }
    else if bmi < 30.0 {
        println!("1단계 비만");
    }
    else if bmi < 35.0 {
        println!("2단계 비만");
    }
    else {
        println!("3단계 비만(고도 비만)");
    }
}

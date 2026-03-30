fn analyze_number(x: i32) -> String {
    let result: String;
    match x {
        ..0 => result = "Negative".to_string(),
        0 => result = "Zero".to_string(),
        1..=5 => result = "Small".to_string(),
        6..=10 => result = "Medium".to_string(),
        _ => result = "Large".to_string()
    }
    result
}

fn main() {
    let x = -123;
    let result = analyze_number(x);

    println!("result: {result}");
}
fn main() {
    let input = "42";
    let input: i32 = input.parse().expect("Not a number");

    match input % 2 {
        0 => println!("Even"),
        _ => println!("Odd")
    };
}
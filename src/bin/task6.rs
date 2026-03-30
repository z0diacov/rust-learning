fn main() {
    let celsius = 10.2;
    let f = celsius * 1.8 + 32.0;

    let status = if f > 100.0 {
        "Hot"
    } else if f < 32.0 {
        "Ice"
    } else {
        "Normal"
    };

    println!("{status}");
}
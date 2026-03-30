fn print_string(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("Hello");
    print_string(s);
    //println!("{s}"); can not
}
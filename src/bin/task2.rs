fn add_exclamation_non_mut(s: String) -> String {
    let mut s = s;
    s.push_str("!");
    s
}

fn main() {
    let mut string = String::from("Hello");
    string.push_str(" world");

    println!("{string}");
    
    let string = add_exclamation_non_mut(string);

    println!("{string}");
}

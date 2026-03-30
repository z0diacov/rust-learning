fn add_exclamation(s: &mut String) {
    s.push_str(" !");
}

fn main() {
    let mut string = String::from("Hello");
    string.push_str(" world");

    println!("{string}");
    add_exclamation(&mut string);
    println!("{string}");
}

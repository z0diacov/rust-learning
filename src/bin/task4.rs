fn print_len(s: &String) {
    println!("{}", s.len());
}

fn add_world(s: &mut String) {
    s.push_str(" world");
}

fn consume(s: String) {
    println!("{s}");
}

fn main() {
    let mut s = String::from("Hello");

    print_len(&s);
    add_world(&mut s);
    consume(s);
    
    // println!("{s}");
}
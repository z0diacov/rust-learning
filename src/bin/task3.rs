fn append_if_long(s: &mut String) {
    if (s.len()) > 5 {
        s.push_str("!!!");
    } else {
        s.push_str("?");
    }
}

fn main() {
    let mut s1 = String::from("Hello");
    let mut s2 = String::from("Hello world");

    append_if_long(&mut s1);
    append_if_long(&mut s2);

    println!("{s1}"); // Hello?
    println!("{s2}"); // Hello world!!!
}
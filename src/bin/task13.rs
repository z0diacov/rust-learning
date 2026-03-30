fn get_user_by_id<'a>(id: usize, users: &'a[&'a str]) -> Option<&'a str> {
    users.get(id).copied()
}

fn main() {
    let users = ["Bogdan", "Anton", "Dmitry"];
    let id_to_find = 1;

    match get_user_by_id(id_to_find, &users) {
        Some(name) => println!("Found user: {name}"),
        None => println!("User with id {id_to_find} does no exist"),
    }
}
struct User<'a> {
    name: &'a str,
    profile: String,
}


fn create_user<'a>(name: &'a str) -> User<'a> {
    let profile = String::from("Default");
    User { name, profile }
}

fn main() {
    let user = create_user("Alice");
    println!("Name: {}, Profile: {}", user.name, user.profile);
}

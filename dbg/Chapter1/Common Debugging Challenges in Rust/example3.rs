struct User<'a> {
    name: &'a str,
    profile: &'a str,
}


fn create_user<'a>(name: &'a str) -> User<'a> {
    let local_profile = String::from("Default");
    let profile = &local_profile;  // Borrowing local_profile, which will go out of scope
    User { name, profile }
}

fn main() {
    let user = create_user("Alice");
    println!("Name: {}, Profile: {}", user.name, user.profile);
}

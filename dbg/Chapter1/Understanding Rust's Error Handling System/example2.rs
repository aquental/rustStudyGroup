fn find_user(id: i32) -> Option<String> {
    if id > 0 {
        Some(String::from("User found"))
    } else {
        None
    }
}
fn main() {
    let user = find_user(1);
   
    match user {
        Some(name) => println!("Success: {}", name),
        None => println!("Error: User not found"),
    }
}

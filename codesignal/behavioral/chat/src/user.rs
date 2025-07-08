use crate::chat_room::Observer;

// TODO: Define the User struct implementing the Observer trait
// - Define a constructor that initializes the user's name
// - Implement the update method taking a message parameter to display a received message
#[derive(PartialEq, Eq, Hash)]
pub struct User {
    name: String,
}
impl User {
    pub fn new(name: String) -> User {
        User { name }
    }
}
impl Observer for User {
    fn update(&self, message: String) {
        println!("{}: {}", self.name, message);
    }
}

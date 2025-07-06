// TODO: Define a trait named Follower
// - Implement a method notify that takes a string parameter named update

// TODO: Implement a ConcreteFollower struct that implements the Follower trait
// - Override the notify method in the ConcreteFollower struct to display the message "<name> received update: <update>"
pub trait Follower {
    fn notify(&self, update: &str);
}

pub struct ConcreteFollower {
    name: String,
}

impl ConcreteFollower {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Follower for ConcreteFollower {
    fn notify(&self, update: &str) {
        println!("{} received update: {}", self.name, update);
    }
}

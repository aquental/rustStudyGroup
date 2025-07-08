use std::collections::HashSet;
use crate::user::User;

// TODO: Define the Observer trait
// - Declare a method update which takes a message parameter
pub trait Observer {
    fn update(&self, message: String);
}

// TODO: Define the ChatRoom struct
// - Define a private member to store chat members
// - Implement a method add_observer to add an observer
// - Implement a method notify_observers to broadcast a message to all observers
// - Implement a method show_message to display the message
pub struct ChatRoom {
    observers: HashSet<User>,
}
impl ChatRoom {
    pub fn new() -> ChatRoom {
        ChatRoom {
            observers: HashSet::new(),
        }
    }
    pub fn add_observer(&mut self, observer: User) {
        self.observers.insert(observer);
    }
    pub fn notify_observers(&self, message: String) {
        for observer in &self.observers {
            observer.update(message.clone());
        }
    }
    pub fn show_message(&self, message: String) {
        println!("{}", message);
    }
}

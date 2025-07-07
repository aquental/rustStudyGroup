use std::collections::HashMap;

// TODO: Define the Observer trait with an update method
// - The update method should accept a string argument as a post
pub trait Observer {
    fn update(&self, post: &str);
}

// TODO: Implement the Influencer struct with methods to manage observers
// - Use a HashMap to store observers with a unique id
// - Implement the add_observer method to add an observer to the list
// - Implement the remove_observer method to remove an observer using the id
// - Implement the post_update method to notify all observers about the new post
pub struct Influencer {
    observers: HashMap<u32, Box<dyn Observer>>,
    next_id: u32,
}
impl Influencer {
    pub fn new() -> Influencer {
        Influencer {
            observers: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.insert(self.next_id, observer);
        self.next_id += 1;
    }
    pub fn remove_observer(&mut self, id: u32) {
        self.observers.remove(&id);
    }
    pub fn post_update(&self, post: &str) {
        for (_, observer) in &self.observers {
            observer.update(post);
        }
    }
}

// TODO: Implement the UserFollower struct that inherits from Observer
// - The constructor should initialize the name of the user follower
// - The update method should print "<name> (User) saw post: <post>"
pub struct UserFollower {
    name: String,
}
impl UserFollower {
    pub fn new(name: String) -> UserFollower {
        UserFollower { name }
    }
}
impl Observer for UserFollower {
    fn update(&self, post: &str) {
        println!("{} (User) saw post: {}", self.name, post);
    }
}

// TODO: Implement the MediaFollower struct that inherits from Observer
// - The constructor should initialize the outlet name of the media follower
// - The update method should print "<outletName> (Media) reported: <post>"
pub struct MediaFollower {
    outlet_name: String,
}
impl MediaFollower {
    pub fn new(outlet_name: String) -> MediaFollower {
        MediaFollower { outlet_name }
    }
}
impl Observer for MediaFollower {
    fn update(&self, post: &str) {
        println!("{} (Media) reported: {}", self.outlet_name, post);
    }
}

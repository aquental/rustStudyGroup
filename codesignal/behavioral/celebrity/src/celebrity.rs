use crate::follower::Follower;
use std::collections::HashMap;

// TODO: Implement a Celebrity struct that manages followers and can notify them
// - Define a HashMap to keep track of followers in the Celebrity struct
// - Implement the add_follower method in the Celebrity struct to add a follower to the HashMap
// - Implement the remove_follower method in the Celebrity struct to remove a follower from the HashMap
// - Implement the post_update method in the Celebrity struct to notify all followers with the update
pub struct Celebrity {
    followers: HashMap<String, Box<dyn Follower>>,
}

impl Celebrity {
    pub fn new() -> Self {
        Celebrity {
            followers: HashMap::new(),
        }
    }
    pub fn add_follower(&mut self, follower_id: String, follower: Box<dyn Follower>) {
        self.followers.insert(follower_id, follower);
    }
    pub fn remove_follower(&mut self, follower_id: &str) {
        self.followers.remove(follower_id);
    }
    pub fn post_update(&self, update: &str) {
        for follower in self.followers.values() {
            follower.notify(update);
        }
    }
}

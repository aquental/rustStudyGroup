use crate::listener::Listener;
use std::collections::HashMap;

pub struct PodcastChannel {
    listeners: HashMap<String, Box<dyn Listener>>,
}

impl PodcastChannel {
    pub fn new() -> Self {
        PodcastChannel {
            listeners: HashMap::new(),
        }
    }

    // TODO: Implement the add_listener method to add a listener to the HashMap

    // TODO: Implement the remove_listener method to remove a listener from the HashMap

    // TODO: Implement the release_episode method to notify all current listeners about the new episode
}

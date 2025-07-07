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
    pub fn add_listener(&mut self, name: &str, listener: Box<dyn Listener>) {
        self.listeners.insert(name.to_string(), listener);
    }

    // TODO: Implement the remove_listener method to remove a listener from the HashMap
    pub fn remove_listener(&mut self, name: &str) {
        self.listeners.remove(name);
    }

    // TODO: Implement the release_episode method to notify all current listeners about the new episode
    pub fn release_episode(&self, episode: &str) {
        for listener in self.listeners.values() {
            listener.update(episode);
        }
    }
}

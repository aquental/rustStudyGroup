use std::collections::HashMap;

pub trait Observer {
    fn update(&self, article: &str);
}

pub struct BlogPublisher {
    subscribers: HashMap<String, Box<dyn Observer>>,
}

impl BlogPublisher {
    pub fn new() -> Self {
        BlogPublisher {
            subscribers: HashMap::new(),
        }
    }

    pub fn add_subscriber(&mut self, id: String, subscriber: Box<dyn Observer>) {
        self.subscribers.insert(id, subscriber);
    }

    pub fn remove_subscriber(&mut self, id: &str) {
        self.subscribers.remove(id);
    }

    pub fn publish(&self, article: &str) {
        for subscriber in self.subscribers.values() {
            subscriber.update(article);
        }
    }
}

use crate::subscriber::Subscriber;
use std::collections::HashMap;

pub struct BookStore {
    subscribers: HashMap<String, Box<dyn Subscriber>>,
}

impl BookStore {
    pub fn new() -> Self {
        BookStore {
            subscribers: HashMap::new(),
        }
    }

    // TODO: Add the add_subscriber method
    // - This method receives a subscriber as a parameter
    // - Simply add the given subscriber to the subscribers map with its id
    pub fn add_subscriber(&mut self, subscriber: Box<dyn Subscriber>) {
        self.subscribers.insert(subscriber.get_id(), subscriber);
    }

    // TODO: Add the remove_subscriber method
    // - This method receives a subscriber id as a parameter
    // - Remove the subscriber from the subscribers map
    pub fn remove_subscriber(&mut self, subscriber_id: &str) {
        self.subscribers.remove(subscriber_id);
    }

    // TODO: Add the notify_new_arrival method
    // - This method receives the new book's title as a parameter
    // - Loop through the subscribers map
    // - Call each subscriber's receive_notification method with the new book's title
    pub fn notify_new_arrival(&self, new_book: &str) {
        for subscriber in self.subscribers.values() {
            subscriber.receive_notification(new_book);
        }
    }
}

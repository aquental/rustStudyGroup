pub trait Subscriber {
    fn receive_notification(&self, book: &str);
    fn get_id(&self) -> String;
}

pub struct ConcreteSubscriber {
    name: String,
}

impl ConcreteSubscriber {
    pub fn new(name: &str) -> Self {
        ConcreteSubscriber {
            name: name.to_string(),
        }
    }
}

impl Subscriber for ConcreteSubscriber {
    fn receive_notification(&self, book: &str) {
        println!("{} was notified about: {}", self.name, book);
    }

    // TODO: Implement the get_id method
    // - This method should return the subscriber's name as their unique ID
    fn get_id(&self) -> String {
        self.name.clone()
    }
}

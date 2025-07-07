pub trait Listener {
    fn update(&self, episode: &str);
}

pub struct ConcreteListener {
    name: String,
}

impl ConcreteListener {
    pub fn new(name: &str) -> Self {
        ConcreteListener {
            name: name.to_string(),
        }
    }
}

// TODO: Implement the Listener trait for ConcreteListener by overriding the update method to print "[name] is listening to: [episode]"

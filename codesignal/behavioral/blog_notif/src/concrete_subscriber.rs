use crate::blog_publisher::Observer;

// TODO: Implement the ConcreteSubscriber struct by implementing the Observer trait
// - Add name as a field
// - Implement the new constructor function that takes a name parameter
// - Implement the update method to print the format: "[name] received a new article: [article]"
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
impl Observer for ConcreteSubscriber {
    fn update(&self, article: &str) {
        println!("{} received a new article: {}", self.name, article);
    }
}

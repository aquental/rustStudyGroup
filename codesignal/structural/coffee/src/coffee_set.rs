use crate::coffee::Coffee;

// TODO: Define the CoffeeSet struct implementing Coffee
// - Create a private Vec of Box<dyn Coffee>
// - Implement a method add to add a Coffee object to the list
// - Implement get_description to concatenate descriptions of all Coffee objects in the list with a prefix "Coffee Set: "
// - Implement cost to sum up the costs of all Coffee objects in the list
pub struct CoffeeSet {
    coffees: Vec<Box<dyn Coffee>>,
}

impl CoffeeSet {
    pub fn new() -> CoffeeSet {
        CoffeeSet {
            coffees: Vec::new(),
        }
    }

    pub fn add(&mut self, coffee: Box<dyn Coffee>) {
        self.coffees.push(coffee);
    }
    pub fn get_description(&self) -> String {
        let mut description = String::from("Coffee Set: ");
        for coffee in &self.coffees {
            description.push_str(&coffee.get_description());
            description.push_str(", ");
        }
        description.pop();
        description.pop();
        description
    }
    pub fn cost(&self) -> f64 {
        let mut cost = 0.0;
        for coffee in &self.coffees {
            cost += coffee.cost();
        }
        cost
    }
}

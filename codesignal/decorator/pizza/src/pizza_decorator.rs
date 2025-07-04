use crate::pizza::Pizza;

// TODO: Define CheeseDecorator struct that takes a Box<dyn Pizza>
// - Implement description() to return pizza.description() + ", Cheese"
// - Implement cost() to increment pizza cost by 1.0 and return it
pub struct CheeseDecorator {
    pub pizza: Box<dyn Pizza>,
}
impl Pizza for CheeseDecorator {
    fn description(&self) -> String {
        self.pizza.description() + ", Cheese"
    }
    fn cost(&self) -> f64 {
        self.pizza.cost() + 1.0
    }
}

// TODO: Define PepperoniDecorator struct that takes a Box<dyn Pizza>
// - Implement description() to return pizza.description() + ", Pepperoni"
// - Implement cost() to increment pizza cost by 1.5 and return it
pub struct PepperoniDecorator {
    pub pizza: Box<dyn Pizza>,
}
impl Pizza for PepperoniDecorator {
    fn description(&self) -> String {
        self.pizza.description() + ", Pepperoni"
    }
    fn cost(&self) -> f64 {
        self.pizza.cost() + 1.5
    }
}

// TODO: Define OlivesDecorator struct that takes a Box<dyn Pizza>
// - Implement description() to return pizza.description() + ", Olives"
// - Implement cost() to increment pizza cost by 0.8 and return it
pub struct OlivesDecorator {
    pub pizza: Box<dyn Pizza>,
}
impl Pizza for OlivesDecorator {
    fn description(&self) -> String {
        self.pizza.description() + ", Olives"
    }
    fn cost(&self) -> f64 {
        self.pizza.cost() + 0.8
    }
}

// The Beverage trait defines the interface for all beverages
pub trait Beverage {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
}

// Concrete Component: The base beverage without any additions
pub struct Espresso;

impl Beverage for Espresso {
    fn description(&self) -> String {
        "Espresso".to_string()
    }

    fn cost(&self) -> f64 {
        1.99
    }
}

// TODO: Define Cinnamon struct that adds cinnamon flavor to coffee with additional cost of 0.3 dollars
// - Implement a constructor that accepts a Box<dyn Beverage> object
// - Override description to return the decorated coffee's description with ", Cinnamon" added
// - Override cost to return the decorated coffee's cost with additional 0.3 dollars
pub struct Cinnamon {
    pub coffee: Box<dyn Beverage>,
}
impl Beverage for Cinnamon {
    fn description(&self) -> String {
        format!("{}, Cinnamon", self.coffee.description())
    }
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.3
    }
}

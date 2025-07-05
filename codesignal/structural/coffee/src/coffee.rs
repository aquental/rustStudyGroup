// TODO: Define the Coffee trait
// - Create a method get_description that returns a String
// - Create a method cost that returns a f64
pub trait Coffee {
    fn get_description(&self) -> String;
    fn cost(&self) -> f64;
}

// TODO: Define the SimpleCoffee struct implementing Coffee
// - Implement get_description to return "Simple Coffee"
// - Implement cost to return 2.0
pub struct SimpleCoffee;

impl SimpleCoffee {
    pub fn new() -> SimpleCoffee {
        SimpleCoffee
    }
}

impl Coffee for SimpleCoffee {
    fn get_description(&self) -> String {
        String::from("Simple Coffee")
    }
    fn cost(&self) -> f64 {
        2.0
    }
}

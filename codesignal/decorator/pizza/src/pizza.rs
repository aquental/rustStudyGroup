// TODO: Define the Pizza trait
// - Include a method description() that returns "Plain Pizza"
// - Include an abstract method cost() that returns a f64
pub trait Pizza {
    fn description(&self) -> String;
    fn cost(&self) -> f64;
}

// TODO: Implement the PlainPizza struct that conforms to Pizza
// - Override the cost method to return 5.0
pub struct PlainPizza {
    pub description: String,
    pub cost: f64,
}
impl PlainPizza {
    pub fn new() -> PlainPizza {
        PlainPizza {
            description: "Plain Pizza".to_string(),
            cost: 5.0,
        }
    }
}
impl Pizza for PlainPizza {
    fn description(&self) -> String {
        self.description.clone()
    }
    fn cost(&self) -> f64 {
        self.cost
    }
}
